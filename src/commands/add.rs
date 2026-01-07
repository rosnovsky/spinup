#![allow(dead_code, unused_variables)]
use crate::structs::Config;

pub async fn run_add_app(
    _config: &mut Config,
    _package_name: &str,
    _install_command: &str,
    _os_name: Option<&str>,
    _dependencies: Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Add command is temporarily disabled for v7 migration");
    Ok(())
}

pub async fn run_add_dotfiles(
    _config: &mut Config,
    _repository: &str,
    _target: Option<&str>,
    _os_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Add command is temporarily disabled for v7 migration");
    Ok(())
}

pub async fn run_add_dependency(
    _config: &mut Config,
    _package_name: &str,
    _install_command: &str,
    _os_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Add command is temporarily disabled for v7 migration");
    Ok(())
}

pub async fn save_and_offer_gist_update(
    _config: &Config,
    _config_path: &str,
    _gist_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub async fn update_gist(
    _config: &Config,
    _gist_id: &str,
    _filename: &str,
    _content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
