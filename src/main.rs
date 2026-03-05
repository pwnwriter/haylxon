use cli::{args, exec};
mod cli;

use clap::{CommandFactory, Parser};

fn available_options() -> String {
    let mut buf = Vec::new();
    args::Cli::command().write_help(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().build())
    }))?;

    let cli = match args::Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // --help and --version: let clap handle normally
            if matches!(
                e.kind(),
                clap::error::ErrorKind::DisplayHelp
                    | clap::error::ErrorKind::DisplayVersion
            ) {
                e.exit();
            }
            eprintln!("{}", cli::ascii::splash());
            // Extract the raw message from clap (strip "error: " prefix)
            let raw = e.render().to_string();
            let msg = raw
                .lines()
                .next()
                .and_then(|l| l.strip_prefix("error: "))
                .unwrap_or(&raw);
            let help = available_options();
            return Err(miette::miette!(
                help = help,
                "{msg}"
            ));
        }
    };

    if let Err(e) = cli.validate_input() {
        eprintln!("{}", cli::ascii::splash());
        return Err(e);
    }

    exec::run(cli).await
}
