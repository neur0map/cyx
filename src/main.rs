use clap::Parser;
use cyx::cli::{Cli, CliContext, CommandHandler};
use cyx::ui::Display;

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();

    // Create CLI context from flags
    let context = CliContext::new(cli.quiet, cli.verbose, cli.no_tty, cli.learn);

    // Auto-check for updates (once per day, non-blocking)
    if cyx::update::auto_check_update().is_err() {
        // Silently ignore auto-check errors
    }

    // Handle commands
    if let Err(e) = CommandHandler::handle(cli.query, cli.command, context) {
        Display::error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}
