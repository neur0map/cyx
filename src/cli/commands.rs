use super::args::{CacheAction, Commands, ConfigAction, OllamaAction};
use super::context::CliContext;
use crate::{
    cache::CacheStorage,
    config::{Config, ConfigManager},
    deps::{DependencyChecker, DependencyStatus, OnnxLibraryFixer},
    session::InteractiveSession,
    ui::Display,
};
use anyhow::Result;
use colored::Colorize;

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(
        query: Option<String>,
        command: Option<Commands>,
        context: CliContext,
    ) -> Result<()> {
        match command {
            Some(Commands::Setup) => {
                Self::setup(&context)?;
            }
            Some(Commands::Config { action }) => {
                Self::config(action)?;
            }
            Some(Commands::Doctor) => {
                Self::doctor()?;
            }
            Some(Commands::Ollama { action }) => {
                Self::ollama(action)?;
            }
            Some(Commands::Cache { action }) => {
                Self::cache(action)?;
            }
            Some(Commands::DownloadModel { size }) => {
                Self::download_model(&size)?;
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
        // Check and fix ONNX library issues first
        println!();
        OnnxLibraryFixer::auto_fix()?;
        println!();

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
                println!("{}", "Current Configuration".bold().cyan());
                println!("{}", "─".repeat(60));
                println!();
                println!("{}", "Provider:".bold());
                println!("  {:?}", config.provider);
                println!();
                println!("{}", "API Keys:".bold());
                println!(
                    "  Groq: {}",
                    if config.api_keys.groq.is_some() {
                        "Set".green()
                    } else {
                        "Not set".dimmed()
                    }
                );
                println!(
                    "  Perplexity: {}",
                    if config.api_keys.perplexity.is_some() {
                        "Set".green()
                    } else {
                        "Not set".dimmed()
                    }
                );
                println!();
                println!("{}", "Ollama:".bold());
                println!("  Model: {}", config.ollama.model.cyan());
                println!("  Base URL: {}", config.ollama.base_url);
                println!();
                println!("{}", "Cache:".bold());
                println!(
                    "  Enabled: {}",
                    if config.cache.enabled {
                        "Yes".green()
                    } else {
                        "No".yellow()
                    }
                );
                println!("  TTL: {} days", config.cache.ttl_days);
                println!();
                println!("{}", "Config file:".dimmed());
                println!(
                    "  {}",
                    Config::config_path()?.display().to_string().dimmed()
                );
            }
        }

        Ok(())
    }

    fn doctor() -> Result<()> {
        println!("{}", "System Status Check".bold().cyan());
        println!("{}", "─".repeat(60));
        println!();

        let checker = DependencyChecker::new()?;
        let results = checker.check_all()?;

        for result in results {
            match result.status {
                DependencyStatus::Installed { ref version } => {
                    println!(
                        "{} {} {}",
                        "[✓]".green(),
                        result.name,
                        format!("({})", version).dimmed()
                    );
                }
                DependencyStatus::NotInstalled => {
                    println!(
                        "{} {} {}",
                        "[✗]".red(),
                        result.name,
                        "(not installed)".dimmed()
                    );
                    println!("    {}", result.instructions.dimmed());
                }
                DependencyStatus::WrongVersion {
                    ref current,
                    ref required,
                } => {
                    println!(
                        "{} {} {} {}",
                        "[!]".yellow(),
                        result.name,
                        format!("(current: {}, required: {})", current, required).dimmed(),
                        "(wrong version)".yellow()
                    );
                }
            }
        }

        println!();
        Ok(())
    }

    fn ollama(action: OllamaAction) -> Result<()> {
        use crate::config::OllamaConfig;
        use crate::llm::OllamaProvider;

        match action {
            OllamaAction::List => {
                let provider = OllamaProvider::new(OllamaConfig::default())?;
                let models = provider.list_models()?;

                if models.is_empty() {
                    println!("{}", "No models installed.".yellow());
                    println!(
                        "Use {} to download a model.",
                        "cyx ollama pull <model>".cyan()
                    );
                } else {
                    println!("{}", "Installed Ollama models:".bold());
                    for model in models {
                        println!("  • {}", model.cyan());
                    }
                }
            }
            OllamaAction::Pull { model } => {
                println!("{}", format!("Downloading {}...", model).cyan());
                OllamaProvider::pull_model(&model, &OllamaConfig::default().base_url)?;
                println!("{}", format!("✓ Successfully pulled {}", model).green());
            }
            OllamaAction::Remove { model } => {
                println!("{}", format!("Removing {}...", model).cyan());
                OllamaProvider::remove_model(&model, &OllamaConfig::default().base_url)?;
                println!("{}", format!("✓ Successfully removed {}", model).green());
            }
        }

        Ok(())
    }

    fn download_model(size: &str) -> Result<()> {
        use crate::cache::ONNXEmbedder;

        let cache_dir = Config::cache_dir()?;
        let models_dir = cache_dir.join("models");
        std::fs::create_dir_all(&models_dir)?;

        println!("📦 Downloading ONNX embedding model: {}", size);

        tokio::runtime::Runtime::new()?
            .block_on(async { ONNXEmbedder::download_model(size, &models_dir).await })?;

        println!("[+] Model downloaded successfully!");
        println!("Location: {}", models_dir.join(size).display());

        Ok(())
    }

    fn cache(action: CacheAction) -> Result<()> {
        let cache_dir = Config::cache_dir()?;
        let storage = CacheStorage::new(&cache_dir)?;

        match action {
            CacheAction::Stats => {
                let stats = storage.stats()?;

                println!("{}", "Cache Statistics".bold().cyan());
                println!("{}", "─".repeat(60));
                println!(
                    "  Total entries: {}",
                    stats.total_entries.to_string().green()
                );
                println!(
                    "  Cache size: {}",
                    format_bytes(stats.total_size_bytes).green()
                );
                println!("  Hit count: {}", stats.hit_count.to_string().green());
                println!("  Miss count: {}", stats.miss_count.to_string().yellow());

                let total_requests = stats.hit_count + stats.miss_count;
                if total_requests > 0 {
                    let hit_rate = (stats.hit_count as f64 / total_requests as f64) * 100.0;
                    println!("  Hit rate: {:.1}%", hit_rate);
                }

                if let Some(oldest) = stats.oldest_entry {
                    println!(
                        "  Oldest entry: {}",
                        oldest.format("%Y-%m-%d %H:%M:%S").to_string().dimmed()
                    );
                }
                if let Some(newest) = stats.newest_entry {
                    println!(
                        "  Newest entry: {}",
                        newest.format("%Y-%m-%d %H:%M:%S").to_string().dimmed()
                    );
                }

                println!(
                    "  Cache location: {}",
                    cache_dir.display().to_string().dimmed()
                );
            }

            CacheAction::List { limit } => {
                let queries = storage.list_all(Some(limit))?;

                if queries.is_empty() {
                    println!("{}", "No cached queries yet.".yellow());
                    println!("Run some queries to populate the cache!");
                    return Ok(());
                }

                println!(
                    "{}",
                    format!("Recent Cached Queries (showing {})", queries.len())
                        .bold()
                        .cyan()
                );
                println!("{}", "─".repeat(80));

                for query in queries {
                    println!();
                    println!("  {}: {}", "Query".bold(), query.query_original.cyan());
                    println!("  {}: {}", "Hash".dimmed(), query.query_hash.dimmed());
                    println!(
                        "  {}: {} | {}: {}",
                        "Provider".dimmed(),
                        query.provider,
                        "Model".dimmed(),
                        query.model
                    );
                    println!(
                        "  {}: {} | {}: {}",
                        "Accessed".dimmed(),
                        query.access_count,
                        "Last access".dimmed(),
                        query.last_accessed.format("%Y-%m-%d %H:%M")
                    );

                    let response_preview = if query.response.len() > 100 {
                        format!("{}...", &query.response[..100])
                    } else {
                        query.response.clone()
                    };
                    println!("  {}: {}", "Response".dimmed(), response_preview.dimmed());
                }
            }

            CacheAction::Clear => {
                println!("{}", "This will delete all cached queries.".yellow());
                let confirm =
                    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
                        .with_prompt("Are you sure?")
                        .default(false)
                        .interact()?;

                if confirm {
                    let count = storage.clear()?;
                    println!("{}", format!("✓ Cleared {} cached queries", count).green());
                } else {
                    println!("{}", "Cancelled.".dimmed());
                }
            }

            CacheAction::Remove { hash } => {
                let removed = storage.remove_by_hash(&hash)?;
                if removed {
                    println!(
                        "{}",
                        format!("✓ Removed cached query with hash {}", hash).green()
                    );
                } else {
                    println!(
                        "{}",
                        format!("Query with hash {} not found in cache", hash).yellow()
                    );
                }
            }

            CacheAction::Cleanup { days } => {
                println!(
                    "{}",
                    format!("Cleaning up entries older than {} days...", days).cyan()
                );
                let count = storage.cleanup_old_entries(days)?;

                if count > 0 {
                    println!(
                        "{}",
                        format!("✓ Removed {} old cache entries", count).green()
                    );
                } else {
                    println!("{}", "No old entries to remove.".dimmed());
                }

                // Show updated stats
                let stats = storage.stats()?;
                println!(
                    "\nRemaining: {} entries, {}",
                    stats.total_entries,
                    format_bytes(stats.total_size_bytes)
                );
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
            crate::config::LLMProvider::Ollama => false, // Ollama doesn't need API key
        };

        if api_key_missing {
            Display::warning("API key not configured for selected provider.");
            Display::info("Running setup...");
            return ConfigManager::interactive_setup();
        }

        Ok(config)
    }
}

fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
