pub mod local;
pub mod pool;
pub mod remote;

use chromiumoxide::browser::Browser;
use std::sync::Arc;

/// Abstraction over how a browser instance is obtained.
///
/// `chromiumoxide::Browser` works identically whether created via `launch()`
/// or `connect()`, so this trait only gates *how we get* the browser — not
/// how we use it.
#[async_trait::async_trait]
pub trait BrowserProvider: Send + Sync {
    /// Return a connected browser and spawn its CDP event handler.
    async fn connect(&self) -> miette::Result<Arc<Browser>>;
}
