pub mod checker;
pub mod metadata;

pub use checker::VersionChecker;
pub use metadata::UpdateMetadata;

use anyhow::Result;
use colored::Colorize;

pub struct UpdateManager {
    checker: VersionChecker,
}

impl UpdateManager {
    pub fn new() -> Result<Self> {
        let checker = VersionChecker::new()?;

        Ok(Self { checker })
    }

    /// Check and display update availability
    pub fn check_and_display(&self) -> Result<()> {
        println!();
        println!("{}", "Checking for updates...".cyan());

        let update_info = self.checker.check()?;

        println!();
        if update_info.needs_update {
            println!("{}", "Update Available!".green().bold());
            println!("{}", "─".repeat(60));
            println!(
                "  Current version: {}",
                format!("v{}", update_info.current_version).yellow()
            );
            println!(
                "  Latest version:  {}",
                format!("v{}", update_info.latest_version).green()
            );
            println!();
            println!("{}", "To update, run:".cyan());
            println!();
            println!("  {}", "cargo install cyx --force".green().bold());
            println!();
        } else {
            println!(
                "{}",
                format!(
                    "✓ You are on the latest version (v{})",
                    update_info.current_version
                )
                .green()
            );
            println!();
        }

        Ok(())
    }
}

/// Auto-check for updates (called on startup)
pub fn auto_check_update() -> Result<()> {
    // Load metadata
    let mut metadata = UpdateMetadata::load()?;

    // Check if we should check now
    if !VersionChecker::should_check_now(metadata.last_check) {
        return Ok(()); // Skip check
    }

    // Perform background check
    let checker = VersionChecker::new()?;

    // Update last check timestamp
    metadata.update_last_check()?;

    // Check for updates (with short timeout)
    let update_info = match checker.check() {
        Ok(info) => info,
        Err(_) => return Ok(()), // Silently fail on network errors
    };

    // Show non-intrusive message if update available
    if update_info.needs_update {
        println!();
        println!(
            "{}",
            format!(
                "[*] Update available: v{} → v{} (run 'cyx update' to install)",
                update_info.current_version, update_info.latest_version
            )
            .cyan()
        );
        println!();
    }

    Ok(())
}
