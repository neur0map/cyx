use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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

#[derive(Debug, Deserialize)]
pub struct ModelRegistry {
    pub models: HashMap<String, ModelInfo>,
    pub default: String,
}

/// Lightweight text embedder using TF-IDF approach (no external models needed)
/// ONNX support prepared for future implementation
pub struct Embedder {
    dimensions: usize,
}

impl Embedder {
    /// Create a simple TF-IDF based embedder (no ONNX models required)
    pub fn new_simple(dimensions: usize) -> Self {
        Self { dimensions }
    }

    /// Generate embedding vector from text
    pub fn embed(&self, text: &str) -> Vec<f32> {
        let normalized_text = text.to_lowercase();
        let words: Vec<&str> = normalized_text.split_whitespace().collect();

        // Create a simple embedding using word frequency and character features
        let mut embedding = vec![0.0; self.dimensions];

        // Feature 1: Word count features
        for (i, word) in words.iter().enumerate() {
            let word_hash = Self::hash_string(word) % self.dimensions;
            embedding[word_hash] += 1.0 / (i + 1) as f32; // Position-weighted
        }

        // Feature 2: Character trigrams
        for word in &words {
            if word.len() >= 3 {
                for i in 0..word.len() - 2 {
                    let trigram = &word[i..i + 3];
                    let trigram_hash = Self::hash_string(trigram) % self.dimensions;
                    embedding[trigram_hash] += 0.5;
                }
            }
        }

        // Feature 3: Word length distribution
        let avg_word_len = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f32 / words.len() as f32
        } else {
            0.0
        };

        if self.dimensions > 10 {
            embedding[self.dimensions - 1] = avg_word_len / 10.0;
            embedding[self.dimensions - 2] = words.len() as f32 / 20.0;
        }

        // Normalize
        Self::normalize_vector(&mut embedding);

        embedding
    }

    fn hash_string(s: &str) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn normalize_vector(vec: &mut [f32]) {
        let norm: f32 = vec.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in vec.iter_mut() {
                *x /= norm;
            }
        }
    }

    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    pub fn get_model_info(model_size: &str) -> Result<ModelInfo> {
        let registry = Self::load_model_registry()?;
        registry
            .models
            .get(model_size)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Unknown model size: {}", model_size))
    }

    pub fn load_model_registry() -> Result<ModelRegistry> {
        // Embed the model registry data at compile time
        const MODELS_JSON: &str = include_str!("../../data/embedding_models.json");

        let registry: ModelRegistry =
            serde_json::from_str(MODELS_JSON).context("Failed to parse embedded model registry")?;

        Ok(registry)
    }

    pub fn list_available_models() -> Result<Vec<(String, ModelInfo)>> {
        let registry = Self::load_model_registry()?;
        let mut models: Vec<_> = registry.models.into_iter().collect();
        models.sort_by(|a, b| a.1.size_mb.cmp(&b.1.size_mb));
        Ok(models)
    }

    pub fn get_default_dimensions() -> usize {
        256 // Balanced dimension for simple embeddings
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
                println!("  ✓ {} already exists, skipping", file.name);
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

        println!(
            "[+] Model '{}' ready at: {}",
            model_size,
            model_dir.display()
        );
        Ok(())
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![-1.0, -2.0, -3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert!((sim - (-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_different_lengths() {
        let vec1 = vec![1.0, 2.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&vec1, &vec2);
        assert_eq!(sim, 0.0);
    }
}
