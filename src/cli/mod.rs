pub mod args;
mod ascii;
pub mod exec;
pub mod screenshot;

pub mod hxn_helper {
    use anyhow::{Context, Result};
    use std::{
        io::{self, BufRead},
        path::Path,
    };

    /// Combines URLs with provided ports, generating a new vector of formatted URLs.
    ///
    /// If `ports` are provided, each URL from the `urls` vector will be combined with each port,
    /// creating a new vector of formatted URLs. If `ports` are not provided, the function returns
    /// the original `urls` vector.
    ///
    /// # Arguments
    ///
    /// * `urls` - A vector containing URLs to be combined with ports if provided.
    /// * `ports` - An optional string containing ports separated by commas.
    ///
    /// # Returns
    ///
    /// A vector of strings containing formatted URLs combined with ports or the original URLs.
    ///
    pub fn combine_urls_with_ports(urls: Vec<String>, ports: Option<String>) -> Vec<String> {
        if let Some(ports) = ports {
            let vector_of_strings: Vec<&str> = ports.split(',').collect();
            urls.iter()
                .flat_map(|url| {
                    vector_of_strings
                        .iter()
                        .map(move |port| format!("{}:{}", url, port))
                })
                .collect()
        } else {
            urls
        }
    }

    /// Reads user's input URLs from stdin line by line and optionally combines them with ports.
    ///
    /// This function reads URLs entered by the user from standard input, processing them line by
    /// line. If `ports` are provided, it combines each URL with the specified ports. Returns a
    /// vector of processed URLs.
    ///
    /// # Arguments
    ///
    /// * `ports` - An optional string containing ports separated by commas.
    ///
    /// # Returns
    ///
    /// A result containing a vector of processed URLs or an error if stdin reading fails.
    ///
    #[inline]
    pub fn read_urls_from_stdin(ports: Option<String>) -> anyhow::Result<Vec<String>> {
        let urls: Vec<String> = io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().trim().to_owned())
            .collect::<Vec<String>>();

        let combined_urls = combine_urls_with_ports(urls, ports);
        Ok(combined_urls)
    }

    /// Reads URLs from a file and optionally combines them with provided ports.
    ///
    /// This function reads URLs from a file specified by `file_path`, processing each line as a URL.
    /// If `ports` are provided, it combines each URL with the specified ports. Returns a vector of
    /// processed URLs.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A path to the file containing URLs to be processed.
    /// * `ports` - An optional string containing ports separated by commas.
    ///
    /// # Returns
    ///
    /// A result containing a vector of processed URLs or an error if file reading fails.
    ///
    #[inline]
    pub fn read_urls_from_file<T: AsRef<Path>>(
        file_path: T,
        ports: Option<String>,
    ) -> Result<Vec<String>> {
        let urls = std::fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path.as_ref()))?;

        let urls_vec: Vec<String> = urls.lines().map(|url| url.to_string()).collect();

        let combined_urls = combine_urls_with_ports(urls_vec, ports);
        Ok(combined_urls)
    }
}
