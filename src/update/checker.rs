use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use semver::Version;
use serde::Deserialize;

pub struct VersionChecker {
    current_version: Version,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: Version,
    pub latest_version: Version,
    pub needs_update: bool,
}

impl VersionChecker {
    pub fn new() -> Result<Self> {
        let current_version = Version::parse(env!("CARGO_PKG_VERSION"))?;

        let client = reqwest::blocking::Client::builder()
            .user_agent(format!("cyx/{}", current_version))
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            current_version,
            client,
        })
    }

    /// Check for updates on crates.io
    pub fn check(&self) -> Result<UpdateInfo> {
        #[derive(Deserialize)]
        struct CratesResponse {
            #[serde(rename = "crate")]
            crate_info: CrateInfo,
        }

        #[derive(Deserialize)]
        struct CrateInfo {
            max_version: String,
        }

        let url = "https://crates.io/api/v1/crates/cyx";
        let response = self.client.get(url).send()?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to check crates.io: HTTP {}", response.status());
        }

        let crates_data: CratesResponse = response.json()?;
        let latest_version = Version::parse(&crates_data.crate_info.max_version)?;

        Ok(UpdateInfo {
            current_version: self.current_version.clone(),
            latest_version: latest_version.clone(),
            needs_update: latest_version > self.current_version,
        })
    }

    /// Check if enough time has passed since last check
    pub fn should_check_now(last_check: Option<DateTime<Utc>>) -> bool {
        match last_check {
            None => true,
            Some(last) => {
                let now = Utc::now();
                let elapsed = now.signed_duration_since(last);
                elapsed > Duration::hours(24)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_check_now() {
        // No previous check
        assert!(VersionChecker::should_check_now(None));

        // Recent check (1 hour ago)
        let recent = Utc::now() - Duration::hours(1);
        assert!(!VersionChecker::should_check_now(Some(recent)));

        // Old check (25 hours ago)
        let old = Utc::now() - Duration::hours(25);
        assert!(VersionChecker::should_check_now(Some(old)));
    }
}
