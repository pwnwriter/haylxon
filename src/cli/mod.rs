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

    #[allow(dead_code)]
    pub fn read_urls_from_file(url: &Option<String>) -> Vec<String> {
        let mut urls = Vec::new();

        if let Some(url) = url {
            if std::path::Path::new(url).exists() {
                if let Ok(file) = std::fs::File::open(url) {
                    let lines = std::io::BufReader::new(file).lines().map_while(Result::ok);
                    urls = lines.collect();
                } else {
                    urls = vec![url.clone()];
                }
            } else {
                urls = vec![url.clone()];
            }
        }

        urls
    }

    #[allow(dead_code)]
    pub fn read_ports() {}
}
