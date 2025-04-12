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
        Commands::Push { problem_id, file } => {
            if let Err(e) = push::handle(problem_id, file, &config).await {
                eprintln!("Failed to push problem: {}", e);
            }
        }
    }
}

