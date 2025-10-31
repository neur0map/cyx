use super::SearchResult;
use anyhow::{Context, Result};
use regex::Regex;

pub struct DuckDuckGo {
    client: reqwest::blocking::Client,
}

impl DuckDuckGo {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub fn search(&self, query: &str, max_results: usize, trusted_sources: &[String]) -> Result<Vec<SearchResult>> {
        let url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(query));

        let response = self
            .client
            .get(&url)
            .send()
            .context("Failed to send search request")?;

        if !response.status().is_success() {
            anyhow::bail!("Search request failed with status: {}", response.status());
        }

        let html = response.text().context("Failed to read response body")?;
        let results = self.parse_results(&html, max_results, trusted_sources)?;

        Ok(results)
    }

    fn parse_results(&self, html: &str, max_results: usize, trusted_sources: &[String]) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        // Parse DuckDuckGo HTML results
        // Results are in <div class="result"> elements
        let result_regex = Regex::new(r#"<div class="result[^"]*">.*?</div>\s*</div>"#).unwrap();
        let title_regex = Regex::new(r#"<a[^>]*class="result__a"[^>]*>([^<]+)</a>"#).unwrap();
        let url_regex = Regex::new(r#"<a[^>]*class="result__a"[^>]*href="([^"]+)""#).unwrap();
        let snippet_regex = Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>([^<]+)</a>"#).unwrap();

        for result_match in result_regex.find_iter(html) {
            if results.len() >= max_results {
                break;
            }

            let result_html = result_match.as_str();

            let title = title_regex
                .captures(result_html)
                .and_then(|c| c.get(1))
                .map(|m| html_escape::decode_html_entities(m.as_str()).to_string())
                .unwrap_or_default();

            let url = url_regex
                .captures(result_html)
                .and_then(|c| c.get(1))
                .map(|m| {
                    let url = m.as_str();
                    // DuckDuckGo uses redirect URLs, extract the actual URL
                    if url.starts_with("//duckduckgo.com/l/?") {
                        Self::extract_redirect_url(url).unwrap_or(url.to_string())
                    } else {
                        url.to_string()
                    }
                })
                .unwrap_or_default();

            let snippet = snippet_regex
                .captures(result_html)
                .and_then(|c| c.get(1))
                .map(|m| html_escape::decode_html_entities(m.as_str()).to_string())
                .unwrap_or_default();

            if !title.is_empty() && !url.is_empty() {
                results.push(SearchResult::new(title, snippet, url, trusted_sources));
            }
        }

        Ok(results)
    }

    fn extract_redirect_url(redirect: &str) -> Option<String> {
        // Extract uddg parameter from DuckDuckGo redirect URL
        let re = Regex::new(r"uddg=([^&]+)").unwrap();
        re.captures(redirect)
            .and_then(|c| c.get(1))
            .map(|m| urlencoding::decode(m.as_str()).ok())
            .flatten()
            .map(|s| s.to_string())
    }
}

impl Default for DuckDuckGo {
    fn default() -> Self {
        Self::new().expect("Failed to create DuckDuckGo client")
    }
}
