use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMetadata {
    pub last_check: Option<DateTime<Utc>>,
    pub last_update: Option<DateTime<Utc>>,
}

impl UpdateMetadata {
    /// Load metadata from cache, or create new if not exists
    pub fn load() -> Result<Self> {
        let metadata_path = Self::metadata_path()?;

        if metadata_path.exists() {
            let content = std::fs::read_to_string(&metadata_path)?;
            let metadata: Self = serde_json::from_str(&content)?;
            Ok(metadata)
        } else {
            let metadata = Self {
                last_check: None,
                last_update: None,
            };

            metadata.save()?;
            Ok(metadata)
        }
    }

    /// Save metadata to cache
    pub fn save(&self) -> Result<()> {
        let metadata_path = Self::metadata_path()?;

        // Ensure cache directory exists
        if let Some(parent) = metadata_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&metadata_path, content)?;
        Ok(())
    }

    /// Update last check timestamp
    pub fn update_last_check(&mut self) -> Result<()> {
        self.last_check = Some(Utc::now());
        self.save()
    }

    /// Update last update timestamp
    pub fn update_last_update(&mut self) -> Result<()> {
        self.last_update = Some(Utc::now());
        self.save()
    }

    fn metadata_path() -> Result<PathBuf> {
        use crate::config::Config;
        let cache_dir = Config::cache_dir()?;
        Ok(cache_dir.join("update_metadata.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_serialization() {
        let metadata = UpdateMetadata {
            last_check: Some(Utc::now()),
            last_update: None,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: UpdateMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(metadata.last_check.is_some(), deserialized.last_check.is_some());
    }
}
