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

#[derive(Parser)]
#[command(author, version, about, long_about = "None")]
struct Cli {
    #[arg(short, long)]
    /// Website URL/filename of file containing URLs
    url: String,
    #[arg(short, long, default_value = "hxnshots")]
    /// Output directory to save screenshots
    outdir: String,
    #[arg(short, long, default_value = "4")]
    /// Maximum number of parallel tabs
    tabs: Option<usize>,

    #[arg(short, long, default_value = "/usr/bin/chrome")]
    /// Browser binary path
    binary_path: String,

    #[arg(long, default_value = "1440")]
    /// Width of the website // URL
    width: Option<u32>,

    #[arg(long, default_value = "900")]
    /// Height of the website // URL
    height: Option<u32>,
    #[arg(long)]
    /// Silent mode (suppress all console output)
    silent: bool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    if !cli.silent {
        println!("{CYAN}{}{RESET}", HXN);
    }

    run(
        cli.url,
        Some(cli.outdir),
        cli.tabs,
        cli.binary_path,
        cli.width,
        cli.height,
        cli.silent,
    )
    .await
    .expect("An error occurred while running :(");

    Ok(())
}
async fn run(
    url: String,
    outdir: Option<String>,
    tabs: Option<usize>,
    binary_path: String,
    width: Option<u32>,
    height: Option<u32>,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        let h = tokio::spawn(take_screenshots(n_tab, chunk, silent));
        handles.push(h);
    }

    for handle in handles {
        handle
            .await?
            .expect("Something went wrong while waiting for taking screenshot and saving to file");
    }

    println!("{RED}♥ {GREEN} {YELLOW_BRIGHT}Yoo 1337!! Screenshots saved in dir {outdir}{RED} ♥ {GREEN}{RESET} ");

    Ok(())
}

async fn take_screenshots(
    page: Page,
    urls: Vec<reqwest::Url>,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for url in urls {
        let url = url.as_str();
        if let Ok(Ok(_res)) = timeout(Duration::from_secs(10), get(url)).await {
            // Thanks to swanandx for this
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
                    &format!("{GREEN}[{CYAN}  {GREEN}] URL={GREEN}{}", url),
                    &format!(
                        "{BLUE}[{CYAN}  {YELLOW}] Title={GREEN}{}",
                        page.get_title().await?.unwrap_or_default()
                    ),
                    &format!("{BLUE}[{CYAN}  {YELLOW}] Status={GREEN}{}", _res.status()),
                ],
            ])
            .set_tabsize(0)
            .make_columns();
            if !silent {
                println!("{_info}");
            }
        } else {
            println!("{RED}[-] Timed out URL={YELLOW}{}", url);
        }
    }

    Ok(())
}
