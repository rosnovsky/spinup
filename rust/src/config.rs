use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: i8,
    pub os: Vec<Os>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Os {
    pub name: String,
    pub applications: Vec<Software>,
    pub fonts: Option<Vec<Software>>,
    pub dependencies: Option<Vec<Software>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Software {
    pub name: String,
    pub package: String,
    pub install: String,
    pub dependencies: Option<Vec<String>>,
}

/// Fetches the config from a GitHub Gist.
///
/// ## Arguments
///
/// * `url` - The URL of the GitHub Gist.
///
/// ## Example
///
/// ```
/// let config = fetch_config_from_gist("https://gist.githubusercontent.com/rosnovsky/0ce4dfd64e11a5f1167b83a72530905d/raw/0fa35a12b687f3c2eb0e62bc8fa138980f8ea682/config.json").await?;
/// ```
///
/// ## Returns
/// This function returns a `Result` containing either the `Config` struct or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn fetch_config_from_gist(url: &str) -> Result<Config, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let config: Config = response.json().await?;
        Ok(config)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch or deserialize config",
        )))
    }
}
