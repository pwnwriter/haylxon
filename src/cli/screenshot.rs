use super::args::ScreenshotType;
use super::ascii::{BAR, RESET};
use crate::log;
use anyhow::Context;
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{browser::Browser, cdp::browser_protocol::page::CaptureScreenshotFormat};
use colored::Colorize;
use columns::Columns;
use regex::Regex;
use reqwest::StatusCode;
use std::sync::Arc;
use std::time::Duration;
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
) -> anyhow::Result<()> {
    let url_chunks: Vec<Vec<_>> = urls.chunks(tabs).map(ToOwned::to_owned).collect();
    let mut handles = Vec::with_capacity(url_chunks.len());

    for urls in url_chunks {
        let browser = Arc::clone(browser);
        let handle = tokio::spawn(async move {
            for url in urls {
                if let Err(error) = take_screenshot(
                    &browser,
                    url,
                    timeout,
                    delay,
                    silent,
                    full_page,
                    screenshot_type,
                    danger_accept_invalid_certs,
                )
                .await
                {
                    log::warn(error.to_string());
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

pub async fn take_screenshot(
    browser: &Browser,
    url: String,
    timeout: u64,
    delay: u64,
    verbose: bool,
    full_page: bool,
    screenshot_type: ScreenshotType,
    danger_accept_invalid_certs: bool,
) -> anyhow::Result<()> {
    let parsed_url = Url::parse(&url)?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(danger_accept_invalid_certs)
        .http1_ignore_invalid_headers_in_responses(danger_accept_invalid_certs)
        .trust_dns(danger_accept_invalid_certs)
        .build()?;
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
    let page = browser.new_page(parsed_url.clone()).await?;
    tokio::time::sleep(Duration::from_secs(delay)).await;
    page.save_screenshot(
        ScreenshotParams::builder()
            .format(screenshot_format)
            .full_page(full_page)
            .omit_background(false)
            .build(),
        filename,
    )
    .await?;

    if verbose {
        let response = time::timeout(
            Duration::from_secs(timeout),
            client.get(parsed_url.clone()).send(),
        )
        .await
        .context(format!("[-] Timed out URL = {url}"))??;

        match page.get_title().await {
            Ok(Some(title)) => show_info(url.clone(), title, response.status()),
            _ => {
                let title = "No title".to_string();
                show_info(url.clone(), title, response.status());
            }
        }
    }
    page.close().await?;

    Ok(())
}

fn show_info(url: String, title: String, status: StatusCode) {
    let info = Columns::from(vec![
        RESET.split('\n').collect::<Vec<_>>(),
        vec![
            &BAR.bold().blue(),
            &format!(" 🔗 URL = {}", url.red()),
            &format!(" 🏠 Title = {}", title.purple()),
            &format!(" 🔥 Status = {}", status).green(),
        ],
    ])
    .set_tabsize(0)
    .make_columns();

    println!("{info}");
}
