use crate::cli::splash;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = splash()  )]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(required = false, short, long)]
    /// a single url or a file containing multiple urls
    pub url: Option<String>,

    #[arg(short, long, default_value = "hxnshots")]
    /// Output directory to save screenshots
    pub outdir: String,

    #[arg(short, long, default_value = "4")]
    /// Maximum number of parallel tabs
    pub tabs: Option<usize>,

    #[arg(short, long, default_value = "/usr/bin/google-chrome")]
    /// Browser binary path
    pub binary_path: String,

    #[arg(short = 'x', long, default_value = "1440")]
    /// Width of the website // URL
    pub width: Option<u32>,

    #[arg(short = 'y', long, default_value = "900")]
    /// Height of the website // URL
    pub height: Option<u32>,

    #[arg(long, default_value = "10")]
    /// Define timeout for urls
    pub timeout: u64,

    #[arg(long)]
    /// Silent mode (suppress all console output)
    pub silent: bool,

    #[arg(long)]
    /// Read urls from the standard in
    pub stdin: bool,
}
