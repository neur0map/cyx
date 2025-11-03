use anyhow::{Context, Result};
use std::process::Command;

pub struct OllamaInstaller;

impl OllamaInstaller {
    pub fn install() -> Result<()> {
        let os = std::env::consts::OS;

        match os {
            "macos" | "linux" => {
                Self::install_unix()?;
            }
            "windows" => {
                Self::install_windows()?;
            }
            _ => anyhow::bail!("Unsupported operating system: {}", os),
        }

        Ok(())
    }

    fn install_unix() -> Result<()> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://ollama.com/install.sh | sh")
            .output()
            .context("Failed to execute install script")?;

        if !output.status.success() {
            anyhow::bail!(
                "Installation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    fn install_windows() -> Result<()> {
        println!("Please download and run the Ollama installer from:");
        println!("https://ollama.com/download/windows");
        println!("\nPress Enter when installation is complete...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(())
    }

    pub fn start_service() -> Result<()> {
        let os = std::env::consts::OS;

        match os {
            "macos" => {
                // Ollama auto-starts on macOS
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            "linux" => {
                let _ = Command::new("systemctl").args(["start", "ollama"]).output();
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            "windows" => {
                // Ollama service should auto-start after install
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            _ => {}
        }

        Ok(())
    }

    pub fn check_available() -> bool {
        // Try connecting to Ollama API
        if let Ok(client) = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
        {
            if let Ok(response) = client.get("http://localhost:11434/api/tags").send() {
                return response.status().is_success();
            }
        }

        false
    }
}
