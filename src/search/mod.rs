pub mod duckduckgo;
pub mod fetcher;

pub use duckduckgo::DuckDuckGo;
pub use fetcher::ContentFetcher;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub snippet: String,
    pub url: String,
    pub is_trusted: bool,
}

impl SearchResult {
    pub fn new(title: String, snippet: String, url: String, trusted_sources: &[String]) -> Self {
        let is_trusted = trusted_sources.iter().any(|source| url.contains(source));

        Self {
            title,
            snippet,
            url,
            is_trusted,
        }
    }
}
