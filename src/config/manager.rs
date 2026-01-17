use super::Config;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub struct ConfigManager;

impl ConfigManager {
    /// Load config from file, or create default if it doesn't exist
    pub fn load() -> Result<Config> {
        let config_path = Config::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save config to file with secure permissions (600)
    pub fn save(config: &Config) -> Result<()> {
        let config_dir = Config::config_dir()?;
        let config_path = Config::config_path()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        // Serialize config to TOML
        let content = toml::to_string_pretty(config).context("Failed to serialize config")?;

        // Write to file
        fs::write(&config_path, content).context("Failed to write config file")?;

        // Set permissions to 600 (read/write for owner only)
        let mut perms = fs::metadata(&config_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_path, perms)
            .context("Failed to set config file permissions")?;

        Ok(())
    }

    /// Interactive setup wizard for first-time configuration
    pub fn interactive_setup() -> Result<Config> {
        println!("{}", "Cyx Configuration Setup".bold().cyan());
        println!("Fast and simple - let's get you started!\n");

        let mut config = Config::default();

        // Check if Ollama is available (optional)
        let ollama_available = crate::deps::OllamaInstaller::check_available();

        // Build provider list - Groq and Perplexity first (cloud providers)
        let mut providers = vec![];
        providers.push("Groq - Cloud API (fast, generous free tier) [RECOMMENDED]");
        providers.push("Perplexity - Cloud API (web search enabled)");
        if ollama_available {
            providers.push("Ollama - Local models (advanced, requires manual setup)");
        }

        // ═══════════════════════════════════════════════
        // STEP 1: Provider Selection
        // ═══════════════════════════════════════════════
        println!("{}", "Step 1: LLM Provider Selection".bold().yellow());
        println!("{}", "─".repeat(60).dimmed());
        println!("Get your API key:");
        println!("  • Groq:       {}", "https://console.groq.com/".cyan());
        println!("  • Perplexity: {}", "https://www.perplexity.ai/settings/api".cyan());
        println!();

        let provider_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred LLM provider")
            .items(&providers)
            .default(0)
            .interact()?;

        let selected_provider = providers[provider_idx];

        // ═══════════════════════════════════════════════
        // STEP 2: Provider Configuration
        // ═══════════════════════════════════════════════
        if selected_provider.starts_with("Ollama") {
            config.provider = super::LLMProvider::Ollama;

            println!("\n{}", "Step 2: Ollama Configuration".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());
            println!("Note: You must have Ollama installed and models downloaded.");
            println!("Install from: {}", "https://ollama.com".cyan());
            println!("Download models with: {}\n", "ollama pull mistral".cyan());

            let model: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter Ollama model name (e.g., mistral:7b-instruct)")
                .default("mistral:7b-instruct".to_string())
                .interact_text()?;

            config.ollama.model = model;
        } else if selected_provider.starts_with("Groq") {
            config.provider = super::LLMProvider::Groq;

            println!("\n{}", "Step 2: Groq API Key".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());

            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Groq API key")
                .interact_text()?;
            config.api_keys.groq = Some(api_key);
        } else {
            config.provider = super::LLMProvider::Perplexity;

            println!("\n{}", "Step 2: Perplexity API Key".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());

            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Perplexity API key")
                .interact_text()?;
            config.api_keys.perplexity = Some(api_key);
        }

        // Auto-enable cache with default settings (no prompts)
        config.cache.enabled = true;
        config.cache.embedding_model = "small".to_string();

        // ═══════════════════════════════════════════════
        // Validation & Summary
        // ═══════════════════════════════════════════════
        println!("\n{}", "Testing configuration...".cyan());

        // Test provider connection
        print!("  {} connection... ", format!("{:?}", config.provider).cyan());
        std::io::Write::flush(&mut std::io::stdout())?;

        match Self::test_provider(&config) {
            Ok(_) => println!("{}", "[✓]".green()),
            Err(e) => {
                println!("{}", "[✗]".red());
                println!("  {}: {}", "Error".red(), e);
                println!(
                    "\n{} Connection test failed. Check your configuration and try: {}\n",
                    "[!]".yellow(),
                    "cyx \"test query\"".cyan()
                );
            }
        }

        // Save configuration
        Self::save(&config)?;

        println!();
        println!("{}", "═".repeat(60).green());
        println!("{}", "  ✓ Setup Complete!".green().bold());
        println!("{}", "═".repeat(60).green());
        println!();
        println!("  Provider: {}", format!("{:?}", config.provider).cyan().bold());
        println!("  Cache:    {} (auto-enabled)", "Smart".green());
        println!();
        println!("{}", "You're ready to go! Try your first query:".bold());
        println!();
        println!("  {}", "cyx \"nmap stealth scan\"".green());
        println!();

        Ok(config)
    }

    /// Test provider connection
    fn test_provider(config: &Config) -> Result<()> {
        use crate::llm::{
            groq::GroqProvider, perplexity::PerplexityProvider, LLMProvider, OllamaProvider,
        };

        // Create provider based on config
        let provider: Box<dyn LLMProvider> = match config.provider {
            super::LLMProvider::Groq => {
                let api_key = config
                    .api_keys
                    .groq
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Groq API key not configured"))?;
                Box::new(GroqProvider::new(api_key)?)
            }
            super::LLMProvider::Perplexity => {
                let api_key = config
                    .api_keys
                    .perplexity
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Perplexity API key not configured"))?;
                Box::new(PerplexityProvider::new(api_key)?)
            }
            super::LLMProvider::Ollama => Box::new(OllamaProvider::new(config.ollama.clone())?),
        };

        // Try a minimal test query
        let test_messages = vec![crate::llm::Message {
            role: "user".to_string(),
            content: "test".to_string(),
        }];

        // Just test the connection, ignore the response
        let _ = provider.send_message(&test_messages)?;

        Ok(())
    }

    /// Set a specific configuration value
    pub fn set_value(key: &str, value: &str) -> Result<()> {
        let mut config = Self::load()?;

        match key {
            "provider" => {
                config.provider = match value.to_lowercase().as_str() {
                    "groq" => super::LLMProvider::Groq,
                    "perplexity" => super::LLMProvider::Perplexity,
                    "ollama" => super::LLMProvider::Ollama,
                    _ => anyhow::bail!("Invalid provider. Options: groq, perplexity, ollama"),
                };
            }
            "groq_api_key" => {
                config.api_keys.groq = Some(value.to_string());
            }
            "perplexity_api_key" => {
                config.api_keys.perplexity = Some(value.to_string());
            }
            "ollama_model" => {
                config.ollama.model = value.to_string();
            }
            "ollama_base_url" => {
                config.ollama.base_url = value.to_string();
            }
            "cache.enabled" => {
                config.cache.enabled = value.to_lowercase() == "true";
            }
            "cache.ttl_days" => {
                config.cache.ttl_days = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number for ttl_days"))?;
            }
            _ => anyhow::bail!(
                "Unknown config key: {}. Try: provider, cache.enabled, cache.ttl_days",
                key
            ),
        }

        Self::save(&config)?;
        println!("{}", format!("✓ Updated {}", key).green());
        Ok(())
    }

    /// Get a specific configuration value
    pub fn get_value(key: &str) -> Result<String> {
        let config = Self::load()?;

        let value = match key {
            "provider" => format!("{:?}", config.provider),
            "groq_api_key" => config
                .api_keys
                .groq
                .unwrap_or_else(|| "Not set".to_string()),
            "perplexity_api_key" => config
                .api_keys
                .perplexity
                .unwrap_or_else(|| "Not set".to_string()),
            "ollama_model" => config.ollama.model,
            "ollama_base_url" => config.ollama.base_url,
            "cache.enabled" => config.cache.enabled.to_string(),
            "cache.ttl_days" => config.cache.ttl_days.to_string(),
            "config_path" => Config::config_path()?.display().to_string(),
            _ => anyhow::bail!("Unknown config key: {}", key),
        };

        Ok(value)
    }
}
