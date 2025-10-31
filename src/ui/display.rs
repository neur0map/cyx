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

    /// Display a loading spinner message
    pub fn loading(message: &str) {
        println!("{} {}", "[~]".cyan().bold(), message.dimmed());
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

    /// Display source information with extracted links
    pub fn sources_with_links(
        provider_name: &str,
        model_name: &str,
        searched: bool,
        links: &[String],
    ) {
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

        // Display extracted links if any
        if !links.is_empty() {
            println!("\n{}", "Links:".dimmed());
            for link in links {
                println!("  {} {}", "-".dimmed(), link);
            }
        }

        println!();
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

    /// Display content in a simple box (for streaming)
    pub fn stream_box_section(title: &str, content: &str) {
        let width = 58;
        println!();
        println!("{}", format!("╭─── {} {}", title, "─".repeat(width - title.len() - 6)).cyan());

        // Detect code blocks and color them yellow
        let mut in_code_block = false;
        for line in content.lines() {
            if line.trim().starts_with("```") {
                in_code_block = !in_code_block;
                println!("{} {}", "│".cyan(), line.dimmed());
            } else if in_code_block {
                println!("{} {}", "│".cyan(), line.yellow());
            } else {
                println!("{} {}", "│".cyan(), line);
            }
        }

        println!("{}", format!("╰{}", "─".repeat(width)).cyan());
    }

    /// Print the box header for live streaming
    pub fn stream_box_header(title: &str) {
        let width = 58;
        println!();
        println!("{}", format!("╭─── {} {}", title, "─".repeat(width - title.len() - 6)).cyan());
    }

    /// Print the box footer for live streaming
    pub fn stream_box_footer() {
        let width = 58;
        println!("{}", format!("╰{}", "─".repeat(width)).cyan());
    }

    /// Print a line with smooth character-by-character animation
    pub fn print_line_animated(line: &str, is_code_fence: bool, is_code: bool) {
        use std::io::{self, Write};
        use std::thread;
        use std::time::Duration;

        // Determine color based on type
        for ch in line.chars() {
            if is_code_fence {
                print!("{}", ch.to_string().dimmed());
            } else if is_code {
                print!("{}", ch.to_string().yellow());
            } else {
                print!("{}", ch);
            }
            io::stdout().flush().unwrap();

            // Add tiny delay for smooth typewriter effect (only for non-whitespace)
            if !ch.is_whitespace() {
                thread::sleep(Duration::from_micros(100));
            }
        }
        println!();
    }

    /// Print sources header with smooth animation
    pub fn print_sources_header(provider_name: &str, model_name: &str, searches_web: bool) {
        use std::io::{self, Write};
        use std::thread;
        use std::time::Duration;

        // Print each part with smooth effect
        print!("{}", "[*] SOURCES".cyan().bold());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
        println!();

        print!("{}", "───────────────────────────────────────".cyan());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(30));
        println!();

        print!("{} ", "Provider:".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));

        print!("{}", provider_name.cyan());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));

        print!(" ({})", model_name.dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));
        println!();

        print!("{} ", "Search:".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));

        let search_status = if searches_web {
            "Yes (performed web search)".green()
        } else {
            "No (knowledge base only)".yellow()
        };
        print!("{}", search_status);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));
        println!();
        println!();

        print!("{}", "Links:".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));
        println!();
    }

    /// Print a link with smooth character-by-character animation
    pub fn print_link_animated(link: &str) {
        use std::io::{self, Write};
        use std::thread;
        use std::time::Duration;

        print!("  {} ", "-".dimmed());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(10));

        // Print link with smooth animation
        for ch in link.chars() {
            print!("{}", ch);
            io::stdout().flush().unwrap();

            // Faster animation for URLs
            if !ch.is_whitespace() {
                thread::sleep(Duration::from_micros(800));
            }
        }
        println!();
    }
}
