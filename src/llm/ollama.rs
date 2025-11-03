use super::provider::LLMProvider;
use super::Message;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};

pub struct OllamaProvider {
    base_url: String,
    model: String,
    client: reqwest::blocking::Client,
    config: crate::config::OllamaConfig,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    options: OllamaOptions,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_ctx: usize,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    message: Message,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

impl OllamaProvider {
    pub fn new(config: crate::config::OllamaConfig) -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .context("Failed to create HTTP client")?;

        Self::check_connection(&client, &config.base_url)?;

        Ok(Self {
            base_url: config.base_url.clone(),
            model: config.model.clone(),
            client,
            config,
        })
    }

    fn check_connection(client: &reqwest::blocking::Client, base_url: &str) -> Result<()> {
        let url = format!("{}/api/tags", base_url);
        client
            .get(&url)
            .send()
            .context("Failed to connect to Ollama. Is Ollama running?")?;
        Ok(())
    }

    pub fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        let response: OllamaTagsResponse = self.client.get(&url).send()?.json()?;

        let models = response.models.iter().map(|m| m.name.clone()).collect();

        Ok(models)
    }

    pub fn pull_model(model: &str, base_url: &str) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/pull", base_url);

        #[derive(Serialize)]
        struct PullRequest {
            name: String,
        }

        let response = client
            .post(&url)
            .json(&PullRequest {
                name: model.to_string(),
            })
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to pull model: {}", response.status());
        }

        Ok(())
    }

    pub fn remove_model(model: &str, base_url: &str) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/api/delete", base_url);

        #[derive(Serialize)]
        struct DeleteRequest {
            name: String,
        }

        let response = client
            .delete(&url)
            .json(&DeleteRequest {
                name: model.to_string(),
            })
            .send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to remove model: {}", response.status());
        }

        Ok(())
    }
}

impl LLMProvider for OllamaProvider {
    fn send_message(&self, messages: &[Message]) -> Result<String> {
        let request = OllamaRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            stream: false,
            options: OllamaOptions {
                temperature: 0.7,
                num_ctx: self.config.context_window,
            },
        };

        let url = format!("{}/api/chat", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Ollama API error ({}): {}", status, error_text);
        }

        let ollama_response: OllamaResponse = response.json()?;
        Ok(ollama_response.message.content)
    }

    fn send_message_stream(
        &self,
        messages: &[Message],
        mut on_chunk: Box<dyn FnMut(&str)>,
    ) -> Result<String> {
        let request = OllamaRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            stream: true,
            options: OllamaOptions {
                temperature: 0.7,
                num_ctx: self.config.context_window,
            },
        };

        let url = format!("{}/api/chat", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .context("Failed to send streaming request to Ollama")?;

        let mut full_response = String::new();
        let reader = BufReader::new(response);

        for line in reader.lines() {
            let line = line.context("Failed to read stream line")?;
            if line.is_empty() {
                continue;
            }

            if let Ok(chunk_response) = serde_json::from_str::<OllamaResponse>(&line) {
                let content = &chunk_response.message.content;
                if !content.is_empty() {
                    on_chunk(content);
                    full_response.push_str(content);
                }

                if chunk_response.done {
                    break;
                }
            }
        }

        Ok(full_response)
    }

    fn name(&self) -> &str {
        "Ollama"
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn searches_web(&self) -> bool {
        false
    }
}
