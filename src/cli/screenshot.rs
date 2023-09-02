use crate::colors::*;
use crate::log::error;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::viewport::Viewport;
use futures::StreamExt;
use std::{
    env,
    io::{BufRead, BufReader},
    path::Path,
};
use tokio::{fs, time::timeout};

use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams,
};
use chromiumoxide::Page;
use columns::Columns;
use core::time::Duration;
use reqwest::get;

#[allow(clippy::too_many_arguments)]
pub async fn run(
    url: Option<String>,
    outdir: Option<String>,
    tabs: Option<usize>,
    binary_path: String,
    width: Option<u32>,
    height: Option<u32>,
    timeout: u64,
    silent: bool,
    stdin: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !Path::new(&binary_path).exists() {
        error("Unble to locate browser binary");

        std::process::exit(0);
    }
    let outdir = match outdir {
        Some(dir) => dir,
        None => "hxnshots".to_string(),
    };

    let viewport_width = width.unwrap_or(1440);
    let viewport_height = height.unwrap_or(900);

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(viewport_width, viewport_height)
            .chrome_executable(Path::new(&binary_path))
            .viewport(Viewport {
                width: viewport_width,
                height: viewport_height,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            })
            .build()?,
    )
    .await?;

    let _handle = tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await;
        }
    });

    if fs::metadata(&outdir).await.is_err() {
        fs::create_dir(&outdir).await?;
    }

    let urls: Vec<String>; // Define the 'urls' variable outside the match statement

    #[allow(unreachable_patterns)]
    match stdin {
        true => {
            urls = crate::cli::hxn_helper::read_urls_from_stdin();
        }

        false => {
            if let Some(url) = &url {
                if Path::new(url).exists() {
                    // Read URLs from file
                    let file = std::fs::File::open(url)?;
                    let lines = BufReader::new(file).lines().map_while(Result::ok);
                    urls = lines.collect(); // Assign the collected lines to 'urls'
                } else {
                    // URL is a single URL
                    urls = vec![url.clone()]; // Assign the single URL to 'urls'
                }
            } else {
                // Handle the case where 'url' is None (you can decide what to do in this case)
                // For now, let's assume it's an empty vector
                urls = vec![];
            }
        }

        _ => {
            // Handle other cases if needed
            // For now, let's assume it's an empty vector
            urls = vec![];
        }
    }

    let mut url_chunks = Vec::new();

    for chunk in urls.chunks(tabs.unwrap_or(4)) {
        let mut urls = Vec::new();
        for url in chunk {
            if let Ok(url) = url::Url::parse(url) {
                urls.push(url);
            }
        }
        url_chunks.push(urls);
    }

    env::set_current_dir(Path::new(&outdir))?;

    let mut handles = Vec::new();

    for chunk in url_chunks {
        let n_tab = browser.new_page("about:blank").await?;
        let h = tokio::spawn(take_screenshots(n_tab, chunk, silent, timeout));
        handles.push(h);
    }

    for handle in handles {
        handle
            .await?
            .expect("Something went wrong while waiting for taking screenshot and saving to file");
    }

    println!("Screenshots saved in dir {outdir}");

    Ok(())
}

async fn take_screenshots(
    page: Page,
    urls: Vec<reqwest::Url>,
    silent: bool,
    timeout_value: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for url in urls {
        let url = url.as_str();
        if let Ok(Ok(_res)) = timeout(Duration::from_secs(timeout_value), get(url)).await {
            let filename = url.replace("://", "-").replace('/', "_") + ".png";
            page.goto(url)
                .await?
                .save_screenshot(
                    CaptureScreenshotParams::builder()
                        .format(CaptureScreenshotFormat::Png)
                        .build(),
                    filename,
                )
                .await?;

            let _info = Columns::from(vec![
                format!("{RESET}").split('\n').collect::<Vec<&str>>(),
                vec![
                    &format!("{BLUE}"),
                    &format!("{GREEN}[{CYAN}  {GREEN}] URL={GREEN}{}", url),
                    &format!(
                        "{BLUE}[{CYAN}  {YELLOW}] Title={GREEN}{}",
                        page.get_title().await?.unwrap_or_default()
                    ),
                    &format!("{BLUE}[{CYAN} ﯜ {YELLOW}] Status={GREEN}{}", _res.status()),
                ],
            ])
            .set_tabsize(0)
            .make_columns();
            if !silent {
                println!("{_info}");
            }
        } else {
            println!("{RED}[-] Timed out URL = {YELLOW}{}", url);
        }
    }

    Ok(())
}
