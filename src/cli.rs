use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "probman")]
#[command(about = "The Competitive Programming tool in your terminal!")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Get {
        #[arg(help = "The problem ID")]
        problem: String,
    },
    Push {
        #[arg (help = "Problem UUID")]
        problem_id: String,

        #[arg(help = "Path to the solution file")]
        file: String,
    },
}
