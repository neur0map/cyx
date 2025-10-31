use super::args::{Commands, ConfigAction};
use super::context::CliContext;
use crate::{
    config::{Config, ConfigManager},
    session::InteractiveSession,
    ui::Display,
};
use anyhow::Result;

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(query: Option<String>, command: Option<Commands>, context: CliContext) -> Result<()> {
        match command {
            Some(Commands::Setup) => {
                Self::setup(&context)?;
            }
            Some(Commands::Config { action }) => {
                Self::config(action)?;
            }
            None => {
                // No subcommand specified - require query
                if let Some(query_text) = query {
                    Self::one_shot(&query_text, context)?;
                } else {
                    anyhow::bail!("No query provided. Usage: cyx \"your query here\"");
                }
            }
        }

        Ok(())
    }

    fn setup(_context: &CliContext) -> Result<()> {
        ConfigManager::interactive_setup()?;
        Ok(())
    }

    fn one_shot(query: &str, context: CliContext) -> Result<()> {
        let config = Self::load_or_setup_config()?;
        InteractiveSession::one_shot(config, query, context)?;
        Ok(())
    }

    fn config(action: ConfigAction) -> Result<()> {
        match action {
            ConfigAction::Set { key, value } => {
                ConfigManager::set_value(&key, &value)?;
            }
            ConfigAction::Get { key } => {
                let value = ConfigManager::get_value(&key)?;
                println!("{}: {}", key, value);
            }
            ConfigAction::Show => {
                let config = ConfigManager::load()?;
                println!("Current configuration:");
                println!("  Provider: {:?}", config.provider);
                println!("  Groq API Key: {}", if config.api_keys.groq.is_some() { "Set" } else { "Not set" });
                println!("  Perplexity API Key: {}", if config.api_keys.perplexity.is_some() { "Set" } else { "Not set" });
                println!("  Config file: {}", Config::config_path()?.display());
            }
        }

        Ok(())
    }

    /// Load config or run setup if not configured
    fn load_or_setup_config() -> Result<Config> {
        let config_path = Config::config_path()?;

        if !config_path.exists() {
            Display::info("First time setup required.");
            return ConfigManager::interactive_setup();
        }

        let config = ConfigManager::load()?;

        // Validate config has required API key
        let api_key_missing = match config.provider {
            crate::config::LLMProvider::Groq => config.api_keys.groq.is_none(),
            crate::config::LLMProvider::Perplexity => config.api_keys.perplexity.is_none(),
        };

        if api_key_missing {
            Display::warning("API key not configured for selected provider.");
            Display::info("Running setup...");
            return ConfigManager::interactive_setup();
        }

        Ok(config)
    }
}
