use colored::Colorize;
use reqwest::StatusCode;
use std::time::Duration;
use tabled::{builder::Builder, settings::Style};

#[derive(serde::Serialize)]
pub struct ScreenshotResult {
    pub url: String,
    pub title: String,
    pub status: u16,
    pub filename: String,
    pub elapsed_secs: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ScreenshotError {
    pub url: String,
    pub error: String,
}

pub fn print_config_table(
    url_count: usize,
    source: &str,
    outdir: &str,
    tabs: usize,
    user_agent: &Option<String>,
    ua_pool_size: usize,
) {
    let mut builder = Builder::default();
    builder.push_record(["URLs", &url_count.to_string()]);
    builder.push_record(["Source", source]);
    builder.push_record(["Output", outdir]);
    builder.push_record(["Tabs", &tabs.to_string()]);
    if let Some(ua) = user_agent {
        builder.push_record(["User-Agent", ua]);
    } else if ua_pool_size > 0 {
        builder.push_record([
            "User-Agent",
            &format!("random-per-url ({ua_pool_size} agents)"),
        ]);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{table}");
}

pub fn show_info(
    url: &str,
    title: &str,
    status: StatusCode,
    filename: &str,
    elapsed: Duration,
    user_agent: &Option<String>,
) {
    let elapsed_secs = elapsed.as_secs_f64();
    let mut builder = Builder::default();
    builder.push_record(["URL", url]);
    builder.push_record(["Title", title]);
    builder.push_record(["Status", &format!("{status}")]);
    builder.push_record(["Saved as", filename]);
    builder.push_record(["Time", &format!("{elapsed_secs:.2}s")]);
    if let Some(ua) = user_agent {
        builder.push_record(["User-Agent", ua]);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{table}");
}

pub fn show_line(url: &str, status: StatusCode, filename: &str, elapsed: Duration) {
    let elapsed_secs = elapsed.as_secs_f64();
    println!(
        "{} {} {} {} {}",
        "✓".green(),
        format!("{}", status.as_u16()).green(),
        url,
        format!("→ {}", filename).cyan(),
        format!("{:.2}s", elapsed_secs).yellow()
    );
}

pub fn show_line_error(error: &str) {
    println!("{} {} {}", "✗".red(), "ERR".red(), error,);
}
