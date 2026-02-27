use cli::{args, exec};
mod cli;

use clap::Parser;

#[tokio::main]
async fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().build())
    }))?;

    let cli = args::Cli::parse();
    if let Err(e) = cli.validate_input() {
        eprintln!("{}", cli::ascii::splash());
        return Err(e);
    }
    exec::run(cli).await
}
