use crate::structs::Config;
use reqwest;
use std::error::Error;

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
