use super::args::{Cli, Input};
use super::screenshot::take_screenshot_in_bulk;
use crate::cli::hxn_helper::combine_urls_with_ports;
use crate::log::info;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
};
use colored::Colorize;
use futures::StreamExt;
use miette::{Context, IntoDiagnostic};
use std::sync::Arc;
use std::{env, path::Path};
use tabled::{builder::Builder, settings::Style};
use tokio::{fs, task};

fn print_config_table(
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
        builder.push_record(["User-Agent", &format!("random-per-url ({ua_pool_size} agents)")]);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{table}");
}

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
    let (user_agent, ua_pool) = match user_agent.as_deref() {
        Some("random") => {
            let ua = super::screenshot::pick_random_user_agent().to_string();
            (Some(ua), Arc::new(Vec::new()))
        }
        Some("random-per-url") => {
            let built_in: Vec<String> = super::screenshot::built_in_user_agents()
                .iter()
                .map(|s| s.to_string())
                .collect();
            (None, Arc::new(built_in))
        }
        Some(path) if Path::new(path).is_file() => {
            let contents = std::fs::read_to_string(path)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to read user-agent file: {path}"))?;
            let agents: Vec<String> = contents
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            if agents.is_empty() {
                return Err(miette::miette!("User-agent file is empty: {path}"));
            }
            (None, Arc::new(agents))
        }
        Some(custom) => (Some(custom.to_string()), Arc::new(Vec::new())),
        None => (None, Arc::new(Vec::new())),
    };

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
            info(
                format!(
                    "A directory already exists as {} bumping ...",
                    outdir.bold()
                ),
                colored::Color::BrightRed,
            );
        }
    } else {
        match fs::create_dir(dump_dir).await {
            Ok(_) => {
                if !json {
                    info(
                        format!("Bump dir {}, created successfully ..", outdir.bold()),
                        colored::Color::Green,
                    );
                }
            }
            Err(err) => {
                if !json {
                    info(
                        format!("Failed to create directory: {}", err),
                        colored::Color::Red,
                    );
                }
                return Err(err).into_diagnostic();
            }
        }
    }

    let is_screenshot_taken = if stdin {
        env::set_current_dir(dump_dir).into_diagnostic()?;
        let urls = super::hxn_helper::read_urls_from_stdin(ports)?;
        if !silent && !json {
            print_config_table(urls.len(), "stdin", &outdir, tabs, &user_agent, ua_pool.len());
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
                let urls = super::hxn_helper::read_urls_from_file(&file_path, ports)?;
                if !silent && !json {
                    print_config_table(urls.len(), &file_path, &outdir, tabs, &user_agent, ua_pool.len());
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

    match is_screenshot_taken {
        Ok(_) => {
            if !silent && !json {
                info(
                    format!("Screenshots Taken and saved in directory {}", outdir.bold()),
                    colored::Color::Cyan,
                );
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
