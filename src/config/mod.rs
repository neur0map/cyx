pub mod manager;

pub use manager::ConfigManager;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: LLMProvider,
    pub api_keys: ApiKeys,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    Perplexity,
    Groq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeys {
    pub perplexity: Option<String>,
    pub groq: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: LLMProvider::Groq,
            api_keys: ApiKeys {
                perplexity: None,
                groq: None,
            },
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
}
