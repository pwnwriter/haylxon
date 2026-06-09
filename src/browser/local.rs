use super::BrowserProvider;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    handler::viewport::Viewport,
};
use futures::StreamExt;
use miette::IntoDiagnostic;
use std::sync::Arc;

pub struct LocalBrowser {
    config: BrowserConfig,
}

impl LocalBrowser {
    pub fn new(
        binary_path: &str,
        width: u32,
        height: u32,
        user_agent: Option<&str>,
        proxy: Option<&str>,
    ) -> miette::Result<Self> {
        let mut builder = BrowserConfig::builder();
        builder = builder
            .no_sandbox()
            .arg("--disable-dev-shm-usage")
            .arg("--disable-gpu")
            .window_size(width, height)
            .chrome_executable(binary_path)
            .viewport(Viewport {
                width,
                height,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            });

        if let Some(ua) = user_agent {
            builder = builder.arg(format!("--user-agent={ua}"));
        }

        if let Some(proxy_url) = proxy {
            builder = builder.arg(format!("--proxy-server={proxy_url}"));
        }

        let config = builder.build().map_err(|e| miette::miette!(e))?;
        Ok(Self { config })
    }
}

#[async_trait::async_trait]
impl BrowserProvider for LocalBrowser {
    async fn connect(&self) -> miette::Result<Arc<Browser>> {
        let (browser, mut handler) = Browser::launch(self.config.clone())
            .await
            .into_diagnostic()?;
        tokio::task::spawn(async move { while handler.next().await.is_some() {} });
        Ok(Arc::new(browser))
    }
}
