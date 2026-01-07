use crate::crypto::EncryptionAPI;
use crate::structs::{AuthResponse, TokenRequest, TokenResponse};
use colored::Colorize;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;

const CLIENT_ID: &str = "Iv23lig5KG3cqmAJZy4E";
const SCOPE: &str = "openid profile email offline_access gist repo";
const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

pub async fn request_device_code() -> Result<AuthResponse, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .post(DEVICE_CODE_URL)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("accept", "application/json")
        .form(&[("client_id", CLIENT_ID), ("scope", SCOPE)])
        .send()
        .await?;

    if response.status().is_success() {
        let auth_response = response.json::<AuthResponse>().await?;
        Ok(auth_response)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get device code: Status {}", response.status()),
        )))
    }
}

async fn request_token(device_code: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let parameters = TokenRequest {
        client_id: CLIENT_ID,
        device_code: device_code.to_string(),
        grant_type: "urn:ietf:params:oauth:grant-type:device_code",
    };

    let response = client
        .post(TOKEN_URL)
        .json(&parameters)
        .header("accept", "application/json")
        .send()
        .await?;

    let parsed_response = response.json::<TokenResponse>().await?;
    Ok(parsed_response)
}

pub async fn authenticate_with_caching() -> Result<String, Box<dyn Error>> {
    let mut crypto = EncryptionAPI::new();

    if let Some(cached) = crypto.get_cached_token()? {
        if !cached.is_expired {
            println!("{} Using cached authentication token", "ℹ".blue());
            return Ok(cached.token);
        }
        println!("{} Cached token has expired, re-authenticating...", "ℹ".blue());
        crypto.clear_cached_token()?;
    }

    let device_code = request_device_code().await?;

    println!();
    println!("{} Authentication required", "🔐".yellow());
    println!("  Device code: {}", device_code.user_code.blue());
    println!("  URL: {}", device_code.verification_uri);
    println!();
    println!("  {} Please complete authorization at the URL above", "→".cyan());
    println!("  {} Code: {}", "→".cyan(), device_code.user_code.bold());
    println!();
    println!("{} Scopes requested:", "📋".yellow());
    println!("  - openid, profile, email (identity)");
    println!("  - gist (access your configuration gist)");
    println!("  - repo (read private repositories for dotfiles)");
    println!();

    let interval = device_code.interval as u64;

    println!("{} Polling for authentication...", "⏳".yellow());

    loop {
        tokio::time::sleep(Duration::from_secs(interval)).await;

        let response = request_token(&device_code.device_code).await?;

        match &response.error {
            Some(error) => match error.as_str() {
                "authorization_pending" => {
                    println!("  {} Waiting for authorization...", "⏳".yellow());
                }
                "slow_down" => {
                    println!("  {} Slowing down polling...", "⏳".yellow());
                }
                "expired_token" => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Device code expired. Please run spinup again.",
                    )));
                }
                "access_denied" => {
                    println!("{} Authorization cancelled by user", "ℹ".blue());
                    std::process::exit(0);
                }
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Authentication error: {}", error),
                    )));
                }
            },
            None => {
                if let Some(access_token) = response.access_token {
                    println!("{} Authentication successful!", "✓".green());

                    if let Err(e) = crypto.store_cached_token(&access_token) {
                        println!("{} Warning: Failed to cache token ({}). Will re-authenticate next time.", "⚠".yellow(), e);
                    } else {
                        println!("{} Token cached for 24 hours", "ℹ".blue());
                    }

                    return Ok(access_token);
                }
            }
        }
    }
}

