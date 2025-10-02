use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Demo,
}

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Some(Cmd::Demo) => {
            let rt = crate::runtime::Runtime::new();
            let spans = rt.fabricate_demo()?;
            println!("{}", serde_json::to_string_pretty(&spans)?);
        }
        None => println!("span_fabricator: no subcommand"),
    }
    Ok(())
}
