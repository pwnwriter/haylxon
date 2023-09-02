use colored::Colorize;
// pub const BAR: &str = r"
// ────────────────────────────────
// ";

pub fn splash() -> String {
    let logo = r"
 ╦ ╦╔═╗╦ ╦╦   ╔═╗╔╗╔
 ╠═╣╠═╣╚╦╝║  𝖃║ ║║║║
 ╩ ╩╩ ╩ ╩ ╩═╝ ╚═╝╝╚╝v0.1.5
             by @PwnWriter
 "
    .purple();
    let quote = " Shoot before the blink  ".italic();

    format!("{logo} {quote}")
}
