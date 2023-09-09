use crate::cli::{args, screenshot::run};
use clap::Parser;

mod cli;
mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = args::Cli::parse();
    run(
        cli.url,
        Some(cli.outdir),
        cli.tabs,
        cli.binary_path,
        cli.width,
        cli.height,
        cli.timeout,
        cli.silent,
        cli.stdin,
    )
    .await?;

    Ok(())
}
