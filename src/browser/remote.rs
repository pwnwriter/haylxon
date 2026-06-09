use super::BrowserProvider;
use chromiumoxide::browser::Browser;
use futures::StreamExt;
use miette::IntoDiagnostic;
use std::sync::Arc;

pub struct RemoteBrowser {
    /// WebSocket URL (`ws://...`) or HTTP URL (`http://host:port`).
    /// `chromiumoxide` handles both — if given an HTTP URL it auto-discovers
    /// the WebSocket endpoint via `/json/version`.
    url: String,
}

impl RemoteBrowser {
    /// Connect using a direct WebSocket debugger URL.
    /// e.g. `ws://127.0.0.1:9222/devtools/browser/<uuid>`
    pub fn from_ws_url(url: String) -> Self {
        Self { url }
    }

    /// Connect using a host:port pair.
    /// chromiumoxide will query `http://host:port/json/version` to discover
    /// the WebSocket URL automatically.
    pub fn from_host(host: String) -> Self {
        Self {
            url: format!("http://{host}"),
        }
    }
}

#[async_trait::async_trait]
impl BrowserProvider for RemoteBrowser {
    async fn connect(&self) -> miette::Result<Arc<Browser>> {
        let (browser, mut handler) = Browser::connect(&self.url)
            .await
            .into_diagnostic()
            .map_err(|e| {
                miette::miette!("Failed to connect to remote browser at {}: {e}", self.url)
            })?;
        tokio::task::spawn(async move { while handler.next().await.is_some() {} });
        Ok(Arc::new(browser))
    }
}
