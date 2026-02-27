use super::args::{Cli, Input};
use super::helpers::{combine_urls_with_ports, read_urls_from_file, read_urls_from_stdin};
use super::output::print_config_table;
use super::screenshot::take_screenshot_in_bulk;
use super::user_agent;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
};
use colored::Colorize;
use futures::StreamExt;
use miette::{Context, IntoDiagnostic};
use std::sync::Arc;
use std::{env, path::Path};
use tokio::{fs, task};

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
    }: Cli,
) -> miette::Result<()> {
    // Resolve --user-agent into a fixed UA + a per-URL rotation pool
    let (user_agent, ua_pool) = user_agent::resolve(user_agent)?;

    let browser = Path::new(&binary_path);
    if !browser.exists() {
        return Err(miette::miette!(
            "Unable to locate browser binary {binary_path}"
        ));
    }

    let mut browser_config = BrowserConfig::builder();
    browser_config = browser_config
        .no_sandbox()
        .arg("--disable-dev-shm-usage")
        .arg("--disable-gpu")
        .window_size(width, height)
        .chrome_executable(browser)
        .viewport(Viewport {
            width,
            height,
            device_scale_factor: None,
            emulating_mobile: false,
            is_landscape: false,
            has_touch: false,
        });

    if let Some(ref ua) = user_agent {
        browser_config = browser_config.arg(format!("--user-agent={ua}"));
    }

    if let Some(ref proxy_url) = proxy {
        browser_config = browser_config.arg(format!("--proxy-server={proxy_url}"));
    }

    let (browser, mut handler) =
        Browser::launch(browser_config.build().map_err(|e| miette::miette!(e))?)
            .await
            .into_diagnostic()
            .wrap_err(format!("Error instantiating browser {binary_path}"))?;
    let browser = Arc::new(browser);

    task::spawn(async move { while handler.next().await.is_some() {} });

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
