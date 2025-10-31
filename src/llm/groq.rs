use super::{provider::LLMProvider, Message};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

pub struct GroqProvider {
    api_key: String,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

impl GroqProvider {
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { api_key, client })
    }
}

impl LLMProvider for GroqProvider {
    fn send_message(&self, messages: &[Message]) -> Result<String> {
        let request = GroqRequest {
            model: "llama-3.3-70b-versatile".to_string(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 8000,
        };

        let response = self
            .client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .context("Failed to send request to Groq API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Groq API error ({}): {}", status, error_text);
        }

        let groq_response: GroqResponse = response
            .json()
            .context("Failed to parse Groq response")?;

        let content = groq_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Groq"))?;

        Ok(content)
    }

    fn name(&self) -> &str {
        "Groq"
    }
}
