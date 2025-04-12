// src/main.rs

mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{get, push};
use config::Config;

#[tokio::main]
async fn main() {

    let config = Config::load().expect("Failed to load config");

    let cli = Cli::parse();


    match cli.command {
        Commands::Get { problem } => {
            if let Err(e) = get::handle(problem, &config).await {
                eprintln!("Failed to fetch problem: {}", e);
            }
        }
        Commands::Push { file } => {
            push::handle(file);
        }
    }
}

