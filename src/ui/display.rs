use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct Display;

impl Display {
    /// Display a banner/header
    pub fn banner() {
        let banner = r#"
  =========================================
     CYX - Cybersecurity Companion
     Your hacking battle buddy
  =========================================
        "#;
        println!("{}", banner.cyan().bold());
    }

    /// Display a section header
    pub fn section(title: &str) {
        println!("\n{}", format!("--- {} ---", title).cyan().bold());
    }

    /// Display a success message
    pub fn success(message: &str) {
        println!("{} {}", "[+]".green().bold(), message.green());
    }

    /// Display an error message
    pub fn error(message: &str) {
        eprintln!("{} {}", "[!]".red().bold(), message.red());
    }

    /// Display an info message
    pub fn info(message: &str) {
        println!("{} {}", "[*]".cyan(), message);
    }

    /// Display a warning message
    pub fn warning(message: &str) {
        println!("{} {}", "[!]".yellow().bold(), message.yellow());
    }

    /// Display formatted LLM response with code highlighting
    pub fn llm_response(content: &str) {
        println!("\n{}", "--- Response ---".cyan().bold());

        // Simple code block detection and highlighting
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;

        for line in lines {
            if line.trim().starts_with("```") {
                in_code_block = !in_code_block;
                println!("{}", line.dimmed());
            } else if in_code_block {
                println!("{}", line.yellow());
            } else {
                println!("{}", line);
            }
        }
        println!();
    }

    /// Display a loading spinner message
    pub fn loading(message: &str) {
        println!("{} {}", "[~]".cyan().bold(), message.dimmed());
    }

    /// Display help text for interactive mode
    pub fn interactive_help() {
        println!("\n{}", "Available Commands:".cyan().bold());
        println!("  {}  - Exit the session", "/exit".yellow());
        println!("  {}  - Clear conversation history", "/clear".yellow());
        println!("  {}  - Show this help message", "/help".yellow());
        println!("\n{}", "Just type your question to start!".dimmed());
    }

    /// Display prompt for interactive mode
    pub fn prompt() {
        print!("{} ", "cyx>".green().bold());
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    /// Display source information for LLM response
    pub fn sources(provider_name: &str, model_name: &str, searched: bool) {
        println!("\n{}", "[*] SOURCES".cyan().bold());
        println!("{}", "───────────────────────────────────────".cyan());
        println!(
            "{} {} ({})",
            "Provider:".dimmed(),
            provider_name.cyan(),
            model_name.dimmed()
        );

        let search_status = if searched {
            "Yes (performed web search)".green()
        } else {
            "No (knowledge base only)".yellow()
        };
        println!("{} {}", "Search:".dimmed(), search_status);
        println!();
    }

    /// Display learn mode separator
    pub fn learn_mode_separator() {
        println!("\n{}", "[LEARN MODE] Detailed Breakdown".yellow().bold());
        println!("{}", "───────────────────────────────────────".yellow());
    }

    /// Display command result header
    pub fn command_result_header() {
        println!("\n{}", "[*] COMMAND RESULT".cyan().bold());
        println!("{}", "───────────────────────────────────────".cyan());
    }

    /// Create a streaming progress bar
    pub fn create_progress_bar(message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg:.dimmed} [{elapsed:.bold}]")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    /// Display content in a bordered box
    pub fn boxed_section(title: &str, content: &str, color: &str) {
        let width = 60;
        let title_text = format!(" {} ", title);
        let padding = (width - title_text.len() - 2).max(0);
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;

        // Top border with title
        let top = format!(
            "╭{}{}{}╮",
            "─".repeat(left_pad),
            title_text,
            "─".repeat(right_pad)
        );

        // Prepare content lines
        let content_lines: Vec<&str> = content.lines().collect();

        // Print box
        match color {
            "cyan" => println!("\n{}", top.cyan()),
            "green" => println!("\n{}", top.green()),
            "yellow" => println!("\n{}", top.yellow()),
            _ => println!("\n{}", top),
        }

        for line in content_lines {
            match color {
                "cyan" => print!("{}", "│".cyan()),
                "green" => print!("{}", "│".green()),
                "yellow" => print!("{}", "│".yellow()),
                _ => print!("│"),
            }
            print!(" {} ", line);
            match color {
                "cyan" => println!("{}", "│".cyan()),
                "green" => println!("{}", "│".green()),
                "yellow" => println!("{}", "│".yellow()),
                _ => println!("│"),
            }
        }

        // Bottom border
        let bottom = format!("╰{}╯", "─".repeat(width));
        match color {
            "cyan" => println!("{}", bottom.cyan()),
            "green" => println!("{}", bottom.green()),
            "yellow" => println!("{}", bottom.yellow()),
            _ => println!("{}", bottom),
        }
    }

    /// Display content in a simple box (for streaming)
    pub fn stream_box_section(title: &str, content: &str) {
        let width = 58;
        println!();
        println!("{}", format!("╭─── {} {}", title, "─".repeat(width - title.len() - 6)).cyan());

        for line in content.lines() {
            println!("{} {}", "│".cyan(), line);
        }

        println!("{}", format!("╰{}", "─".repeat(width)).cyan());
    }
}
