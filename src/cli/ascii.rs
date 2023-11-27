use colored::Colorize;

pub const BAR: &str = r"
────────────────────────────────
";

pub const RESET: &str = "\x1B[0m"; // (resets the text color to the default)

pub fn splash() -> String {
    let hxn_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
    ┓┏    ┓      
    ┣┫┏┓┓┏┃┓┏┏┓┏┓
    ┛┗┗┻┗┫┗┛┗┗┛┛┗ v{}
         ┛       
    Shoot before the blink
                @pwnwriter/haylxon
 "#,
        hxn_version
    )
    .purple();

    format!("{logo}")
}
