use super::args::ScreenshotType;
use super::output::{self, ScreenshotError, ScreenshotResult};
use super::user_agent;
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{browser::Browser, cdp::browser_protocol::page::CaptureScreenshotFormat};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{Context, IntoDiagnostic};
use regex::Regex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
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
    proxy: Option<String>,
    json: bool,
    ua_pool: Arc<Vec<String>>,
) -> miette::Result<()> {
    let total = urls.len() as u64;
    let pb = if silent || json {
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

    let semaphore = Arc::new(Semaphore::new(tabs));
    let mut handles = Vec::with_capacity(urls.len());

    for url in urls {
        let permit = semaphore.clone().acquire_owned().await.into_diagnostic()?;
        let browser = Arc::clone(browser);
        let js = javascript.clone();
        let pb = Arc::clone(&pb);
        let ua = user_agent.clone();
        let px = proxy.clone();
        let pool = Arc::clone(&ua_pool);
        let handle = tokio::spawn(async move {
            pb.set_message(url.clone());
            if let Err(error) = take_screenshot(
                &browser,
                url.clone(),
                timeout,
                delay,
                silent,
                full_page,
                screenshot_type,
                danger_accept_invalid_certs,
                js,
                &pb,
                is_bulk,
                ua,
                px,
                json,
                &pool,
            )
            .await
            {
                // Build full error chain so the root cause is visible
                let full_err = {
                    let mut msg = error.to_string();
                    let mut source: Option<&dyn std::error::Error> = error.source();
                    while let Some(cause) = source {
                        msg.push_str(": ");
                        msg.push_str(&cause.to_string());
                        source = cause.source();
                    }
                    msg
                };
                if json && !silent {
                    let err = ScreenshotError {
                        url,
                        error: full_err,
                    };
                    println!("{}", serde_json::to_string(&err).unwrap());
                } else if is_bulk && !silent {
                    pb.suspend(|| {
                        output::show_line_error(&full_err);
                    });
                } else {
                    pb.suspend(|| {
                        eprintln!("{} {full_err}", "warning:".bold().yellow());
                    });
                }
            }
            pb.inc(1);
            drop(permit);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.into_diagnostic()?;
    }

    pb.finish_and_clear();

    Ok(())
}

async fn take_screenshot(
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
    proxy: Option<String>,
    json: bool,
    ua_pool: &[String],
) -> miette::Result<()> {
    let url = if !url.starts_with("http://") && !url.starts_with("https://") {
        format!("https://{url}")
    } else {
        url
    };

    // Resolve effective user-agent: per-URL pool rotation takes priority
    let user_agent = if !ua_pool.is_empty() {
        Some(user_agent::pick_from_pool(ua_pool).to_string())
    } else {
        user_agent
    };

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
    if let Some(ref proxy_url) = proxy {
        let proxy = reqwest::Proxy::all(proxy_url).into_diagnostic()?;
        client_builder = client_builder.proxy(proxy);
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

    // Create blank page, set per-page user-agent via CDP, then navigate
    let page = browser
        .new_page("about:blank")
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to open page: {url}"))?;
    if let Some(ref ua) = user_agent {
        page.set_user_agent(ua)
            .await
            .into_diagnostic()
            .wrap_err("Failed to set user-agent")?;
    }
    page.goto(parsed_url.clone())
        .await
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to navigate to: {url}"))?;
    tokio::time::sleep(Duration::from_secs(delay)).await;

    // Evaluate JavaScript if provided
    if let Some(js) = javascript {
        let result = page.evaluate(js.as_str()).await;
        if !json {
            match result {
                Ok(_) => pb.suspend(|| {
                    eprintln!(
                        "{} JavaScript executed successfully",
                        "info:".bold().magenta()
                    )
                }),
                Err(e) => pb.suspend(|| {
                    eprintln!(
                        "{} JavaScript execution failed: {e:?}",
                        "warning:".bold().yellow()
                    )
                }),
            }
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

        if json {
            let result = ScreenshotResult {
                url: url.clone(),
                title,
                status: response.status().as_u16(),
                filename: filename.clone(),
                elapsed_secs: elapsed.as_secs_f64(),
                user_agent: user_agent.clone(),
            };
            println!("{}", serde_json::to_string(&result).unwrap());
        } else if is_bulk {
            pb.suspend(|| output::show_line(&url, response.status(), &filename, elapsed));
        } else {
            pb.suspend(|| {
                output::show_info(
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
