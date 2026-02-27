use super::ascii;
use clap::{Args, Parser, ValueEnum};

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
    #[command(flatten)]
    pub input: Input,

    /// Browser binary path
    #[arg(short, long, default_value = "/usr/bin/chrome")]
    pub binary_path: String,

    /// Output directory to save screenshots
    #[arg(short, long, default_value = "hxnshots")]
    pub outdir: String,

    /// Maximum number of parallel tabs
    #[arg(short, long, default_value = "4")]
    pub tabs: usize,

    /// Width of the website; URL
    #[arg(short = 'x', long, default_value = "1440")]
    pub width: u32,

    /// Height of the website; URL
    #[arg(short = 'y', long, default_value = "900")]
    pub height: u32,

    /// Define timeout for urls
    #[arg(long, default_value = "100")]
    pub timeout: u64,

    /// Define delay for client side loading
    #[arg(long, default_value = "0")]
    pub delay: u64,

    /// Provide ports as a range (x..y) or absolute values (x,y,z).
    #[arg(long, num_args(0..=1000), required=false)]
    pub ports: Option<String>,

    /// Read urls from the standard input
    #[arg(long)]
    pub stdin: bool,

    /// Silent mode, suppress extra output (useful for automation)
    #[arg(short, long)]
    pub silent: bool,

    /// Take fullpage screenshot
    #[arg(long, default_value = "false")]
    pub fullpage: bool,

    /// Define your image type
    #[arg(long, default_value = "png")]
    pub screenshot_type: ScreenshotType,

    /// Accept invalid certs, trust dns
    #[arg(long)]
    pub accept_invalid_certs: bool,

    /// Run arbiraty javascript
    #[arg(long)]
    pub javascript: Option<String>,

    /// Custom user-agent string
    #[arg(long, conflicts_with = "random_user_agent")]
    pub user_agent: Option<String>,

    /// Use a random user-agent from a built-in list
    #[arg(long, conflicts_with = "user_agent")]
    pub random_user_agent: bool,

    /// Proxy URL (e.g., http://127.0.0.1:8080, socks5://127.0.0.1:9050)
    #[arg(long)]
    pub proxy: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
pub enum ScreenshotType {
    /// format png
    Png,
    /// format Jpeg
    Jpeg,
    /// format Webg
    Webg,
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
