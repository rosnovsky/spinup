use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u8,
}

#[derive(Serialize, Debug)]
struct TokenRequest {
    client_id: &'static str,
    device_code: String,
    grant_type: &'static str,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    error: Option<String>,
    access_token: Option<String>,
}

pub async fn request_device_code(
    client_id: &str,
    scope: &str,
    url: &str,
) -> Result<AuthResponse, Box<dyn Error>> {
    let client = Client::new();

    let mut form = HashMap::new();
    form.insert("client_id", client_id);
    form.insert("scope", scope);

    let response = client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("accept", "application/json")
        .form(&form)
        .send()
        .await?;

    if response.status().is_success() {
        match response.json::<AuthResponse>().await {
            Ok(auth_response) => Ok(auth_response),
            Err(e) => Err(Box::new(e)),
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to get valid response: Status code {}",
                response.status()
            ),
        )))
    }
}

async fn request_token(device_code: &str) -> Result<TokenResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let uri = "https://github.com/login/oauth/access_token";
    let parameters = TokenRequest {
        client_id: "Iv23lig5KG3cqmAJZy4E",
        device_code: device_code.to_string(),
        grant_type: "urn:ietf:params:oauth:grant-type:device_code",
    };
    let response = client
        .post(uri)
        .json(&parameters)
        .header("accept", "application/json")
        .send()
        .await?;
    let parsed_response = response.json::<TokenResponse>().await?;
    Ok(parsed_response)
}

pub async fn poll_for_token(device_code: &str, interval: u64) {
    loop {
        let response = request_token(device_code)
            .await
            .expect("Failed to request token");
        match &response.error {
            Some(error) => match error.as_str() {
                "authorization_pending" => {
                    tokio::time::sleep(Duration::from_secs(interval)).await;
                }
                "slow_down" => {
                    tokio::time::sleep(Duration::from_secs(interval + 5)).await;
                }
                "expired_token" => {
                    println!("The device code has expired. Please run `login` again.");
                    std::process::exit(1);
                }
                "access_denied" => {
                    println!("Login cancelled by user.");
                    std::process::exit(1);
                }
                _ => {
                    println!("{:?}", response);
                    std::process::exit(1);
                }
            },
            None => {
                if let Some(access_token) = response.access_token {
                    let mut file = File::create("./.token").expect("Failed to create token file");
                    file.write_all(access_token.as_bytes())
                        .expect("Failed to write token");
                    let mut perms = file.metadata().unwrap().permissions();
                    perms.set_mode(0o600); // Sets file permissions so only the owner can read or write
                    file.set_permissions(perms)
                        .expect("Failed to set permissions");
                    println!("Successfully obtained token.");
                    break;
                }
            }
        }
    }
}
