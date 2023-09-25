mod cli;
mod log;

use clap::Parser;
use cli::{args, screenshot};

#[tokio::main]
async fn main() {
    let cli = args::Cli::parse();
    if let Err(error) = screenshot::run(cli).await {
        log::error(error.to_string())
    }
}
