use super::args::{Cli, Input};
use super::helpers::{combine_urls_with_ports, read_urls_from_file, read_urls_from_stdin};
use super::output::print_config_table;
use super::report::{self, ScreenshotOutcome};
use super::screenshot::take_screenshot_in_bulk;
use super::user_agent;
use crate::browser::pool::PagePool;
use crate::browser::{local::LocalBrowser, remote::RemoteBrowser, BrowserProvider};
use colored::Colorize;
use miette::{Context, IntoDiagnostic};
use std::sync::Arc;
use std::{env, path::Path};
use tokio::fs;

pub async fn run(
    Cli {
        binary_path,
        remote_url,
        remote_host,
        input: Input { url, file_path },
        stdin,
        outdir,
        tabs,
        width,
        height,
        timeout,
        delay,
        silent,
        fullpage,
        screenshot_type,
        ports,
        accept_invalid_certs,
        javascript,
        user_agent,
        proxy,
        json,
        html_report,
        reuse_tabs,
        pool_size,
    }: Cli,
) -> miette::Result<()> {
    // Resolve --user-agent into a fixed UA + a per-URL rotation pool
    let (user_agent, ua_pool) = user_agent::resolve(user_agent)?;

    // --- Select browser provider ---
    let provider: Box<dyn BrowserProvider> = if let Some(ws_url) = remote_url {
        Box::new(RemoteBrowser::from_ws_url(ws_url))
    } else if let Some(host) = remote_host {
        Box::new(RemoteBrowser::from_host(host))
    } else {
        let browser = Path::new(&binary_path);
        if !browser.exists() {
            return Err(miette::miette!(
                "Unable to locate browser binary {binary_path}"
            ));
        }
        Box::new(LocalBrowser::new(
            &binary_path,
            width,
            height,
            user_agent.as_deref(),
            proxy.as_deref(),
        )?)
    };

    let browser = provider.connect().await?;

    // --- Optional page pool ---
    let pool = if reuse_tabs {
        Some(Arc::new(PagePool::new(browser.clone(), pool_size)))
    } else {
        None
    };

    let dump_dir = Path::new(&outdir);

    if dump_dir.exists() {
        if !json {
            eprintln!(
                "{} A directory already exists as {}, bumping...",
                "info:".bold().bright_red(),
                outdir.bold()
            );
        }
    } else {
        fs::create_dir(dump_dir)
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to create output directory: {outdir}"))?;
        if !json {
            eprintln!(
                "{} Bump dir {}, created successfully",
                "info:".bold().green(),
                outdir.bold()
            );
        }
    }

    let is_screenshot_taken = if stdin {
        env::set_current_dir(dump_dir).into_diagnostic()?;
        let urls = read_urls_from_stdin(ports)?;
        if !silent && !json {
            print_config_table(
                urls.len(),
                "stdin",
                &outdir,
                tabs,
                &user_agent,
                ua_pool.len(),
            );
        }
        take_screenshot_in_bulk(
            &browser,
            pool.clone(),
            urls,
            tabs,
            timeout,
            delay,
            silent,
            fullpage,
            screenshot_type,
            accept_invalid_certs,
            javascript,
            true,
            user_agent,
            proxy,
            json,
            html_report,
            ua_pool.clone(),
        )
        .await
    } else {
        match (url, file_path) {
            (None, Some(file_path)) => {
                let urls = read_urls_from_file(&file_path, ports)?;
                if !silent && !json {
                    print_config_table(
                        urls.len(),
                        &file_path,
                        &outdir,
                        tabs,
                        &user_agent,
                        ua_pool.len(),
                    );
                }
                env::set_current_dir(dump_dir).into_diagnostic()?;
                take_screenshot_in_bulk(
                    &browser,
                    pool.clone(),
                    urls,
                    tabs,
                    timeout,
                    delay,
                    silent,
                    fullpage,
                    screenshot_type,
                    accept_invalid_certs,
                    javascript,
                    true,
                    user_agent,
                    proxy,
                    json,
                    html_report,
                    ua_pool.clone(),
                )
                .await
            }
            (Some(url), None) => {
                let urls = if let Some(ports) = ports {
                    combine_urls_with_ports(vec![url.to_string()], Some(ports))
                } else {
                    vec![url.to_string()]
                };
                env::set_current_dir(dump_dir).into_diagnostic()?;
                take_screenshot_in_bulk(
                    &browser,
                    pool.clone(),
                    urls,
                    tabs,
                    timeout,
                    delay,
                    silent,
                    fullpage,
                    screenshot_type,
                    accept_invalid_certs,
                    javascript,
                    false,
                    user_agent,
                    proxy,
                    json,
                    html_report,
                    ua_pool.clone(),
                )
                .await
            }
            _ => unreachable!(),
        }
    };

    let outcomes = is_screenshot_taken?;

    if html_report {
        report::write_html_report("index.html", &outcomes)?;
        if !silent && !json {
            eprintln!(
                "{} HTML report written to {}",
                "info:".bold().cyan(),
                "index.html".bold()
            );
        }
    }

    if !silent && !json {
        print_completion_summary(&outdir, &outcomes);
    }

    Ok(())
}

fn print_completion_summary(outdir: &str, outcomes: &[ScreenshotOutcome]) {
    let (captured, failed) = completion_counts(outcomes);

    if captured > 0 {
        eprintln!(
            "{} {captured} screenshot(s) saved in directory {}",
            "info:".bold().cyan(),
            outdir.bold()
        );
        if failed > 0 {
            eprintln!("{} {failed} capture(s) failed", "warning:".bold().yellow());
        }
    } else if failed > 0 {
        eprintln!(
            "{} no screenshots were captured ({failed} failed)",
            "warning:".bold().yellow()
        );
    }
}

fn completion_counts(outcomes: &[ScreenshotOutcome]) -> (usize, usize) {
    let captured = outcomes
        .iter()
        .filter(|outcome| matches!(outcome, ScreenshotOutcome::Success(_)))
        .count();
    let failed = outcomes.len().saturating_sub(captured);
    (captured, failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::output::{ScreenshotError, ScreenshotResult};

    #[test]
    fn completion_counts_all_failures() {
        let outcomes = vec![ScreenshotOutcome::Error(ScreenshotError {
            url: "https://ok.oo".to_string(),
            error: "net::ERR_NAME_NOT_RESOLVED".to_string(),
        })];

        assert_eq!(completion_counts(&outcomes), (0, 1));
    }

    #[test]
    fn completion_counts_mixed_results() {
        let outcomes = vec![
            ScreenshotOutcome::Success(ScreenshotResult {
                url: "https://example.com".to_string(),
                title: "Example".to_string(),
                status: 200,
                filename: "example.png".to_string(),
                elapsed_secs: 0.1,
                user_agent: None,
            }),
            ScreenshotOutcome::Error(ScreenshotError {
                url: "https://bad.invalid".to_string(),
                error: "failed".to_string(),
            }),
        ];

        assert_eq!(completion_counts(&outcomes), (1, 1));
    }
}
