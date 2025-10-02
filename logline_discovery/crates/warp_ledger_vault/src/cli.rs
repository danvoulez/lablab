use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Show basic log status for the default ledger path
    Status,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Some(Cmd::Status) => {
            let log = crate::runtime::SpanLog::new("ledger/spans/discovery.ndjson")?;
            println!("entries: {}", log.count()?);
        }
        None => println!("ledger_vault: no subcommand"),
    }
    Ok(())
}
