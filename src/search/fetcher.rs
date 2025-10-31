use anyhow::{Context, Result};
use colored::Colorize;

pub struct ContentFetcher {
    client: reqwest::blocking::Client,
}

impl ContentFetcher {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    /// Fetch HTML content from URL and convert to markdown
    pub fn fetch_as_markdown(&self, url: &str) -> Result<String> {
        println!("{} {}", "[~] Fetching:".dimmed(), url.cyan());

        let response = self
            .client
            .get(url)
            .send()
            .context("Failed to fetch URL")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error {}: {}", response.status(), url);
        }

        let html = response.text().context("Failed to read response body")?;
        let markdown = self.html_to_markdown(&html);

        Ok(markdown)
    }

    /// Convert HTML to clean markdown using html2text
    fn html_to_markdown(&self, html: &str) -> String {
        // Configure html2text for better markdown output
        let width = 120; // Line width for wrapping
        let markdown = html2text::from_read(html.as_bytes(), width);

        // Clean up the markdown
        self.clean_markdown(&markdown)
    }

    /// Clean up markdown output and sanitize for security
    fn clean_markdown(&self, markdown: &str) -> String {
        let mut cleaned = markdown.to_string();

        // SECURITY: Remove potential prompt injection patterns
        cleaned = self.sanitize_prompt_injection(&cleaned);

        // Remove excessive newlines (more than 2 consecutive)
        let re = regex::Regex::new(r"\n{3,}").unwrap();
        cleaned = re.replace_all(&cleaned, "\n\n").to_string();

        // Trim whitespace
        cleaned = cleaned.trim().to_string();

        // Limit length to avoid token overflow (max ~12k chars for safety)
        if cleaned.len() > 12000 {
            cleaned.truncate(12000);
            cleaned.push_str("\n\n[Content truncated due to length...]");
        }

        cleaned
    }

    /// Sanitize content to prevent prompt injection attacks
    fn sanitize_prompt_injection(&self, content: &str) -> String {
        let mut sanitized = content.to_string();

        // Remove common prompt injection patterns
        let dangerous_patterns = [
            r"(?i)ignore\s+(all\s+)?previous\s+(instructions|prompts)",
            r"(?i)disregard\s+(all\s+)?(previous|above|prior)\s+(instructions|prompts|context)",
            r"(?i)forget\s+(all\s+)?(previous|above|prior)\s+(instructions|prompts)",
            r"(?i)new\s+instructions?:",
            r"(?i)system\s*:\s*you\s+are",
            r"(?i)you\s+are\s+now",
            r"(?i)roleplay\s+as",
            r"(?i)pretend\s+(you\s+are|to\s+be)",
            r"(?i)act\s+as\s+(if|a|an)",
            r"(?i)respond\s+as\s+(if|a|an)",
        ];

        for pattern in dangerous_patterns {
            let re = regex::Regex::new(pattern).unwrap();
            sanitized = re.replace_all(&sanitized, "[removed for security]").to_string();
        }

        // Remove excessive repetition (potential DoS)
        let repeat_re = regex::Regex::new(r"(.{1,50})\1{10,}").unwrap();
        sanitized = repeat_re.replace_all(&sanitized, "$1 [repetition removed]").to_string();

        sanitized
    }

    /// Fetch multiple URLs in parallel and combine their markdown content
    pub fn fetch_multiple(&self, urls: &[String]) -> Vec<(String, Result<String>)> {
        urls.iter()
            .map(|url| {
                let result = self.fetch_as_markdown(url);
                (url.clone(), result)
            })
            .collect()
    }
}

impl Default for ContentFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create ContentFetcher")
    }
}
