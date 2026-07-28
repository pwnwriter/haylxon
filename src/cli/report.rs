use super::output::{ScreenshotError, ScreenshotResult};
use miette::{Context, IntoDiagnostic};
use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub enum ScreenshotOutcome {
    Success(ScreenshotResult),
    Error(ScreenshotError),
}

pub fn write_html_report<P: AsRef<Path>>(
    path: P,
    outcomes: &[ScreenshotOutcome],
) -> miette::Result<()> {
    std::fs::write(&path, render_html_report(outcomes))
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write HTML report: {:?}", path.as_ref()))
}

pub fn render_html_report(outcomes: &[ScreenshotOutcome]) -> String {
    let total = outcomes.len();
    let succeeded = outcomes
        .iter()
        .filter(|outcome| matches!(outcome, ScreenshotOutcome::Success(_)))
        .count();
    let failed = total.saturating_sub(succeeded);

    let mut cards = String::new();
    for outcome in outcomes {
        match outcome {
            ScreenshotOutcome::Success(result) => cards.push_str(&success_card(result)),
            ScreenshotOutcome::Error(error) => cards.push_str(&error_card(error)),
        }
    }

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Haylxon Report</title>
  <style>
    :root {{
      color-scheme: light dark;
      --bg: #f7f7f4;
      --fg: #202124;
      --muted: #6b6f76;
      --line: #d8d7d0;
      --panel: #ffffff;
      --ok: #1f7a4d;
      --err: #b42318;
    }}
    @media (prefers-color-scheme: dark) {{
      :root {{
        --bg: #151617;
        --fg: #f1f1ed;
        --muted: #a7abb3;
        --line: #34363a;
        --panel: #202225;
      }}
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      background: var(--bg);
      color: var(--fg);
      font: 14px/1.45 system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }}
    header {{
      padding: 28px clamp(16px, 4vw, 40px) 18px;
      border-bottom: 1px solid var(--line);
    }}
    h1 {{
      margin: 0 0 12px;
      font-size: clamp(28px, 4vw, 44px);
      line-height: 1;
      letter-spacing: 0;
    }}
    .stats {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      color: var(--muted);
    }}
    .stats span {{
      border: 1px solid var(--line);
      border-radius: 6px;
      padding: 5px 8px;
      background: var(--panel);
    }}
    main {{
      padding: 22px clamp(16px, 4vw, 40px) 40px;
    }}
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(min(100%, 320px), 1fr));
      gap: 16px;
    }}
    article {{
      overflow: hidden;
      border: 1px solid var(--line);
      border-radius: 8px;
      background: var(--panel);
    }}
    img {{
      display: block;
      width: 100%;
      aspect-ratio: 16 / 10;
      object-fit: cover;
      background: #111;
      border-bottom: 1px solid var(--line);
    }}
    .body {{
      padding: 12px;
    }}
    h2 {{
      margin: 0 0 8px;
      font-size: 15px;
      line-height: 1.25;
      letter-spacing: 0;
      overflow-wrap: anywhere;
    }}
    dl {{
      display: grid;
      grid-template-columns: max-content minmax(0, 1fr);
      gap: 5px 10px;
      margin: 0;
      color: var(--muted);
    }}
    dt {{ font-weight: 650; }}
    dd {{
      margin: 0;
      overflow-wrap: anywhere;
    }}
    .status-ok {{ color: var(--ok); }}
    .status-err {{ color: var(--err); }}
    .placeholder {{
      display: grid;
      min-height: 190px;
      place-items: center;
      color: var(--err);
      background: color-mix(in srgb, var(--err) 9%, var(--panel));
      border-bottom: 1px solid var(--line);
      font-weight: 700;
    }}
  </style>
</head>
<body>
  <header>
    <h1>Haylxon Report</h1>
    <div class="stats">
      <span>{total} total</span>
      <span>{succeeded} captured</span>
      <span>{failed} failed</span>
    </div>
  </header>
  <main>
    <section class="grid">
{cards}
    </section>
  </main>
</body>
</html>
"#
    )
}

fn success_card(result: &ScreenshotResult) -> String {
    let title = if result.title.trim().is_empty() {
        "No title"
    } else {
        &result.title
    };

    format!(
        r#"      <article>
        <a href="{image_href}" target="_blank" rel="noreferrer"><img src="{image_src}" alt="{alt}"></a>
        <div class="body">
          <h2>{title}</h2>
          <dl>
            <dt>URL</dt><dd><a href="{url}" target="_blank" rel="noreferrer">{url}</a></dd>
            <dt>Status</dt><dd class="status-ok">{status}</dd>
            <dt>File</dt><dd>{filename}</dd>
            <dt>Time</dt><dd>{elapsed:.2}s</dd>
{user_agent}
          </dl>
        </div>
      </article>
"#,
        image_href = escape_attr(&result.filename),
        image_src = escape_attr(&result.filename),
        alt = escape_attr(title),
        title = escape_html(title),
        url = escape_html(&result.url),
        status = result.status,
        filename = escape_html(&result.filename),
        elapsed = result.elapsed_secs,
        user_agent = result
            .user_agent
            .as_ref()
            .map(|ua| format!(
                "            <dt>User-Agent</dt><dd>{}</dd>\n",
                escape_html(ua)
            ))
            .unwrap_or_default()
    )
}

fn error_card(error: &ScreenshotError) -> String {
    format!(
        r#"      <article>
        <div class="placeholder">Capture failed</div>
        <div class="body">
          <h2>{url}</h2>
          <dl>
            <dt>URL</dt><dd>{url}</dd>
            <dt>Status</dt><dd class="status-err">ERR</dd>
            <dt>Error</dt><dd>{error}</dd>
          </dl>
        </div>
      </article>
"#,
        url = escape_html(&error.url),
        error = escape_html(&error.error)
    )
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(input: &str) -> String {
    escape_html(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_success_and_error_cards() {
        let html = render_html_report(&[
            ScreenshotOutcome::Success(ScreenshotResult {
                url: "https://example.com".to_string(),
                title: "Example Domain".to_string(),
                status: 200,
                filename: "https-example.com.png".to_string(),
                elapsed_secs: 0.42,
                user_agent: Some("hxn-test".to_string()),
            }),
            ScreenshotOutcome::Error(ScreenshotError {
                url: "https://bad.invalid".to_string(),
                error: "navigation failed".to_string(),
            }),
        ]);

        assert!(html.contains("<span>2 total</span>"));
        assert!(html.contains("<span>1 captured</span>"));
        assert!(html.contains("<span>1 failed</span>"));
        assert!(html.contains("Example Domain"));
        assert!(html.contains("https-example.com.png"));
        assert!(html.contains("navigation failed"));
    }

    #[test]
    fn escapes_untrusted_values() {
        let html = render_html_report(&[ScreenshotOutcome::Success(ScreenshotResult {
            url: "https://example.com/?q=<script>".to_string(),
            title: "<script>alert(1)</script>".to_string(),
            status: 200,
            filename: "shot\"bad.png".to_string(),
            elapsed_secs: 1.0,
            user_agent: None,
        })]);

        assert!(!html.contains("<script>alert"));
        assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(html.contains("shot&quot;bad.png"));
    }
}
