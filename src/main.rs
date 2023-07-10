mod args;
use args::*;
mod colors;
use colors::*;
mod ascii;
use ascii::{BAR, HXN};
use std::{
    env,
    io::{BufRead, BufReader},
    path::Path,
    time::Duration,
};

use reqwest::get;

use tokio::{fs, time::timeout};

use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams,
};
use chromiumoxide::handler::viewport::Viewport;
use chromiumoxide::Page;

use clap::Parser;

use columns::Columns;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{CYAN}{}{RESET}", HXN);
    let cli = Cli::parse();
    run(
        cli.url,
        Some(cli.outdir),
        cli.tabs,
        cli.binary_path,
        cli.width,
        cli.height,
        cli.timeout_value,
        cli.silent,
    )
    .await
    .expect("An error occurred while running :(");

    Ok(())
}

fn exit_on_error() {
    std::process::exit(0);
}

async fn run(
    url: String,
    outdir: Option<String>,
    tabs: Option<usize>,
    binary_path: String,
    width: Option<u32>,
    height: Option<u32>,
    timeout_value: u64,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check if the browser binary path is valid
    if !Path::new(&binary_path).exists() {
        println!("{RED}[   ]Browser binary not found at path {}, You should try manually pasing the binary path !!{RESET}", binary_path);
        println!("{BLUE}[ ﯦ  ]{RESET}{CYAN} $ hxn -b $(which brave) <url> or use --help flag");
        exit_on_error();
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

    let urls: Vec<String> = if Path::new(&url).exists() {
        // Read URLs from file
        let file = std::fs::File::open(&url)?;
        let lines = BufReader::new(file).lines().filter_map(Result::ok);
        lines.collect()
    } else {
        // URL is a single URL
        vec![url]
    };

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

    // Set current working directory to output directory
    // So that we can save screenshots in it without specifying whole path.
    env::set_current_dir(Path::new(&outdir))?;

    let mut handles = Vec::new();

    for chunk in url_chunks {
        let n_tab = browser.new_page("about:blank").await?;
        let h = tokio::spawn(take_screenshots(n_tab, chunk, silent, timeout_value));
        handles.push(h);
    }

    for handle in handles {
        handle
            .await?
            .expect("Something went wrong while waiting for taking screenshot and saving to file");
    }

    exit_on_error();

    println!(
        "{RED}♥ {GREEN} {YELLOW_BRIGHT}Screenshots saved in dir {outdir}{RED} ♥ {GREEN}{RESET} "
    );

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
                    &format!("{BLUE}{BAR}"),
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
