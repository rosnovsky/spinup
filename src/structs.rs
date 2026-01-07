use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ----------------------------------------------------------------------------
// GIST & GITHUB API STRUCTS (Unchanged)
// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
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

// ----------------------------------------------------------------------------
// NEW CONFIGURATION SCHEMA (v7)
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: u8,

    // The [common] section for cross-platform tools
    #[serde(default)]
    pub common: Option<CommonConfig>,

    // Captures [fedora], [macos], etc.
    #[serde(flatten)]
    pub os_entries: HashMap<String, OsConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonConfig {
    #[serde(default)]
    pub packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OsConfig {
    pub description: Option<String>,

    // Replaces the old "applications" mixed bag.
    // Now separated into batchable managers (dnf/brew) and custom tasks.
    #[serde(default)]
    pub manager: HashMap<String, ManagerConfig>,

    #[serde(default)]
    pub tasks: HashMap<String, TaskConfig>,

    pub dotfiles: Option<Dotfiles>,
}

// Represents [os.manager.dnf] or [os.manager.brew]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManagerConfig {
    #[serde(default)]
    pub packages: Vec<String>,

    #[serde(default)]
    pub flags: Vec<String>,

    // If this manager itself needs setup (e.g. brew on linux), link to a task
    pub depends_on: Option<String>,
}

// Represents [os.tasks.kubectl] - complex scripts
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskConfig {
    pub script: String,
    pub description: Option<String>,
    pub depends_on: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dotfiles {
    pub repository: String,

    #[serde(default)]
    pub packages: Option<Vec<String>>,

    // 'alias' allows TOML to use "target" while Rust uses "target_directory"
    #[serde(alias = "target")]
    pub target_directory: Option<String>,

    #[serde(alias = "dry-run")]
    pub dry_run: Option<bool>,
}

// ----------------------------------------------------------------------------
// AUTHENTICATION STRUCTS
// ----------------------------------------------------------------------------

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

// ----------------------------------------------------------------------------
// REPORTING & DIFF STRUCTS (Updated for v7 logic)
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct SystemStatus {
    pub os_name: String,
    // Generalized: "dnf: git, neovim"
    pub installed_packages: HashMap<String, Vec<String>>,
    pub missing_packages: HashMap<String, Vec<String>>,
    pub dotfiles_status: DotfilesStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct DotfilesStatus {
    pub cloned: bool,
    pub applied: bool,
    pub packages: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigDiff {
    pub os_name: String,

    // Batch install commands to run
    // Key = Manager (dnf), Value = List of packages to install
    pub packages_to_install: HashMap<String, Vec<String>>,

    // Complex tasks that need to run
    pub tasks_to_run: Vec<String>,

    pub dotfiles_diff: DotfilesDiff,
}

#[derive(Debug, Clone, Serialize)]
pub struct DotfilesDiff {
    pub needs_clone: bool,
    pub packages_to_apply: Vec<String>,
    pub packages_already_applied: Vec<String>,
}
