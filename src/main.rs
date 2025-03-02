mod commands;
mod settings;
mod utils;
use std::process;

use clap::Parser;
use utils::{config, git};

fn main() {
    if git::is_git_repo() {
        let app_settings = settings::Settings::new("gch.yml");
        let cli = commands::Cli::parse();

        let config = config::read(&app_settings).unwrap_or_else(|e| {
            eprintln!("Error reading config file: {}", e);
            process::exit(1);
        });

        match &cli.command {
            commands::Commands::Commit(args) => {
                commands::commit(&args, &config);
            }
        }
    } else {
        eprintln!("Not a Git repository");
    }
}
