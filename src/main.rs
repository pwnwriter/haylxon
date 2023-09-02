mod cli;
mod log;
use cli::args;
mod colors;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = args::Cli::parse();
    crate::cli::screenshot::run(
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
