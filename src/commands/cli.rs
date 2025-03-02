use clap::Parser;

#[derive(Parser)]
#[command(name = "gch")]
#[command(about = "A Git-like CLI tool", version = "0.1")]
pub struct Cli {
    #[command(subcommand)]
    pub command: super::Commands,
}
