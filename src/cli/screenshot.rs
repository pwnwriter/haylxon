use super::args::{Cli, Input};
use super::ascii::{BAR, RESET};
use crate::log;
use anyhow::Context;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::{CaptureScreenshotFormat, CaptureScreenshotParams},
    handler::viewport::Viewport,
};
use colored::{Color, Colorize};
use columns::Columns;
use futures::StreamExt;
use reqwest::StatusCode;
use std::sync::Arc;
use std::{env, path::Path, time::Duration};
use tokio::{fs, task, time};
use url::Url;

pub async fn run(
    Cli {
        binary_path,
        input: Input { url, file_path },
        stdin,
        outdir,
        tabs,
        width,
        height,
        timeout,
        silent,
    }: Cli,
) -> anyhow::Result<()> {
    let browser = Path::new(&binary_path);
    if !browser.exists() {
        return Err(anyhow::Error::msg(format!(
            "Unable to locate browser binary {binary_path}"
        )));
    }

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(width, height)
            .chrome_executable(browser)
            .viewport(Viewport {
                width,
                height,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            })
            .build()
            .map_err(anyhow::Error::msg)?,
    )
    .await
    .context(format!("Error instantiating browser {binary_path}"))?;
    let browser = Arc::new(browser);

    task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let dump_dir = Path::new(&outdir);
    if !dump_dir.exists() {
        // TODO: Check error cases for reporting
        fs::create_dir(dump_dir).await?;
    }

    if stdin {
        env::set_current_dir(dump_dir)?;
        let urls = super::hxn_helper::read_urls_from_stdin()?;
        take_screenshot_in_bulk(&browser, urls, tabs, timeout, silent).await?;
    } else {
        match (url, file_path) {
            (None, Some(file_path)) => {
                let urls = super::hxn_helper::read_urls_from_file(file_path)?;
                env::set_current_dir(dump_dir)?;
                take_screenshot_in_bulk(&browser, urls, tabs, timeout, silent).await?;
            }
            (Some(url), None) => {
                env::set_current_dir(dump_dir)?;
                take_screenshot(&browser, url, timeout, silent).await?;
            }
            _ => unreachable!(),
        }
    }

    println!(
        "{}: {}",
        "Screenshots Taken and saved in directory"
            .bold()
            .color(Color::Green),
        outdir
    );

    Ok(())
}

async fn take_screenshot_in_bulk(
    browser: &Arc<Browser>,
    urls: Vec<String>,
    tabs: usize,
    timeout: u64,
    silent: bool,
) -> anyhow::Result<()> {
    let url_chunks: Vec<Vec<_>> = urls.chunks(tabs).map(ToOwned::to_owned).collect();
    let mut handles = Vec::with_capacity(url_chunks.len());

    for urls in url_chunks {
        let browser = Arc::clone(browser);
        let handle = tokio::spawn(async move {
            for url in urls {
                if let Err(error) = take_screenshot(&browser, url, timeout, silent).await {
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

async fn take_screenshot(
    browser: &Browser,
    url: String,
    timeout: u64,
    silent: bool,
) -> anyhow::Result<()> {
    let parsed_url = Url::parse(&url)?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .http1_ignore_invalid_headers_in_responses(true)
        .trust_dns(true)
        .build()?;

    let response = time::timeout(
        Duration::from_secs(timeout),
        client.get(parsed_url.clone()).send(),
    )
    .await
    .context(format!("[-] Timed out URL = {url}"))??;

    let filename = format!("{}.png", url.replace("://", "-").replace('/', "_"));
    let page = browser.new_page(parsed_url).await?;
    page.save_screenshot(
        CaptureScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .build(),
        filename,
    )
    .await?;

    if !silent {
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
