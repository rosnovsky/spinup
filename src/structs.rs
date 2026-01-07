use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ----------------------------------------------------------------------------
// GIST & GITHUB API STRUCTS (Unchanged)
// ----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    #[allow(dead_code)]
    pub id: u64,
    #[allow(dead_code)]
    pub node_id: String,
    #[allow(dead_code)]
    pub avatar_url: String,
    #[allow(dead_code)]
    pub gravatar_id: Option<String>,
    #[allow(dead_code)]
    pub url: String,
    #[allow(dead_code)]
    pub html_url: String,
    #[allow(dead_code)]
    pub followers_url: String,
    #[allow(dead_code)]
    pub following_url: String,
    #[allow(dead_code)]
    pub gists_url: String,
    #[allow(dead_code)]
    pub starred_url: String,
    #[allow(dead_code)]
    pub subscriptions_url: String,
    #[allow(dead_code)]
    pub organizations_url: String,
    #[allow(dead_code)]
    pub repos_url: String,
    #[allow(dead_code)]
    pub events_url: String,
    #[allow(dead_code)]
    pub received_events_url: String,
    #[allow(dead_code)]
    pub site_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct GistFile {
    #[allow(dead_code)]
    pub filename: String,
    #[allow(dead_code)]
    pub r#type: String,
    #[allow(dead_code)]
    pub language: Option<String>,
    pub raw_url: String,
    #[allow(dead_code)]
    pub size: usize,
    #[allow(dead_code)]
    pub truncated: Option<bool>,
    #[allow(dead_code)]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Gist {
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub node_id: String,
    #[allow(dead_code)]
    pub git_pull_url: String,
    #[allow(dead_code)]
    pub git_push_url: String,
    #[allow(dead_code)]
    pub html_url: String,
    pub files: HashMap<String, GistFile>,
    #[allow(dead_code)]
    pub public: bool,
    #[allow(dead_code)]
    pub raw_url: Option<String>,
    #[allow(dead_code)]
    pub created_at: String,
    #[allow(dead_code)]
    pub updated_at: String,
    #[allow(dead_code)]
    pub description: Option<String>,
    #[allow(dead_code)]
    pub comments: u32,
    #[allow(dead_code)]
    pub user: Option<User>,
    #[allow(dead_code)]
    pub forks_url: String,
    #[allow(dead_code)]
    pub commits_url: String,
    #[allow(dead_code)]
    pub comments_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GistList(pub Vec<Gist>);

impl GistList {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[allow(dead_code)]
    pub expires_in: u64,
    pub interval: u8,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
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
