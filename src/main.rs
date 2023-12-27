use cli::{args, exec};
mod cli;
mod log;

use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = args::Cli::parse();
    if let Err(error) = exec::run(cli).await {
        log::error(error.to_string())
    }
}
