pub mod args;
pub mod ascii;
pub mod screenshot;
pub use args::*;
pub use ascii::*;
pub use screenshot::*;

pub mod hxn_helper {

    use std::io::BufRead;

    /// https://www.youtube.com/watch?v=K_wnB9ibCMg&t=1078s
    /// Reads user input from stdin line by line
    pub fn read_urls_from_stdin() -> Vec<String> {
        let mut input = String::new();
        let mut urls = Vec::new();

        loop {
            input.clear();
            match std::io::stdin().lock().read_line(&mut input) {
                Ok(0) => break, // EOF reached
                Ok(_) => urls.push(input.trim().to_string()),
                Err(err) => panic!("Error reading from stdin: {}", err),
            }
        }

        urls
    }
}
