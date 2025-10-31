use colored::Colorize;

pub struct Display;

impl Display {
    /// Display a banner/header
    pub fn banner() {
        let banner = r#"
  =========================================
     CYX - Cybersecurity Companion
     Your hacking toolkit in command
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
        println!("{} {} ({})", "Provider:".dimmed(), provider_name.cyan(), model_name.dimmed());

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
}
