use super::args::{Cli, Input};
use super::screenshot::{take_screenshot, take_screenshot_in_bulk};
use anyhow::Context;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
};
use colored::{Color, Colorize};
use futures::StreamExt;
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
        verbose,
        fullpage,
        screenshot_type,
    }: Cli,
) -> anyhow::Result<()> {
    let browser = Path::new(&binary_path);
    if !browser.exists() {
        return Err(anyhow::Error::msg(format!(
            "Unable to locate browser binary {binary_path}"
        )));
    }

    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .no_sandbox()
            .window_size(width, height)
            .chrome_executable(browser)
            .viewport(Viewport {
                width,
                height,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            })
            .build()
            .map_err(anyhow::Error::msg)?,
    )
    .await
    .context(format!("Error instantiating browser {binary_path}"))?;
    let browser = Arc::new(browser);

    task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let dump_dir = Path::new(&outdir);
    if !dump_dir.exists() {
        // TODO: Check error cases for reporting
        fs::create_dir(dump_dir).await?;
    }

    if stdin {
        env::set_current_dir(dump_dir)?;
        let urls = super::hxn_helper::read_urls_from_stdin()?;
        take_screenshot_in_bulk(
            &browser,
            urls,
            tabs,
            timeout,
            verbose,
            fullpage,
            screenshot_type,
        )
        .await?;
    } else {
        match (url, file_path) {
            (None, Some(file_path)) => {
                let urls = super::hxn_helper::read_urls_from_file(file_path)?;
                env::set_current_dir(dump_dir)?;
                take_screenshot_in_bulk(
                    &browser,
                    urls,
                    tabs,
                    timeout,
                    verbose,
                    fullpage,
                    screenshot_type,
                )
                .await?;
            }
            (Some(url), None) => {
                env::set_current_dir(dump_dir)?;
                take_screenshot(&browser, url, timeout, verbose, fullpage, screenshot_type).await?;
            }
            _ => unreachable!(),
        }
    }

    println!(
        "{}: {}",
        "Screenshots Taken and saved in directory"
            .bold()
            .color(Color::Green),
        outdir
    );

    Ok(())
}
