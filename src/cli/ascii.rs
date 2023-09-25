use colored::Colorize;

pub const BAR: &str = r"
────────────────────────────────
";

pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)

pub fn splash() -> String {
    let hxn_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
 ╦ ╦╔═╗╦ ╦╦   ╔═╗╔╗╔
 ╠═╣╠═╣╚╦╝║  𝖃║ ║║║║
 ╩ ╩╩ ╩ ╩ ╩═╝ ╚═╝╝╚╝v{}
             by @PwnWriter
 "#,
        hxn_version
    )
    .purple();

    let quote = " Shoot before the blink  ".italic();

    format!("{logo} {quote}")
}
