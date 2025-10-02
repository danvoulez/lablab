use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: Option<String>,
    pub ledger_path: PathBuf,
    pub manuscripts_path: PathBuf,
    pub log_level: String,
    pub worker_id: Option<String>,
    pub scheduler_interval_seconds: u64,
    pub job_timeout_minutes: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from config files, environment variables, etc.
        let ledger_path = std::env::var("LEDGER_PATH")
            .unwrap_or_else(|_| "ledger/spans/discovery.ndjson".to_string())
            .into();

        let manuscripts_path = std::env::var("MANUSCRIPTS_PATH")
            .unwrap_or_else(|_| "manuscripts".to_string())
            .into();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL").ok(),
            ledger_path,
            manuscripts_path,
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            worker_id: std::env::var("WORKER_ID").ok(),
            scheduler_interval_seconds: std::env::var("SCHEDULER_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            job_timeout_minutes: std::env::var("JOB_TIMEOUT_MINUTES")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
        })
    }

    pub fn validate(&self) -> Result<()> {
        if !self.ledger_path.exists() {
            return Err(Error::Config(format!(
                "Ledger path does not exist: {}",
                self.ledger_path.display()
            )));
        }

        if !self.manuscripts_path.exists() {
            std::fs::create_dir_all(&self.manuscripts_path)?;
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: None,
            ledger_path: "ledger/spans/discovery.ndjson".into(),
            manuscripts_path: "manuscripts".into(),
            log_level: "info".to_string(),
            worker_id: None,
            scheduler_interval_seconds: 30,
            job_timeout_minutes: 60,
        }
    }
}
