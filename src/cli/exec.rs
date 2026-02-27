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
use std::time::Instant;
use std::{env, path::Path};
use tabled::{builder::Builder, settings::Style};
use tokio::{fs, task};

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
];

fn print_config_table(
    url_count: usize,
    source: &str,
    outdir: &str,
    tabs: usize,
    user_agent: &Option<String>,
) {
    let mut builder = Builder::default();
    builder.push_record(["URLs", &url_count.to_string()]);
    builder.push_record(["Source", source]);
    builder.push_record(["Output", outdir]);
    builder.push_record(["Tabs", &tabs.to_string()]);
    if let Some(ua) = user_agent {
        builder.push_record(["User-Agent", ua]);
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
        random_user_agent,
        proxy,
    }: Cli,
) -> miette::Result<()> {
    let user_agent = if let Some(ua) = user_agent {
        Some(ua)
    } else if random_user_agent {
        let idx = Instant::now().elapsed().subsec_nanos() as usize % USER_AGENTS.len();
        Some(USER_AGENTS[idx].to_string())
    } else {
        None
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
        info(
            format!(
                "A directory already exists as {} bumping ...",
                outdir.bold()
            ),
            colored::Color::BrightRed,
        );
    } else {
        match fs::create_dir(dump_dir).await {
            Ok(_) => info(
                format!("Bump dir {}, created successfully ..", outdir.bold()),
                colored::Color::Green,
            ),
            Err(err) => {
                info(
                    format!("Failed to create directory: {}", err),
                    colored::Color::Red,
                );
                return Err(err).into_diagnostic();
            }
        }
    }

    let is_screenshot_taken = if stdin {
        env::set_current_dir(dump_dir).into_diagnostic()?;
        let urls = super::hxn_helper::read_urls_from_stdin(ports)?;
        if !silent {
            print_config_table(urls.len(), "stdin", &outdir, tabs, &user_agent);
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
        )
        .await
    } else {
        match (url, file_path) {
            (None, Some(file_path)) => {
                let urls = super::hxn_helper::read_urls_from_file(&file_path, ports)?;
                if !silent {
                    print_config_table(urls.len(), &file_path, &outdir, tabs, &user_agent);
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
                )
                .await
            }
            _ => unreachable!(),
        }
    };

    match is_screenshot_taken {
        Ok(_) => {
            if !silent {
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
