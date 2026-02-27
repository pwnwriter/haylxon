use cli::{args, exec};
mod cli;

use clap::Parser;

const AVAILABLE_OPTIONS: &str = "\
Available options:
  -u, --url <URL>              Website URL
  -f, --file-path <FILE>       Path of the file containing URLs
      --stdin                   Read urls from standard input
  -b, --binary-path <PATH>     Browser binary path
  -o, --outdir <DIR>           Output directory to save screenshots
  -t, --tabs <N>               Maximum number of parallel tabs
      --timeout <SECS>          Define timeout for urls
      --delay <SECS>            Define delay for client side loading
      --ports <PORTS>           Ports as a range (x..y) or absolute (x,y,z)
      --user-agent <UA>         \"random\", \"random-per-url\", path to UA file, or custom string
      --proxy <URL>             Proxy URL (http, socks5)
      --fullpage                Take fullpage screenshot
      --screenshot-type <TYPE>  png, jpeg, or webg
      --javascript <JS>         Run arbitrary javascript
      --json                    Output results as NDJSON
      --accept-invalid-certs    Accept invalid certs, trust dns
  -s, --silent                  Suppress extra output

Run `hxn --help` for full details.";

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
            return Err(miette::miette!(
                help = AVAILABLE_OPTIONS,
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
