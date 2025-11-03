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
        use crate::deps::{DependencyChecker, DependencyStatus};
        
        println!("{}", "Cyx Configuration Setup".bold().cyan());
        println!("Let's get you set up with dependencies and preferences.\n");

        let mut config = Config::default();

        // Check Ollama availability
        println!("{}", "Step 1: Checking dependencies...".bold().yellow());
        println!("{}", "─".repeat(60));
        
        let checker = DependencyChecker::new()?;
        let results = checker.check_all()?;
        
        for result in &results {
            match result.status {
                DependencyStatus::Installed { ref version } => {
                    println!("  {} {} {}", "[✓]".green().bold(), result.name.bold(), format!("({})", version).dimmed());
                }
                DependencyStatus::NotInstalled => {
                    println!("  {} {} {}", "[✗]".red().bold(), result.name, "(not installed)".dimmed());
                }
                _ => {}
            }
        }
        println!();

        let ollama_available = results.iter()
            .any(|r| r.name == "Ollama" && matches!(r.status, DependencyStatus::Installed { .. }));

        // ═══════════════════════════════════════════════
        // STEP 2: Install Missing Dependencies
        // ═══════════════════════════════════════════════
        if !ollama_available {
            println!("{}", "Step 2: Install Ollama (Optional)".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());
            println!("Ollama enables local LLM inference with zero API costs.");
            println!("Models run on your machine (requires 4-8GB RAM).\n");

            let install_choices = vec![
                "Yes, install Ollama now (recommended)",
                "No, use cloud providers only (Groq/Perplexity)",
                "I'll install Ollama manually later"
            ];

            let install_choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Install Ollama?")
                .items(&install_choices)
                .default(0)
                .interact()?;

            if install_choice == 0 {
                println!("\n{}", "Installing Ollama...".cyan());
                if let Err(e) = crate::deps::OllamaInstaller::install() {
                    println!("{} Failed to install Ollama: {}", "[!]".yellow(), e);
                    println!("You can install manually from: {}\n", "https://ollama.com".cyan());
                } else {
                    println!("{} Ollama installed successfully!", "[✓]".green());
                    
                    // Start Ollama service
                    println!("{}", "Starting Ollama service...".cyan());
                    if let Err(e) = crate::deps::OllamaInstaller::start_service() {
                        println!("{} Could not start service: {}", "[!]".yellow(), e);
                    } else {
                        println!("{} Ollama is running\n", "[✓]".green());
                    }
                }
            }
            println!();
        }

        // Re-check Ollama availability after potential installation
        let ollama_available = crate::deps::OllamaInstaller::check_available();

        // Build provider list based on availability
        let mut providers = vec![];
        if ollama_available {
            providers.push("Ollama - Local models (free, private, offline) [RECOMMENDED]");
        }
        providers.push("Groq - Cloud API (fast, free tier available)");
        providers.push("Perplexity - Cloud API (web search enabled)");

        // ═══════════════════════════════════════════════
        // STEP 3: Provider Selection
        // ═══════════════════════════════════════════════
        println!("{}", "Step 3: LLM Provider Selection".bold().yellow());
        println!("{}", "─".repeat(60).dimmed());
        let provider_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred LLM provider")
            .items(&providers)
            .default(0)
            .interact()?;

        let selected_provider = providers[provider_idx];

        // ═══════════════════════════════════════════════
        // STEP 4: Provider Configuration
        // ═══════════════════════════════════════════════
        if selected_provider.starts_with("Ollama") {
            config.provider = super::LLMProvider::Ollama;

            println!("\n{}", "Step 4: Ollama Model Selection".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());

            // Check for installed models
            if let Ok(ollama_provider) = crate::llm::OllamaProvider::new(super::OllamaConfig::default()) {
                let models = ollama_provider.list_models().unwrap_or_default();

                if models.is_empty() {
                    println!("No models installed yet. Let's download one!\n");

                    let model_profiles = vec![
                        "fast - llama3.2:3b (2 GB, quick responses)",
                        "balanced - mistral:7b-instruct (4.1 GB, best quality/speed) [RECOMMENDED]",
                        "quality - mixtral:8x7b (26 GB, highest quality)",
                        "code - codellama:7b-instruct (3.8 GB, code-focused)"
                    ];

                    let model_idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select model profile")
                        .items(&model_profiles)
                        .default(1)
                        .interact()?;

                    let model_name = match model_idx {
                        0 => "llama3.2:3b",
                        1 => "mistral:7b-instruct",
                        2 => "mixtral:8x7b",
                        3 => "codellama:7b-instruct",
                        _ => "mistral:7b-instruct",
                    };

                    println!("\n{} Downloading {}...", "[~]".cyan(), model_name);
                    println!("This may take a few minutes depending on your connection.\n");

                    if let Err(e) = crate::llm::OllamaProvider::pull_model(model_name, &super::OllamaConfig::default().base_url) {
                        println!("{} Failed to download model: {}", "[!]".red(), e);
                        println!("You can try manually: {}", format!("ollama pull {}", model_name).cyan());
                    } else {
                        println!("{} Model downloaded successfully!\n", "[✓]".green());
                    }
                    config.ollama.model = model_name.to_string();
                } else {
                    println!("{}", "Installed Ollama models:".bold());
                    for model in &models {
                        println!("  • {}", model.cyan());
                    }
                    println!();
                    
                    let model_idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select model to use")
                        .items(&models)
                        .default(0)
                        .interact()?;

                    config.ollama.model = models[model_idx].clone();
                }
            }
        } else if selected_provider.starts_with("Groq") {
            config.provider = super::LLMProvider::Groq;
            
            println!("\n{}", "Step 4: API Key Configuration".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());
            println!("{}", "Tip: You can add more providers later with 'cyx config set'".dimmed());
            println!();
            
            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Groq API key")
                .interact_text()?;
            config.api_keys.groq = Some(api_key);
        } else {
            config.provider = super::LLMProvider::Perplexity;
            
            println!("\n{}", "Step 4: API Key Configuration".bold().yellow());
            println!("{}", "─".repeat(60).dimmed());
            println!("{}", "Tip: You can add more providers later with 'cyx config set'".dimmed());
            println!();
            
            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your Perplexity API key")
                .interact_text()?;
            config.api_keys.perplexity = Some(api_key);
        }

        // ═══════════════════════════════════════════════
        // STEP 5: Cache Configuration
        // ═══════════════════════════════════════════════
        println!("\n{}", "Step 5: Smart Cache Configuration".bold().yellow());
        println!("{}", "─".repeat(60).dimmed());
        println!("Smart caching reduces API costs and improves response time.\n");

        let enable_cache = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Enable smart caching?")
            .default(true)
            .interact()?;

        config.cache.enabled = enable_cache;

        if enable_cache {
            println!();
            let model_sizes = vec![
                "small - all-MiniLM-L6-v2 (86 MB, 384D, good balance) [RECOMMENDED]",
                "medium - bge-base-en-v1.5 (400 MB, 768D, better accuracy)",
                "large - e5-large-v2 (1.3 GB, 1024D, best accuracy)"
            ];

            let size_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select embedding model for semantic search")
                .items(&model_sizes)
                .default(0)
                .interact()?;

            let model_size = match size_idx {
                0 => "small",
                1 => "medium",
                2 => "large",
                _ => "small",
            };

            config.cache.embedding_model = model_size.to_string();

            println!("\n{} Embedding model configured: {}", "[~]".cyan(), model_size);
            println!("Model will be downloaded automatically on first use.");
            println!("Or download now with: {}\n", format!("cyx download-model {}", model_size).cyan());
        }

        // ═══════════════════════════════════════════════
        // STEP 6: Validation & Summary
        // ═══════════════════════════════════════════════
        println!("\n{}", "Step 6: Validation".bold().yellow());
        println!("{}", "─".repeat(60).dimmed());
        println!("Testing configuration...\n");

        // Test provider connection
        print!("  Testing {} connection... ", format!("{:?}", config.provider).cyan());
        std::io::Write::flush(&mut std::io::stdout())?;
        
        match Self::test_provider(&config) {
            Ok(_) => println!("{}", "[✓]".green()),
            Err(e) => {
                println!("{}", "[✗]".red());
                println!("  {}: {}\n", "Error".red(), e);
                println!("{}", "Warning: Provider connection failed. Please check your configuration.".yellow());
                println!("You can test it later with: {}\n", "cyx \"test query\"".cyan());
            }
        }

        // Initialize cache if enabled
        if config.cache.enabled {
            print!("  Initializing cache... ");
            std::io::Write::flush(&mut std::io::stdout())?;
            
            let cache_dir = Config::cache_dir()?;
            match crate::cache::storage::CacheStorage::new(&cache_dir) {
                Ok(_) => println!("{}", "[✓]".green()),
                Err(e) => {
                    println!("{}", "[✗]".red());
                    println!("  {}: {}\n", "Error".red(), e);
                }
            }
        }

        // Save configuration
        Self::save(&config)?;

        println!();
        println!("{}", "═".repeat(60).green());
        println!("{}",  "  ✓ Configuration Complete!".green().bold());
        println!("{}", "═".repeat(60).green());
        println!();
        println!("{}",format!("  Provider:  {}", format!("{:?}", config.provider).cyan()).bold());
        if config.cache.enabled {
            println!("{}", format!("  Cache:     Enabled ({} model)", config.cache.embedding_model).bold());
        } else {
            println!("{}", "  Cache:     Disabled".dimmed());
        }
        println!();
        println!("Config saved to: {}", Config::config_path()?.display().to_string().cyan());
        println!();
        println!("{}", "You're ready to use Cyx!".bold());
        println!();
        println!("{}", "Try it out:".bold());
        println!("  {} {}", "cyx".green(), "\"nmap stealth scan\"".dimmed());
        println!("  {} {}", "cyx cache stats".green(), "(view cache performance)".dimmed());
        println!("  {} {}", "cyx doctor".green(), "(check system health)".dimmed());
        println!();

        Ok(config)
    }

    /// Test provider connection
    fn test_provider(config: &Config) -> Result<()> {
        use crate::llm::{LLMProvider, groq::GroqProvider, perplexity::PerplexityProvider, OllamaProvider};

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
            super::LLMProvider::Ollama => {
                Box::new(OllamaProvider::new(config.ollama.clone())?)
            }
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
                config.cache.ttl_days = value.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid number for ttl_days"))?;
            }
            _ => anyhow::bail!("Unknown config key: {}. Try: provider, cache.enabled, cache.ttl_days", key),
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
            "groq_api_key" => config.api_keys.groq.unwrap_or_else(|| "Not set".to_string()),
            "perplexity_api_key" => config.api_keys.perplexity.unwrap_or_else(|| "Not set".to_string()),
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
