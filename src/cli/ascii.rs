use colored::Colorize;
pub const BAR: &str = r"
────────────────────────────────
";

pub const RESET: &str = "\x1B[0m"; //( resets the text color to the default)

pub fn splash() -> String {
    let logo = r"
 ╦ ╦╔═╗╦ ╦╦   ╔═╗╔╗╔
 ╠═╣╠═╣╚╦╝║  𝖃║ ║║║║
 ╩ ╩╩ ╩ ╩ ╩═╝ ╚═╝╝╚╝v0.1.6
             by @PwnWriter
 "
    .purple();
    let quote = " Shoot before the blink  ".italic();

    format!("{logo} {quote}")
}
