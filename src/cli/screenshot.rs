use super::args::ScreenshotType;
use crate::log;
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{browser::Browser, cdp::browser_protocol::page::CaptureScreenshotFormat};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{Context, IntoDiagnostic};
use regex::Regex;
use reqwest::StatusCode;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tabled::{builder::Builder, settings::Style};
use tokio::time;
use url::Url;

pub async fn take_screenshot_in_bulk(
    browser: &Arc<Browser>,
    urls: Vec<String>,
    tabs: usize,
    timeout: u64,
    delay: u64,
    silent: bool,
    full_page: bool,
    screenshot_type: ScreenshotType,
    danger_accept_invalid_certs: bool,
    javascript: Option<String>,
    is_bulk: bool,
    user_agent: Option<String>,
) -> miette::Result<()> {
    let total = urls.len() as u64;
    let pb = if silent {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("=>-"),
        );
        pb
    };
    let pb = Arc::new(pb);

    let url_chunks: Vec<Vec<_>> = urls.chunks(tabs).map(ToOwned::to_owned).collect();
    let mut handles = Vec::with_capacity(url_chunks.len());

    for urls in url_chunks {
        let browser = Arc::clone(browser);
        let js = javascript.clone();
        let pb = Arc::clone(&pb);
        let ua = user_agent.clone();
        let handle = tokio::spawn(async move {
            for url in urls {
                pb.set_message(url.clone());
                if let Err(error) = take_screenshot(
                    &browser,
                    url,
                    timeout,
                    delay,
                    silent,
                    full_page,
                    screenshot_type,
                    danger_accept_invalid_certs,
                    js.clone(),
                    &pb,
                    is_bulk,
                    ua.clone(),
                )
                .await
                {
                    if is_bulk && !silent {
                        pb.suspend(|| {
                            show_line_error(&error.to_string());
                        });
                    } else {
                        pb.suspend(|| log::warn(error.to_string()));
                    }
                }
                pb.inc(1);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.into_diagnostic()?;
    }

    pb.finish_and_clear();

    Ok(())
}

pub async fn take_screenshot(
    browser: &Browser,
    url: String,
    timeout: u64,
    delay: u64,
    silent: bool,
    full_page: bool,
    screenshot_type: ScreenshotType,
    danger_accept_invalid_certs: bool,
    javascript: Option<String>,
    pb: &ProgressBar,
    is_bulk: bool,
    user_agent: Option<String>,
) -> miette::Result<()> {
    let start = Instant::now();
    let parsed_url = Url::parse(&url)
        .into_diagnostic()
        .wrap_err_with(|| format!("Invalid URL: {url}"))?;
    let mut client_builder = reqwest::Client::builder()
        .danger_accept_invalid_certs(danger_accept_invalid_certs)
        .http1_ignore_invalid_headers_in_responses(danger_accept_invalid_certs);
    if let Some(ref ua) = user_agent {
        client_builder = client_builder.user_agent(ua);
    }
    let client = client_builder.build().into_diagnostic()?;
    let re = Regex::new(r"[<>?.~!@#$%^&*\\/|;:']").unwrap();
    let regurl = re.replace_all(&url, "").to_string();

    let filename = format!(
        "{}.png",
        regurl
            .replace("://", "-")
            .replace('/', "_")
            .replace(':', "-")
    );
    let screenshot_format = match screenshot_type {
        ScreenshotType::Png => CaptureScreenshotFormat::Png,
        ScreenshotType::Jpeg => CaptureScreenshotFormat::Jpeg,
        ScreenshotType::Webg => CaptureScreenshotFormat::Webp,
    };
    let page = browser
        .new_page(parsed_url.clone())
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to open page: {url}"))?;
    tokio::time::sleep(Duration::from_secs(delay)).await;

    // Evaluate JavaScript if provided
    if let Some(js) = javascript {
        let result = page.evaluate(js.as_str()).await;
        match result {
            Ok(_) => pb.suspend(|| {
                log::info(
                    "JavaScript executed successfully".to_string(),
                    colored::Color::Magenta,
                )
            }),
            Err(e) => pb.suspend(|| log::warn(format!("JavaScript execution failed: {:?}", e))),
        }
    }

    page.save_screenshot(
        ScreenshotParams::builder()
            .format(screenshot_format)
            .full_page(full_page)
            .omit_background(false)
            .build(),
        &filename,
    )
    .await
    .into_diagnostic()
    .wrap_err_with(|| format!("Failed to save screenshot for: {url}"))?;

    if !silent {
        let elapsed = start.elapsed();
        let response = time::timeout(
            Duration::from_secs(timeout),
            client.get(parsed_url.clone()).send(),
        )
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Timed out URL = {url}"))?
        .into_diagnostic()?;

        let title = match page.get_title().await {
            Ok(Some(t)) => t,
            _ => "No title".to_string(),
        };

        if is_bulk {
            pb.suspend(|| show_line(&url, response.status(), &filename, elapsed));
        } else {
            pb.suspend(|| {
                show_info(
                    &url,
                    &title,
                    response.status(),
                    &filename,
                    elapsed,
                    &user_agent,
                )
            });
        }
    }

    page.close().await.into_diagnostic()?;

    Ok(())
}

fn show_info(
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

fn show_line(url: &str, status: StatusCode, filename: &str, elapsed: Duration) {
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

fn show_line_error(error: &str) {
    println!("{} {} {}", "✗".red(), "ERR".red(), error,);
}
