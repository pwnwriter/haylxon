use colored::{Color, Colorize};

/// Prints the given message to the console and aborts the process.
#[allow(dead_code)]
pub fn abort(msg: &str) -> ! {
    error(msg);
    std::process::exit(1);
}

#[allow(dead_code)]
pub fn info(msg: &str, color: Color) {
    println!("{}: {}", "info".bold().color(color), msg);
}

pub fn error(msg: &str) {
    println!("{}: {}", "error".bold().color(Color::Red), msg);
}

#[allow(dead_code)]
pub fn success(msg: &str) {
    println!("{}: {}", "success".bold().color(Color::Green), msg);
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    println!("{}: {}", "warning".bold().color(Color::Yellow), msg);
}
