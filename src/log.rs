#![allow(unused)]

use colored::{Color, Colorize};

/// Prints the given message to the console and aborts the process.
pub fn abort(msg: String) -> ! {
    error(msg);
    std::process::exit(1);
}

pub fn info(msg: String, color: Color) {
    println!("{}: {}", "info".bold().color(color), msg);
}

pub fn error(msg: String) {
    eprintln!("{}: {}", "error".bold().color(Color::Red), msg);
}

pub fn success(msg: String) {
    println!("{}: {}", "success".bold().color(Color::Green), msg);
}

pub fn warn(msg: String) {
    println!("{}: {}", "warning".bold().color(Color::Yellow), msg);
}
