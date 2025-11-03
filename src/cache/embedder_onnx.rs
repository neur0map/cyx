use anyhow::{Context, Result};
use ndarray::{Array2, ArrayView2, Axis, CowArray};
use ort::{Environment, GraphOptimizationLevel, LoggingLevel, Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokenizers::Tokenizer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub dimensions: usize,
    pub size_mb: u64,
    pub description: String,
    pub hf_repo: String,
    pub onnx_file: String,
    pub tokenizer_file: String,
    pub files: Vec<ModelFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFile {
    pub name: String,
    pub url: String,
}

pub struct ONNXEmbedder {
    session: Session,
    tokenizer: Tokenizer,
    dimensions: usize,
    _environment: Arc<Environment>,
}

impl ONNXEmbedder {
    pub fn new(model_size: &str, models_dir: &Path) -> Result<Self> {
        let model_info = Self::get_model_info(model_size)?;
        let model_dir = models_dir.join(model_size);

        // Check if model files exist
        let onnx_path = model_dir.join("model.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        if !onnx_path.exists() || !tokenizer_path.exists() {
            anyhow::bail!(
                "Model files not found for '{}'. Run: cyx cache download-model {}",
                model_size,
                model_size
            );
        }

        // Initialize ONNX Runtime environment
        let environment = Arc::new(
            Environment::builder()
                .with_name("cyx-embedder")
                .with_log_level(LoggingLevel::Warning)
                .build()
                .context("Failed to initialize ONNX Runtime")?,
        );

        // Load ONNX session
        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .with_model_from_file(&onnx_path)
            .with_context(|| format!("Failed to load ONNX model from {}", onnx_path.display()))?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        println!(
            "[+] Loaded ONNX model: {} ({}D)",
            model_info.name, model_info.dimensions
        );

        Ok(Self {
            session,
            tokenizer,
            dimensions: model_info.dimensions,
            _environment: environment,
        })
    }

    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        // Tokenize input
        let encoding = self
            .tokenizer
            .encode(text, false)
            .map_err(|e| anyhow::anyhow!("Tokenization failed: {}", e))?;

        let input_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();

        // Convert to i64
        let input_ids: Vec<i64> = input_ids.iter().map(|&id| id as i64).collect();
        let attention_mask: Vec<i64> = attention_mask.iter().map(|&m| m as i64).collect();

        let seq_len = input_ids.len();

        // Create input tensors
        let input_ids_array = Array2::from_shape_vec((1, seq_len), input_ids)?;
        let attention_mask_array = Array2::from_shape_vec((1, seq_len), attention_mask.clone())?;

        // token_type_ids (all zeros for sentence transformers)
        let token_type_ids: Vec<i64> = vec![0; seq_len];
        let token_type_ids_array = Array2::from_shape_vec((1, seq_len), token_type_ids)?;

        // Run ONNX inference
        let input_ids_dyn = input_ids_array.into_dyn();
        let attention_mask_dyn = attention_mask_array.into_dyn();
        let token_type_ids_dyn = token_type_ids_array.into_dyn();

        let input_ids_cow = CowArray::from(&input_ids_dyn);
        let attention_mask_cow = CowArray::from(&attention_mask_dyn);
        let token_type_ids_cow = CowArray::from(&token_type_ids_dyn);

        let outputs = self.session.run(vec![
            ort::Value::from_array(self.session.allocator(), &input_ids_cow)?,
            ort::Value::from_array(self.session.allocator(), &attention_mask_cow)?,
            ort::Value::from_array(self.session.allocator(), &token_type_ids_cow)?,
        ])?;

        // Extract embeddings (last_hidden_state) - shape is [batch, seq_len, hidden_dim]
        let embeddings_tensor = outputs[0].try_extract::<f32>()?;
        let embeddings_view = embeddings_tensor.view();

        // embeddings_view shape: [1, seq_len, hidden_size]
        // We need to get the first batch and reshape to [seq_len, hidden_size]
        let shape = embeddings_view.shape();
        let _seq_len_out = shape[1];
        let _hidden_size = shape[2];

        // Extract the first batch slice: [seq_len, hidden_size]
        let batch_slice = embeddings_view.index_axis(Axis(0), 0);

        // Convert to proper 2D view
        let batch_2d = batch_slice.into_dimensionality::<ndarray::Ix2>()?;

        // Mean pooling over sequence dimension
        let pooled = self.mean_pooling(batch_2d, &attention_mask)?;

        // Normalize
        let normalized = Self::normalize_vector(&pooled);

        Ok(normalized)
    }

    fn mean_pooling(
        &self,
        embeddings: ArrayView2<f32>,
        attention_mask: &[i64],
    ) -> Result<Vec<f32>> {
        let seq_len = embeddings.shape()[0];
        let hidden_size = embeddings.shape()[1];

        let mut sum = vec![0.0f32; hidden_size];
        let mut count = 0.0f32;

        for i in 0..seq_len {
            if i < attention_mask.len() && attention_mask[i] == 1 {
                for j in 0..hidden_size {
                    sum[j] += embeddings[[i, j]];
                }
                count += 1.0;
            }
        }

        if count > 0.0 {
            for val in &mut sum {
                *val /= count;
            }
        }

        Ok(sum)
    }

    fn normalize_vector(vec: &[f32]) -> Vec<f32> {
        let norm: f32 = vec.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter().map(|&x| x / norm).collect()
        } else {
            vec.to_vec()
        }
    }

    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    pub fn get_model_info(model_size: &str) -> Result<ModelInfo> {
        let registry_path = Self::get_data_path("embedding_models.json")?;
        let content = std::fs::read_to_string(&registry_path).with_context(|| {
            format!("Failed to read model registry: {}", registry_path.display())
        })?;

        #[derive(Deserialize)]
        struct Registry {
            models: HashMap<String, ModelInfo>,
        }

        let registry: Registry =
            serde_json::from_str(&content).context("Failed to parse model registry")?;

        registry
            .models
            .get(model_size)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Unknown model size: {}", model_size))
    }

    pub async fn download_model(model_size: &str, models_dir: &Path) -> Result<()> {
        let model_info = Self::get_model_info(model_size)?;
        let model_dir = models_dir.join(model_size);

        // Create model directory
        std::fs::create_dir_all(&model_dir)
            .with_context(|| format!("Failed to create directory: {}", model_dir.display()))?;

        println!(
            "📦 Downloading {} model ({} MB)...",
            model_info.name, model_info.size_mb
        );

        // Download each file
        for file in &model_info.files {
            let file_path = model_dir.join(&file.name);

            if file_path.exists() {
                println!("  ✓ {} already exists", file.name);
                continue;
            }

            println!("  ⬇️  Downloading {}...", file.name);

            let response = reqwest::get(&file.url)
                .await
                .with_context(|| format!("Failed to download {}", file.url))?;

            if !response.status().is_success() {
                anyhow::bail!("Download failed with status: {}", response.status());
            }

            let bytes = response.bytes().await?;
            std::fs::write(&file_path, &bytes)
                .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

            println!(
                "  ✓ Downloaded {} ({:.1} MB)",
                file.name,
                bytes.len() as f64 / 1_048_576.0
            );
        }

        println!("[+] Model '{}' ready!", model_size);
        Ok(())
    }

    fn get_data_path(relative_path: &str) -> Result<PathBuf> {
        // Try current directory first
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let current_data = current_dir.join("data").join(relative_path);
        if current_data.exists() {
            return Ok(current_data);
        }

        // Try relative to executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let build_data = exe_dir.join("../../../data").join(relative_path);
                if build_data.exists() {
                    return Ok(build_data);
                }
            }
        }

        anyhow::bail!("Could not find data file: {}", relative_path)
    }
}
