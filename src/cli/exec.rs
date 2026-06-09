use super::args::{Cli, Input};
use super::helpers::{combine_urls_with_ports, read_urls_from_file, read_urls_from_stdin};
use super::output::print_config_table;
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
                    ua_pool.clone(),
                )
                .await
            }
            _ => unreachable!(),
        }
    };

    is_screenshot_taken?;

    if !silent && !json {
        eprintln!(
            "{} Screenshots taken and saved in directory {}",
            "info:".bold().cyan(),
            outdir.bold()
        );
    }

    Ok(())
}
