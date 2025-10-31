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
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    #[serde(default)]
    message: Option<Message>,
    #[serde(default)]
    delta: Option<Delta>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(default)]
    content: Option<String>,
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
            stream: None,
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
            .and_then(|c| c.message.as_ref())
            .map(|m| m.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from Groq"))?;

        Ok(content)
    }

    fn send_message_stream(&self, messages: &[Message], mut on_chunk: Box<dyn FnMut(&str)>) -> Result<String> {
        use std::io::{BufRead, BufReader};

        let request = GroqRequest {
            model: "llama-3.3-70b-versatile".to_string(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 8000,
            stream: Some(true),
        };

        let response = self
            .client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .context("Failed to send streaming request to Groq API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Groq API error ({}): {}", status, error_text);
        }

        let mut full_response = String::new();
        let reader = BufReader::new(response);

        for line in reader.lines() {
            let line = line.context("Failed to read stream line")?;

            // Skip empty lines and non-data lines
            if line.is_empty() || !line.starts_with("data: ") {
                continue;
            }

            // Extract the JSON part
            let data = &line[6..]; // Skip "data: " prefix

            // Check for end of stream
            if data == "[DONE]" {
                break;
            }

            // Parse the SSE data
            if let Ok(chunk_response) = serde_json::from_str::<GroqResponse>(data) {
                if let Some(choice) = chunk_response.choices.first() {
                    if let Some(delta) = &choice.delta {
                        if let Some(content) = &delta.content {
                            on_chunk(content);
                            full_response.push_str(content);
                        }
                    }
                }
            }
        }

        Ok(full_response)
    }

    fn name(&self) -> &str {
        "Groq"
    }

    fn model(&self) -> &str {
        "llama-3.3-70b-versatile"
    }

    fn searches_web(&self) -> bool {
        false // Groq uses knowledge base only
    }
}
