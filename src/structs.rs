use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: Option<String>,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    site_admin: bool,
    // Additional fields can be added here
}

#[derive(Debug, Deserialize)]
pub struct GistFile {
    pub filename: String,
    pub r#type: String,
    pub language: Option<String>,
    pub raw_url: String,
    pub size: usize,
    pub truncated: Option<bool>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Gist {
    pub id: String,
    pub node_id: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub html_url: String,
    pub files: HashMap<String, GistFile>,
    pub public: bool,
    pub raw_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub comments: u32,
    pub user: Option<User>,
    pub forks_url: String,
    pub commits_url: String,
    pub comments_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GistList(pub Vec<Gist>);

impl GistList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn find_by_file_name(&self, file_name: &str) -> Option<&Gist> {
        self.0
            .iter()
            .find(|gist| gist.files.contains_key(file_name))
    }
}

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
    pub dotfiles: Option<Dotfiles>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dotfiles {
    pub repository: String,
    pub packages: Option<Vec<String>>,
    pub target_directory: Option<String>, // Default: ~/dotfiles
    pub dry_run: Option<bool>, // Default: false
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Software {
    pub name: String,
    pub package: String,
    pub install: String,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u8,
}

#[derive(Serialize, Debug)]
pub struct TokenRequest {
    pub client_id: &'static str,
    pub device_code: String,
    pub grant_type: &'static str,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub error: Option<String>,
    pub access_token: Option<String>,
}
