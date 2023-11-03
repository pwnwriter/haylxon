use super::ascii;
use clap::{Args, Parser};

#[derive(Debug, Args)]
#[group(required = false, multiple = false, conflicts_with = "stdin")]
pub struct Input {
    /// Website URL
    #[arg(short, long)]
    pub url: Option<String>,

    /// Path of the file containing URLs
    #[arg(short, long)]
    pub file_path: Option<String>,
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = ascii::splash(),
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Browser binary path
    #[arg(short, long, default_value = "/usr/bin/chrome")]
    pub binary_path: String,

    /// Read urls from the standard input
    #[arg(long)]
    pub stdin: bool,

    #[command(flatten)]
    pub input: Input,

    /// Output directory to save screenshots
    #[arg(short, long, default_value = "hxnshots")]
    pub outdir: String,

    /// Maximum number of parallel tabs
    #[arg(short, long, default_value = "4")]
    pub tabs: usize,

    /// Width of the website // URL
    #[arg(short = 'x', long, default_value = "1440")]
    pub width: u32,

    /// Height of the website // URL
    #[arg(short = 'y', long, default_value = "900")]
    pub height: u32,

    /// Define timeout for urls
    #[arg(long, default_value = "10")]
    pub timeout: u64,

    /// verbose mode to show status code,title and more
    #[arg(long)]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {
    use clap::error::ErrorKind;

    use super::*;

    #[test]
    fn test_no_input_urls() {
        let args = Cli::try_parse_from(["-b my_browser"]);
        assert!(args.is_err());
        assert_eq!(
            args.unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }

    #[test]
    fn test_stdin_source_mutual_exclusion() {
        let args = Cli::try_parse_from([
            "-b my_browser",
            "--stdin",
            "-u https://example.com",
            "-f /my/file",
        ]);
        assert!(args.is_err());
        assert_eq!(args.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_url_mutual_exclusion_with_file_path() {
        let args = Cli::try_parse_from(["-b my_browser", "-u https://example.com", "-f /my/file"]);
        assert!(args.is_err());
        assert_eq!(args.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_file_path_as_source() {
        let args = Cli::try_parse_from(["-b my_browser", "-f /my/file"]);
        assert!(args.is_ok());
    }
}
