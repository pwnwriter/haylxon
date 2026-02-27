use miette::{Context, IntoDiagnostic};
use std::path::Path;
use std::sync::Arc;

/// Built-in list of common browser user-agent strings.
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

/// Select a random user-agent from the built-in list.
pub fn pick_random() -> &'static str {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    USER_AGENTS[nanos % USER_AGENTS.len()]
}

/// Select a random user-agent from a custom pool.
pub fn pick_from_pool(pool: &[String]) -> &str {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    &pool[nanos % pool.len()]
}

/// Parse the `--user-agent` CLI value into a fixed UA and a per-URL rotation pool.
///
/// Modes:
/// - `"random"` — pick one random UA from the built-in list (fixed for the session)
/// - `"random-per-url"` — rotate through the built-in list (one per URL)
/// - a file path — read custom UAs from file (one per line), rotate per URL
/// - any other string — use it as-is for every request
/// - `None` — no custom user-agent
pub fn resolve(user_agent: Option<String>) -> miette::Result<(Option<String>, Arc<Vec<String>>)> {
    match user_agent.as_deref() {
        Some("random") => {
            let ua = pick_random().to_string();
            Ok((Some(ua), Arc::new(Vec::new())))
        }
        Some("random-per-url") => {
            let built_in: Vec<String> = USER_AGENTS.iter().map(|s| s.to_string()).collect();
            Ok((None, Arc::new(built_in)))
        }
        Some(path) if Path::new(path).is_file() => {
            let contents = std::fs::read_to_string(path)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to read user-agent file: {path}"))?;
            let agents: Vec<String> = contents
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            if agents.is_empty() {
                return Err(miette::miette!("User-agent file is empty: {path}"));
            }
            Ok((None, Arc::new(agents)))
        }
        Some(custom) => Ok((Some(custom.to_string()), Arc::new(Vec::new()))),
        None => Ok((None, Arc::new(Vec::new()))),
    }
}
