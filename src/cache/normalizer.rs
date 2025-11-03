use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationConfig {
    pub lowercase: bool,
    pub remove_punctuation: bool,
    pub expand_abbreviations: bool,
    pub trim_whitespace: bool,
    pub remove_stopwords: bool,
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        Self {
            lowercase: true,
            remove_punctuation: false,
            expand_abbreviations: true,
            trim_whitespace: true,
            remove_stopwords: true,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AbbreviationsData {
    abbreviations: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct StopwordsData {
    stopwords: Vec<String>,
}

pub struct QueryNormalizer {
    config: NormalizationConfig,
    abbreviations: HashMap<String, String>,
    stopwords: HashSet<String>,
}

impl QueryNormalizer {
    pub fn new(config: NormalizationConfig) -> Result<Self> {
        let abbreviations = Self::load_abbreviations()?;
        let stopwords = Self::load_stopwords()?;

        Ok(Self {
            config,
            abbreviations,
            stopwords,
        })
    }

    pub fn with_defaults() -> Result<Self> {
        Self::new(NormalizationConfig::default())
    }

    fn load_abbreviations() -> Result<HashMap<String, String>> {
        let path = Self::get_data_path("normalization/abbreviations.json")?;
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read abbreviations file: {}", path.display()))?;
        
        let data: AbbreviationsData = serde_json::from_str(&content)
            .context("Failed to parse abbreviations JSON")?;

        Ok(data.abbreviations)
    }

    fn load_stopwords() -> Result<HashSet<String>> {
        let path = Self::get_data_path("normalization/stopwords.json")?;
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read stopwords file: {}", path.display()))?;
        
        let data: StopwordsData = serde_json::from_str(&content)
            .context("Failed to parse stopwords JSON")?;

        Ok(data.stopwords.into_iter().collect())
    }

    fn get_data_path(relative_path: &str) -> Result<std::path::PathBuf> {
        // Try relative to executable first
        let exe_path = std::env::current_exe()
            .context("Failed to get executable path")?;
        
        if let Some(exe_dir) = exe_path.parent() {
            // Check in release/debug build directories
            let build_data = exe_dir.join("../../../data").join(relative_path);
            if build_data.exists() {
                return Ok(build_data);
            }
        }

        // Try current directory
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
        let current_data = current_dir.join("data").join(relative_path);
        if current_data.exists() {
            return Ok(current_data);
        }

        // Try from project root (for tests)
        let project_root = current_dir.join("../../..").join("data").join(relative_path);
        if project_root.exists() {
            return Ok(project_root);
        }

        anyhow::bail!("Could not find data file: {}", relative_path)
    }

    pub fn normalize(&self, query: &str) -> Result<String> {
        let mut normalized = query.to_string();

        // Step 1: Trim whitespace
        if self.config.trim_whitespace {
            normalized = normalized.trim().to_string();
        }

        // Step 2: Convert to lowercase
        if self.config.lowercase {
            normalized = normalized.to_lowercase();
        }

        // Step 3: Expand abbreviations
        if self.config.expand_abbreviations {
            normalized = self.expand_abbreviations(&normalized);
        }

        // Step 4: Remove excessive punctuation (but keep hyphens in words)
        if self.config.remove_punctuation {
            normalized = self.clean_punctuation(&normalized);
        }

        // Step 5: Remove stopwords
        if self.config.remove_stopwords {
            normalized = self.remove_stopwords(&normalized);
        }

        // Step 6: Normalize whitespace (collapse multiple spaces)
        normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");

        Ok(normalized)
    }

    fn expand_abbreviations(&self, text: &str) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut expanded = Vec::new();

        for word in words {
            // Remove trailing punctuation for matching
            let clean_word = word.trim_end_matches(|c: char| !c.is_alphanumeric());
            
            if let Some(expansion) = self.abbreviations.get(clean_word) {
                expanded.push(expansion.as_str());
            } else {
                expanded.push(word);
            }
        }

        expanded.join(" ")
    }

    fn clean_punctuation(&self, text: &str) -> String {
        let mut result = String::new();
        let mut last_was_space = false;

        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == '/' {
                result.push(ch);
                last_was_space = false;
            } else if ch.is_whitespace() {
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            } else {
                // Replace other punctuation with space
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            }
        }

        result.trim().to_string()
    }

    fn remove_stopwords(&self, text: &str) -> String {
        text.split_whitespace()
            .filter(|word| !self.stopwords.contains(*word))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn compute_hash(&self, normalized_query: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        normalized_query.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_normalizer() -> QueryNormalizer {
        let mut abbreviations = HashMap::new();
        abbreviations.insert("nmap".to_string(), "network mapper nmap".to_string());
        abbreviations.insert("syn".to_string(), "stealth synchronize".to_string());
        abbreviations.insert("sqli".to_string(), "sql injection".to_string());
        abbreviations.insert("privesc".to_string(), "privilege escalation".to_string());

        let mut stopwords = HashSet::new();
        stopwords.insert("show".to_string());
        stopwords.insert("me".to_string());
        stopwords.insert("how".to_string());
        stopwords.insert("to".to_string());
        stopwords.insert("the".to_string());
        stopwords.insert("a".to_string());
        stopwords.insert("an".to_string());
        stopwords.insert("do".to_string());
        stopwords.insert("for".to_string());

        QueryNormalizer {
            config: NormalizationConfig::default(),
            abbreviations,
            stopwords,
        }
    }

    #[test]
    fn test_lowercase() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("Show Me NMAP Scan").unwrap();
        assert_eq!(result, "network mapper nmap scan");
    }

    #[test]
    fn test_abbreviation_expansion() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("nmap syn scan").unwrap();
        assert_eq!(result, "network mapper nmap stealth synchronize scan");
    }

    #[test]
    fn test_stopword_removal() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("show me how to use nmap").unwrap();
        assert_eq!(result, "use network mapper nmap");
    }

    #[test]
    fn test_punctuation_removal() {
        let mut config = NormalizationConfig::default();
        config.remove_punctuation = true;
        config.remove_stopwords = false;
        
        let normalizer = QueryNormalizer {
            config,
            abbreviations: HashMap::new(),
            stopwords: HashSet::new(),
        };

        let result = normalizer.normalize("nmap --syn scan!!!").unwrap();
        assert_eq!(result, "nmap --syn scan");
    }

    #[test]
    fn test_whitespace_normalization() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("nmap    scan      target").unwrap();
        assert_eq!(result, "network mapper nmap scan target");
    }

    #[test]
    fn test_complex_query() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("Show me how to do an nmap SYN scan for sqli testing").unwrap();
        assert_eq!(result, "network mapper nmap stealth synchronize scan sql injection testing");
    }

    #[test]
    fn test_hash_consistency() {
        let normalizer = create_test_normalizer();
        let query = "nmap scan";
        let normalized = normalizer.normalize(query).unwrap();
        let hash1 = normalizer.compute_hash(&normalized);
        let hash2 = normalizer.compute_hash(&normalized);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_similar_queries_same_hash() {
        let normalizer = create_test_normalizer();
        
        let query1 = "show me nmap scan";
        let query2 = "Show Me NMAP Scan";
        let query3 = "nmap scan";

        let norm1 = normalizer.normalize(query1).unwrap();
        let norm2 = normalizer.normalize(query2).unwrap();
        let norm3 = normalizer.normalize(query3).unwrap();

        let hash1 = normalizer.compute_hash(&norm1);
        let hash2 = normalizer.compute_hash(&norm2);
        let hash3 = normalizer.compute_hash(&norm3);

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn test_different_queries_different_hash() {
        let normalizer = create_test_normalizer();
        
        let query1 = "nmap scan";
        let query2 = "nmap privesc";

        let norm1 = normalizer.normalize(query1).unwrap();
        let norm2 = normalizer.normalize(query2).unwrap();

        let hash1 = normalizer.compute_hash(&norm1);
        let hash2 = normalizer.compute_hash(&norm2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_empty_query() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_only_stopwords() {
        let normalizer = create_test_normalizer();
        let result = normalizer.normalize("show me how to the a").unwrap();
        assert_eq!(result, "");
    }
}
