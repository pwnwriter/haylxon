use miette::{Context, IntoDiagnostic, Result};
use std::{
    io::{self, BufRead},
    path::Path,
};

/// Combines URLs with provided ports, generating a new vector of formatted URLs.
///
/// If `ports` are provided, each URL from the `urls` vector will be combined with each port,
/// creating a new vector of formatted URLs. If `ports` are not provided, the function returns
/// the original `urls` vector.
pub fn combine_urls_with_ports(urls: Vec<String>, ports: Option<String>) -> Vec<String> {
    if let Some(ports) = ports {
        if ports.contains("..") {
            let parts: Vec<&str> = ports.split("..").collect();
            if let (Some(start), Some(end)) = (parts.first(), parts.last()) {
                if let (Ok(start_num), Ok(end_num)) = (start.parse::<u32>(), end.parse::<u32>())
                {
                    let port_range: Vec<u32> = (start_num..=end_num).collect();
                    return urls
                        .iter()
                        .flat_map(|url| {
                            port_range
                                .iter()
                                .map(move |port| format!("{}:{}", url, port))
                        })
                        .collect();
                } else {
                    println!("Invalid port range provided");
                }
            }
        }

        let vector_of_strings: Vec<&str> = ports.split(',').collect();
        return urls
            .iter()
            .flat_map(|url| {
                vector_of_strings
                    .iter()
                    .map(move |port| format!("{}:{}", url, port))
            })
            .collect();
    } else {
        urls
    }
}

/// Reads user's input URLs from stdin line by line and optionally combines them with ports.
#[inline]
pub fn read_urls_from_stdin(ports: Option<String>) -> Result<Vec<String>> {
    let urls: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .collect::<Vec<String>>();

    let combined_urls = combine_urls_with_ports(urls, ports);
    Ok(combined_urls)
}

/// Reads URLs from a file and optionally combines them with provided ports.
#[inline]
pub fn read_urls_from_file<T: AsRef<Path>>(
    file_path: T,
    ports: Option<String>,
) -> Result<Vec<String>> {
    let urls = std::fs::read_to_string(&file_path)
        .into_diagnostic()
        .with_context(|| format!("Failed to read file: {:?}", file_path.as_ref()))?;

    let urls_vec: Vec<String> = urls.lines().map(|url| url.to_string()).collect();

    let combined_urls = combine_urls_with_ports(urls_vec, ports);
    Ok(combined_urls)
}

#[cfg(test)]
mod tests {
    use super::combine_urls_with_ports;

    #[test]
    fn test_with_ports() {
        let ports = "1..4".to_string();
        let urls = vec![
            "https://metislinux.org".to_string(),
            "https://pwnwriter.xyz".to_string(),
        ];
        let expected_urls = vec![
            "https://metislinux.org:1",
            "https://metislinux.org:2",
            "https://metislinux.org:3",
            "https://metislinux.org:4",
            "https://pwnwriter.xyz:1",
            "https://pwnwriter.xyz:2",
            "https://pwnwriter.xyz:3",
            "https://pwnwriter.xyz:4",
        ];

        let combined_urls = combine_urls_with_ports(urls, Some(ports));
        for url in &combined_urls {
            println!("{}", url);
        }

        assert_eq!(combined_urls, expected_urls);
    }

    #[test]
    fn test_no_ports() {
        let urls = vec![
            "https://example.com".to_string(),
            "https://test.org".to_string(),
        ];
        let result = combine_urls_with_ports(urls.clone(), None);
        assert_eq!(result, urls);
    }
}
