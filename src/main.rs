use cli::{args, exec};
mod cli;
mod log;

use clap::Parser;

#[tokio::main]
async fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().build())
    }))?;

    let cli = args::Cli::parse();
    exec::run(cli).await
}
