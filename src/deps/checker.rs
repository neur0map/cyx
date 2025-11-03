use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

pub struct DependencyChecker {
    checks: Vec<Box<dyn DependencyCheckImpl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum DependencyStatus {
    Installed { version: String },
    NotInstalled,
    WrongVersion { current: String, required: String },
}

pub trait DependencyCheckImpl: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self) -> Result<DependencyStatus>;
    fn install_instructions(&self) -> String;
}

impl Clone for Box<dyn DependencyCheckImpl> {
    fn clone(&self) -> Self {
        panic!("Cloning Box<dyn DependencyCheckImpl> is not supported")
    }
}

impl DependencyChecker {
    pub fn new() -> Result<Self> {
        let checks: Vec<Box<dyn DependencyCheckImpl>> =
            vec![Box::new(SqliteCheck), Box::new(OllamaCheck)];

        Ok(Self { checks })
    }

    pub fn check_all(&self) -> Result<Vec<DepCheckResult>> {
        let mut results = Vec::new();

        for check in &self.checks {
            let status = check.check().unwrap_or(DependencyStatus::NotInstalled);
            results.push(DepCheckResult {
                name: check.name().to_string(),
                status,
                instructions: check.install_instructions(),
            });
        }

        Ok(results)
    }

    pub fn check_dependency(&self, name: &str) -> Result<DependencyStatus> {
        for check in &self.checks {
            if check.name() == name {
                return check.check();
            }
        }
        anyhow::bail!("Unknown dependency: {}", name)
    }
}

#[derive(Debug, Clone)]
pub struct DepCheckResult {
    pub name: String,
    pub status: DependencyStatus,
    pub instructions: String,
}

// SQLite Check
struct SqliteCheck;

impl DependencyCheckImpl for SqliteCheck {
    fn name(&self) -> &str {
        "SQLite"
    }

    fn check(&self) -> Result<DependencyStatus> {
        // SQLite is bundled via rusqlite, so it's always available
        Ok(DependencyStatus::Installed {
            version: "bundled".to_string(),
        })
    }

    fn install_instructions(&self) -> String {
        "SQLite is bundled with Cyx (no installation needed)".to_string()
    }
}

// Ollama Check
struct OllamaCheck;

impl DependencyCheckImpl for OllamaCheck {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn check(&self) -> Result<DependencyStatus> {
        // Try to get version from command line
        let version_output = Command::new("ollama").arg("--version").output();

        if let Ok(output) = version_output {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                let version = version.trim().replace("ollama version ", "");
                return Ok(DependencyStatus::Installed { version });
            }
        }

        // Try to check if service is running
        if let Ok(client) = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
        {
            if let Ok(response) = client.get("http://localhost:11434/api/tags").send() {
                if response.status().is_success() {
                    return Ok(DependencyStatus::Installed {
                        version: "running".to_string(),
                    });
                }
            }
        }

        Ok(DependencyStatus::NotInstalled)
    }

    fn install_instructions(&self) -> String {
        let os = std::env::consts::OS;
        match os {
            "macos" | "linux" => {
                "Install via: curl -fsSL https://ollama.com/install.sh | sh".to_string()
            }
            "windows" => "Download installer from: https://ollama.com/download/windows".to_string(),
            _ => "Visit https://ollama.com for installation instructions".to_string(),
        }
    }
}
