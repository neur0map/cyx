pub mod manager;

pub use manager::ConfigManager;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: LLMProvider,
    pub api_keys: ApiKeys,
    #[serde(default)]
    pub ollama: OllamaConfig,
    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    Perplexity,
    Groq,
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    pub perplexity: Option<String>,
    pub groq: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_context_window")]
    pub context_window: usize,
}

fn default_base_url() -> String {
    "http://localhost:11434".to_string()
}

fn default_model() -> String {
    "mistral:7b-instruct".to_string()
}

fn default_timeout() -> u64 {
    120
}

fn default_context_window() -> usize {
    8192
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: default_base_url(),
            model: default_model(),
            timeout_seconds: default_timeout(),
            context_window: default_context_window(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_enabled")]
    pub enabled: bool,
    #[serde(default = "default_ttl_days")]
    pub ttl_days: u32,
    #[serde(default = "default_embedding_model")]
    pub embedding_model: String,
    #[serde(default = "default_similarity_threshold")]
    pub similarity_threshold: f32,
}

fn default_embedding_model() -> String {
    "small".to_string()
}

fn default_similarity_threshold() -> f32 {
    0.80
}

fn default_cache_enabled() -> bool {
    true
}

fn default_ttl_days() -> u32 {
    30
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: default_cache_enabled(),
            ttl_days: default_ttl_days(),
            embedding_model: default_embedding_model(),
            similarity_threshold: default_similarity_threshold(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: LLMProvider::Groq,
            api_keys: ApiKeys {
                perplexity: None,
                groq: None,
            },
            ollama: OllamaConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Config {
    pub fn config_dir() -> anyhow::Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("", "", "cyx")
            .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;
        Ok(dirs.config_dir().to_path_buf())
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn cache_dir() -> anyhow::Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("", "", "cyx")
            .ok_or_else(|| anyhow::anyhow!("Failed to determine cache directory"))?;
        Ok(dirs.cache_dir().to_path_buf())
    }
}
