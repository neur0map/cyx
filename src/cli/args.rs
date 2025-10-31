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

    /// Run initial setup wizard
    Setup,
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
