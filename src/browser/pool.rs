use chromiumoxide::{browser::Browser, Page};
use miette::IntoDiagnostic;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A pool of reusable browser pages.
///
/// Instead of creating and destroying a page for every URL, pages are returned
/// to the pool after use and recycled for subsequent screenshots. Each recycled
/// page is navigated to `about:blank` to reset its state before reuse.
pub struct PagePool {
    browser: Arc<Browser>,
    pages: Mutex<Vec<Page>>,
    max_size: usize,
}

impl PagePool {
    pub fn new(browser: Arc<Browser>, max_size: usize) -> Self {
        Self {
            browser,
            pages: Mutex::new(Vec::with_capacity(max_size)),
            max_size,
        }
    }

    /// Get a page from the pool, or create a new one if the pool is empty.
    pub async fn acquire(&self) -> miette::Result<Page> {
        let mut pages = self.pages.lock().await;
        if let Some(page) = pages.pop() {
            drop(pages);
            // Reset state so the page is clean for the next URL
            let _ = page.goto("about:blank").await;
            Ok(page)
        } else {
            drop(pages);
            self.browser.new_page("about:blank").await.into_diagnostic()
        }
    }

    /// Return a page to the pool for reuse, or close it if the pool is full.
    pub async fn release(&self, page: Page) {
        let mut pages = self.pages.lock().await;
        if pages.len() < self.max_size {
            pages.push(page);
        } else {
            drop(pages);
            let _ = page.close().await;
        }
    }
}
