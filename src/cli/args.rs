use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cyx",
    version,
    about = "Cyx: Your cybersecurity companion in command.",
    long_about = "A fast, terminal-based cybersecurity companion that retrieves commands, \
                  documentation, and techniques from trusted sources like HackTricks, \
                  PayloadsAllTheThings, and OWASP."
)]
pub struct Cli {
    /// Query to search for (one-shot mode)
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,

    /// Quiet mode - only show final response (no banners, tables, etc.)
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Verbose mode - show detailed progress and debugging info
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// No-TTY mode - disable interactive prompts and colors (for scripting/testing)
    #[arg(long, global = true)]
    pub no_tty: bool,

    /// Learn mode - show detailed command explanations with flag breakdowns
    #[arg(short, long, global = true)]
    pub learn: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure Cyx settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Initial setup wizard (Groq/Perplexity API key)
    Setup,

    /// Check system dependencies and health
    Doctor,

    /// Manage Ollama models (Advanced - requires Ollama installed)
    Ollama {
        #[command(subcommand)]
        action: OllamaAction,
    },

    /// Manage query cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },

    /// Check for updates (cargo install cyx --force to update)
    Update {
        /// Check for updates without installing
        #[arg(long)]
        check_only: bool,
    },
}

#[derive(Subcommand)]
pub enum OllamaAction {
    /// List installed models
    List,

    /// Pull/download a model
    Pull {
        /// Model name (e.g., mistral:7b-instruct)
        #[arg(value_name = "MODEL")]
        model: String,
    },

    /// Remove a model
    Remove {
        /// Model name to remove
        #[arg(value_name = "MODEL")]
        model: String,
    },
}

#[derive(Subcommand)]
pub enum CacheAction {
    /// Show cache statistics
    Stats,

    /// List cached queries
    List {
        /// Maximum number of entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Clear all cached queries
    Clear,

    /// Remove a specific cached query by hash
    Remove {
        /// Query hash to remove
        #[arg(value_name = "HASH")]
        hash: String,
    },

    /// Clean up old cache entries
    Cleanup {
        /// Remove entries older than N days
        #[arg(short, long, default_value = "30")]
        days: u32,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Configuration key
        #[arg(value_name = "KEY")]
        key: String,

        /// Configuration value
        #[arg(value_name = "VALUE")]
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// Show all configuration
    Show,
}
