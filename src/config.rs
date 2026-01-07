use crate::structs::Config;
use reqwest;
use std::error::Error;
use std::io;

pub async fn fetch_config_from_gist(url: &str) -> Result<Config, Box<dyn Error>> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let content = response.text().await?;
    parse_config(&content)
}

const OLD_JSON_ERROR: &str = r#"Detected old JSON config format with $schema field.

The new SpinUp version uses TOML configuration. Please migrate your config to TOML format.

Example TOML config:
version = 6

[fedora]
description = "Fedora Linux"

[fedora.applications]
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"

[fedora.dotfiles]
repository = "https://github.com/yourusername/dotfiles.git"

See docs/schema.toml for the full schema reference."#;

pub fn parse_config(content: &str) -> Result<Config, Box<dyn Error>> {
    let trimmed = content.trim_start();

    if trimmed.starts_with("{") {
        let json_result = serde_json::from_str::<serde_json::Value>(content);
        let json_value = match json_result {
            Ok(v) => v,
            Err(e) => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid JSON: {}", e),
                )));
            }
        };

        if json_value.get("$schema").is_some() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                OLD_JSON_ERROR.to_string(),
            )));
        }

        let config = match serde_json::from_value(json_value) {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid config structure: {}", e),
                )));
            }
        };
        Ok(config)
    } else if trimmed.starts_with("[") || trimmed.contains("[") {
        match toml::from_str(content) {
            Ok(c) => Ok(c),
            Err(e) => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid TOML: {}", e),
            ))),
        }
    } else {
        match toml::from_str(content) {
            Ok(c) => Ok(c),
            Err(_) => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Unknown config format. Expected JSON (starts with '{') or TOML.",
            ))),
        }
    }
}

#[allow(dead_code)]
pub fn serialize_config(config: &Config) -> Result<String, Box<dyn Error>> {
    toml::to_string_pretty(config).map_err(|e| Box::new(e) as Box<dyn Error>)
}
