pub mod args;
mod ascii;
pub mod exec;
pub mod screenshot;

pub mod hxn_helper {
    use std::{io, path::Path};

    /// Reads user's input from stdin line by line.
    #[inline]
    pub fn read_urls_from_stdin() -> anyhow::Result<Vec<String>> {
        Ok(io::read_to_string(io::stdin().lock())?
            .lines()
            .map(|url| url.trim().to_owned())
            .collect())
    }

    /// Reads URLs from a file.
    #[inline]
    pub fn read_urls_from_file<T: AsRef<Path>>(file_path: T) -> anyhow::Result<Vec<String>> {
        Ok(std::fs::read_to_string(file_path)?
            .lines()
            .map(|url| url.trim().to_owned())
            .collect())
    }
}
