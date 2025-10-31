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

        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save config to file with secure permissions (600)
    pub fn save(config: &Config) -> Result<()> {
        let config_dir = Config::config_dir()?;
        let config_path = Config::config_path()?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context("Failed to create config directory")?;
        }

        // Serialize config to TOML
        let content = toml::to_string_pretty(config)
            .context("Failed to serialize config")?;

        // Write to file
        fs::write(&config_path, content)
            .context("Failed to write config file")?;

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
        println!("Let's get you set up with API keys and preferences.\n");

        let mut config = Config::default();

        // Select LLM provider
        let providers = vec!["Groq (Fast, Free tier available)", "Perplexity (Search-optimized)"];
        let provider_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred LLM provider")
            .items(&providers)
            .default(0)
            .interact()?;

        config.provider = if provider_idx == 0 {
            super::LLMProvider::Groq
        } else {
            super::LLMProvider::Perplexity
        };

        // Get API keys
        println!("\n{}", "API Keys:".bold());
        println!("{}", "Tip: You can add more providers later with 'cyx config set'".dimmed());

        match config.provider {
            super::LLMProvider::Groq => {
                let api_key: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your Groq API key")
                    .interact_text()?;
                config.api_keys.groq = Some(api_key);
            }
            super::LLMProvider::Perplexity => {
                let api_key: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your Perplexity API key")
                    .interact_text()?;
                config.api_keys.perplexity = Some(api_key);
            }
        }

        // Save configuration
        Self::save(&config)?;

        println!("\n{}", "[+] Configuration saved successfully!".green().bold());
        println!("Config location: {}", Config::config_path()?.display().to_string().dimmed());

        Ok(config)
    }

    /// Set a specific configuration value
    pub fn set_value(key: &str, value: &str) -> Result<()> {
        let mut config = Self::load()?;

        match key {
            "provider" => {
                config.provider = match value.to_lowercase().as_str() {
                    "groq" => super::LLMProvider::Groq,
                    "perplexity" => super::LLMProvider::Perplexity,
                    _ => anyhow::bail!("Invalid provider. Options: groq, perplexity"),
                };
            }
            "groq_api_key" => {
                config.api_keys.groq = Some(value.to_string());
            }
            "perplexity_api_key" => {
                config.api_keys.perplexity = Some(value.to_string());
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }

        Self::save(&config)?;
        println!("{}", format!("[+] Updated {}", key).green());
        Ok(())
    }

    /// Get a specific configuration value
    pub fn get_value(key: &str) -> Result<String> {
        let config = Self::load()?;

        let value = match key {
            "provider" => format!("{:?}", config.provider),
            "groq_api_key" => config.api_keys.groq.unwrap_or_else(|| "Not set".to_string()),
            "perplexity_api_key" => config.api_keys.perplexity.unwrap_or_else(|| "Not set".to_string()),
            "config_path" => Config::config_path()?.display().to_string(),
            _ => anyhow::bail!("Unknown config key: {}", key),
        };

        Ok(value)
    }
}
