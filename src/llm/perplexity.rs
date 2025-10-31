use super::{provider::LLMProvider, Message};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const PERPLEXITY_API_URL: &str = "https://api.perplexity.ai/chat/completions";

pub struct PerplexityProvider {
    api_key: String,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
struct PerplexityRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct PerplexityResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

impl PerplexityProvider {
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { api_key, client })
    }
}

impl LLMProvider for PerplexityProvider {
    fn send_message(&self, messages: &[Message]) -> Result<String> {
        let request = PerplexityRequest {
            model: "sonar-pro".to_string(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 8000,
        };

        let response = self
            .client
            .post(PERPLEXITY_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .context("Failed to send request to Perplexity API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Perplexity API error ({}): {}", status, error_text);
        }

        let perplexity_response: PerplexityResponse = response
            .json()
            .context("Failed to parse Perplexity response")?;

        let content = perplexity_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Perplexity"))?;

        Ok(content)
    }

    fn name(&self) -> &str {
        "Perplexity"
    }
}
