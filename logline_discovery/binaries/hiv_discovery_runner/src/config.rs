use std::env;
use std::path::PathBuf;

use anyhow::Result;
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub ledger_path: PathBuf,
    pub database_url: Option<String>,
}

impl RunnerConfig {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        let ledger_path = env::var("LEDGER_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("ledger/spans/discovery.ndjson"));

        let database_url = env::var("DATABASE_URL").ok();

        Ok(Self {
            ledger_path,
            database_url,
        })
    }
}
