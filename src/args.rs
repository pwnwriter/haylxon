use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "None")]
pub struct Cli {
    #[arg(short, long)]
    /// Website URL/filename of file containing URLs
    pub url: String,

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

    #[arg(short, long)]
    /// Silent mode (suppress all console output)
    pub silent: bool,
}
