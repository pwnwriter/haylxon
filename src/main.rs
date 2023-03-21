mod colors;
use colors::*;
mod ascii;
use ascii::*;
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
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = "None")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(short, long)]
        /// Website URL/filename of file containing URLs
        url: String,
        #[arg(short, long, default_value = "screenshots")]
        /// Output directory to save screenshots (default is 'screenshots')
        outdir: String,
        #[arg(short, long)]
        /// "Maximum number of parallel tabs (default 4)"
        tabs: Option<usize>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{RED}{}{RESET}", HYALCON);

    let cli = Cli::parse();

    match cli.commands {
        Commands::New { url, outdir, tabs } => {
            run(url, Some(outdir), tabs)
                .await
                .expect("An error occurred while running :(");
        }
    }
    Ok(())
}

async fn run(
    url: String,
    outdir: Option<String>,
    tabs: Option<usize>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let outdir = if outdir.is_some() {
        outdir.unwrap()
    } else {
        "screenshots".to_string()
    };

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(1440, 900)
            .viewport(Viewport {
                width: 1440,
                height: 900,
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

    if fs::metadata(&url).await.is_ok() {
        let file = std::fs::File::open(url)?;
        let lines = BufReader::new(file).lines();

        let mut urls = vec![Vec::new(); tabs.unwrap()];
        let mut pt = 0;

        // Only take valid URLs
        // push them in urls in round robin manner
        for line in lines.flatten() {
            if let Ok(url) = url::Url::parse(&line) {
                urls[pt].push(url);
                pt += 1;
                pt %= tabs.unwrap();
            }
        }

        // Set current working directory to output directory
        // So that we can save screenshots in it without specifying whole path.
        env::set_current_dir(Path::new(&outdir))?;

        let mut handles = Vec::new();

        for chunk in urls {
            let n_tab = browser.new_page("about:blank").await?;
            let h = tokio::spawn(take_screenshots(n_tab, chunk));
            handles.push(h);
        }

        for handle in handles {
            handle.await?.expect(
                "Something went wrong while waiting for taking screenshot and saving to file",
            );
        }
    } else if let Ok(valid_url) = url::Url::parse(&url) {
        env::set_current_dir(Path::new(&outdir))?;
        let n_tab = browser.new_page("about:blank").await?;
        take_screenshots(n_tab, vec![valid_url]).await?;
    }

    println!("{GREEN}[{CYAN}{GREEN}] {YELLOW_BRIGHT}Screenshots Taken {GREEN}[{CYAN}{GREEN}]");

    Ok(())
}

async fn take_screenshots(
    page: Page,
    urls: Vec<reqwest::Url>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for url in urls {
        let url = url.as_str();
        if let Ok(Ok(res)) = timeout(Duration::from_secs(10), get(url)).await {
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
            println!(
                "{GREEN}[{CYAN}{GREEN}] status={BLUE}{}{CYAN} title={CYAN}{}{BLUE} URL={GREEN}{}",
                res.status(),
                page.get_title().await?.unwrap_or_default(),
                url
            );
        } else {
            println!("{RED}[-] Timed out URL={YELLOW}{}", url);
        }
    }

    Ok(())
}
