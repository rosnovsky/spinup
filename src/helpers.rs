use crate::structs::{Config, ConfigDiff, Dotfiles, DotfilesDiff, DotfilesStatus, GistList, OsConfig, SystemStatus};
use colored::*;
use figlet_rs::FIGfont;
use prettytable::{format, Cell, Row, Table};
use reqwest::{header, Client};
use std::collections::HashMap;
use std::process::Command;
use std::process::Stdio;
use std::{error::Error, path::Path};
use sysinfo::{Cpu, System, IS_SUPPORTED_SYSTEM};

pub fn is_app_installed(app_package_name: &str) -> bool {
    let output = Command::new("which").arg(app_package_name).output();
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub async fn check_applications_status(
    applications: &[String],
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let mut installed = vec![];
    let mut missing = vec![];

    for app in applications {
        if is_app_installed(app) {
            installed.push(app.clone());
        } else {
            missing.push(app.clone());
        }
    }

    Ok((installed, missing))
}

pub fn display_installation_status(installed: &Vec<String>, missing: &Vec<String>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP);

    let installed_header = format!(
        "{}{}{}",
        "         ".bold(),
        "Installed".underline().bold().green(),
        "         "
    );
    let missing_header = format!(
        "{}{}{}",
        "           ".bold(),
        "Missing".underline().bold().red(),
        "           "
    );

    table.set_titles(Row::new(vec![
        Cell::new(&installed_header),
        Cell::new(&missing_header),
    ]));

    let max_len = std::cmp::max(installed.len(), missing.len());
    for i in 0..max_len {
        let installed_cell = installed
            .get(i)
            .map_or(Cell::new(""), |name| Cell::new(format!("{} {}", "✔".green(), name).as_str()));

        let missing_cell = missing
            .get(i)
            .map_or(Cell::new(""), |name| Cell::new(format!("{} {}", "✖".red(), name).as_str()));
        table.add_row(Row::new(vec![installed_cell, missing_cell]));
    }
    table.printstd();
}

pub fn get_all_configured_apps(config: &Config, os_config: &OsConfig) -> Vec<String> {
    let mut apps = Vec::new();

    // 1. Common packages
    if let Some(common) = &config.common {
        apps.extend(common.packages.clone());
    }

    // 2. Manager packages
    for manager in os_config.manager.values() {
        apps.extend(manager.packages.clone());
    }

    // 3. Tasks
    for task_name in os_config.tasks.keys() {
        apps.push(task_name.clone());
    }

    apps
}

#[allow(dead_code)]
pub fn get_all_configured_dependencies(_os_config: &OsConfig) -> Vec<String> {
    Vec::new() // Dependencies section removed in v7
}

pub async fn check_current_os_name() -> Result<String, Box<dyn Error>> {
    let mut system = System::new_all();
    system.refresh_all();
    let os_name = System::long_os_version().unwrap();
    Ok(os_name)
}

pub fn detect_os_key(os_description: &str) -> &str {
    if os_description.contains("Fedora") || os_description.contains("Red Hat") || os_description.contains("Linux") {
        "fedora"
    } else if os_description.contains("macOS") || os_description.contains("MacOS") {
        "macos"
    } else if os_description.contains("Ubuntu") || os_description.contains("Debian") {
        "ubuntu"
    } else if os_description.contains("Arch") {
        "arch"
    } else {
        ""
    }
}

pub async fn find_matching_os(config: &Config, os_description: &str) -> Option<(String, OsConfig)> {
    let os_key = detect_os_key(os_description);

    if let Some(os_config) = config.os_entries.get(os_key) {
        return Some((os_key.to_string(), os_config.clone()));
    }

    for (key, os_config) in &config.os_entries {
        if let Some(desc) = &os_config.description {
            if os_description.to_lowercase().contains(&desc.to_lowercase())
                || desc.to_lowercase().contains(&os_description.to_lowercase())
            {
                return Some((key.clone(), os_config.clone()));
            }
        }
    }

    None
}

fn categorize_apps(apps: &[String], os_config: &OsConfig, config: &Config) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for app in apps {
        let mut placed = false;
        for (mgr_name, mgr_cfg) in &os_config.manager {
            if mgr_cfg.packages.contains(app) {
                map.entry(mgr_name.clone()).or_insert_with(Vec::new).push(app.clone());
                placed = true;
                break;
            }
        }
        if placed { continue; }

        if os_config.tasks.contains_key(app) {
             map.entry("tasks".to_string()).or_insert_with(Vec::new).push(app.clone());
             placed = true;
        }
        if placed { continue; }

        if let Some(common) = &config.common {
            if common.packages.contains(app) {
                 map.entry("common".to_string()).or_insert_with(Vec::new).push(app.clone());
                 placed = true;
            }
        }
        if placed { continue; }

        map.entry("other".to_string()).or_insert_with(Vec::new).push(app.clone());
    }
    map
}

pub async fn check_applications(applications: &[String]) -> Result<(), Box<dyn Error>> {
    let apps = check_applications_status(applications).await;
    if apps.is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to check applications",
        )));
    }

    let (installed, missing) = apps.unwrap();

    display_installation_status(&installed, &missing);

    Ok(())
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[3J\x1B[H");
}

pub async fn display_system_info() {
    if !IS_SUPPORTED_SYSTEM {
        println!("This system is not supported.");
        return;
    }

    let mut system = System::new_all();
    system.refresh_all();

    let cpu = system.cpus().iter().next().unwrap();
    let brand = Cpu::brand(cpu);
    let model = Cpu::name(cpu);
    let os_release = System::long_os_version().unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);

    table.set_titles(Row::new(vec![
        Cell::new(&format!("{}", "Operating System".blue())),
        Cell::new(&format!("{}", os_release.bright_green())),
    ]));

    table.add_row(Row::new(vec![
        Cell::new(&format!("{}", "Processor".blue())),
        Cell::new(&format!("{} {}", brand.bright_green(), model.bright_green())),
    ]));

    table.printstd();
}

pub fn print_banner() {
    let author_name = "Art Rosnovsky".to_string();
    let author_email = "art@rosnovsky.us".to_string();
    let current_version = "1.0.3";

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("SpinUp");
    println!("{}", figure.unwrap().to_string().green());
    println!(
        "{} {} {} {}{}{}{}",
        "Version".bold(),
        current_version.bright_green(),
        "by".bold(),
        author_name.bold().bright_green(),
        " <".bold(),
        author_email.bold().bright_blue(),
        ">"
    );
}

pub async fn get_gists(token: &str) -> Result<GistList, Box<dyn Error>> {
    let client = Client::new();
    let request_url = "https://api.github.com/gists";

    let response = client
        .get(request_url)
        .header(header::AUTHORIZATION, format!("token {}", token))
        .header(header::USER_AGENT, "spinup")
        .send()
        .await?;

    let status = response.status();

    if status.is_success() {
        let gists = response.json::<GistList>().await?;
        Ok(gists)
    } else {
        match status.as_u16() {
            401 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "401 Unauthorized: Your authentication token is invalid or expired.",
            ))),
            403 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "403 Forbidden: Access denied. This might be due to rate limiting or insufficient permissions.",
            ))),
            404 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "404 Not Found: Unable to find gists.",
            ))),
            429 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "429 Too Many Requests: You are being rate-limited by GitHub. Please wait a moment before trying again.",
            ))),
            500..=599 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("GitHub Server Error: {} - Please try again later.", status),
            ))),
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to fetch gists: {}", status),
            ))),
        }
    }
}

pub async fn install_applications(applications: &[String], config: &Config) -> Result<(), Box<dyn Error>> {
    let os_description = check_current_os_name().await?;
    let Some((_, os_config)) = find_matching_os(config, &os_description).await else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("No matching OS config found for: {}", os_description),
        )));
    };

    let apps_status = check_applications_status(applications).await?;
    let (_, missing) = apps_status;

    for app_name in missing.iter() {
        // 1. Try to find in tasks
        if let Some(task) = os_config.tasks.get(app_name) {
            println!("Running task: {}", app_name);
            execute_install_command(&task.script, app_name).await?;
            continue;
        }

        // 2. Try to find in managers
        let mut found_in_manager = false;
        for (manager_name, manager_config) in &os_config.manager {
            if manager_config.packages.contains(app_name) {
                let flags = manager_config.flags.join(" ");
                // Basic heuristic for install command
                let install_cmd = if manager_name == "brew" {
                    format!("brew install {} {}", flags, app_name)
                } else {
                    format!("sudo {} install {} {}", manager_name, flags, app_name)
                };

                println!("Installing {} via {}...", app_name, manager_name);
                execute_install_command(&install_cmd, app_name).await?;
                found_in_manager = true;
                break;
            }
        }

        if !found_in_manager {
            println!("Could not determine how to install {}", app_name);
        }
    }

    Ok(())
}

async fn execute_install_command(command: &str, app_name: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Empty install command for {}", app_name),
        )));
    }

    let command_name = parts[0];
    let args = &parts[1..];

    let output = Command::new(command_name)
        .args(args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output();

    match output {
        Ok(status) if status.status.success() => Ok(()),
        Ok(status) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Install command exited with code: {:?}", status.status.code()),
        ))),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to execute install command for {}: {}", app_name, e),
        ))),
    }
}

pub fn is_stow_installed() -> bool {
    Command::new("which")
        .arg("stow")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub async fn install_stow() -> Result<(), Box<dyn Error>> {
    println!("{} Installing stow...", "⚙".yellow());

    let os_name = check_current_os_name().await?;

    let install_command = if os_name.contains("Fedora") || os_name.contains("Red Hat") {
        "sudo dnf install -y stow"
    } else if os_name.contains("Ubuntu") || os_name.contains("Debian") {
        "sudo apt update && sudo apt install -y stow"
    } else if os_name.contains("Arch") {
        "sudo pacman -S stow"
    } else if os_name.contains("macOS") {
        "brew install stow"
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS for stow installation"
        )));
    };

    let parts: Vec<&str> = install_command.split_whitespace().collect();
    let command = parts[0];
    let args = &parts[1..];

    let output = Command::new(command)
        .args(args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()?;

    if output.status.success() {
        println!("{} Stow installed successfully", "✓".green());
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to install stow"
        )))
    }
}

pub async fn clone_dotfiles(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
    let target_dir = dotfiles.target_directory
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("dotfiles");

    let home_dir = std::env::var("HOME")?;
    let full_path = format!("{}/{}", home_dir, target_dir);

    if Path::new(&full_path).exists() {
        println!("{} Dotfiles directory already exists at {}", "ⓘ".blue(), full_path);
        return Ok(());
    }

    println!("{} Cloning dotfiles from {}...", "⬇".yellow(), dotfiles.repository);

    let output = Command::new("git")
        .args(&["clone", &dotfiles.repository, &full_path])
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()?;

    if output.success() {
        println!("{} Dotfiles cloned successfully", "✓".green());
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to clone dotfiles repository"
        )))
    }
}

pub async fn apply_dotfiles(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
    let target_dir = dotfiles.target_directory
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("dotfiles");

    let home_dir = std::env::var("HOME")?;
    let dotfiles_path = format!("{}/{}", home_dir, target_dir);

    std::env::set_current_dir(&dotfiles_path)?;

    let packages = match &dotfiles.packages {
        Some(packages) => packages.clone(),
        None => {
            let entries = std::fs::read_dir(".")?;
            let mut discovered_packages = Vec::new();

            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !name.starts_with('.') && name != "README.md" {
                            discovered_packages.push(name.to_string());
                        }
                    }
                }
            }
            discovered_packages
        }
    };

    let dry_run = dotfiles.dry_run.unwrap_or(false);

    if dry_run {
        println!("{} DRY RUN: Would apply dotfiles packages: {}", "🔍".yellow(), packages.join(", "));
    } else {
        println!("{} Applying dotfiles packages: {}", "🔗".yellow(), packages.join(", "));
    }

    for package in &packages {
        if dry_run {
            println!("  {} DRY RUN: Would stow {}...", "→".cyan(), package);

            let output = Command::new("stow")
                .args(&["--no", "-v", package])
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .output()?;

            if output.status.success() {
                println!("  {} {} would be applied successfully", "✓".green(), package);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);

                if stderr.contains("would cause conflicts") || stderr.contains("existing target") {
                    println!("  {} {} would have conflicts with existing files", "⚠".yellow(), package);
                    println!("    {} To resolve: backup existing files or use 'stow --adopt {}'", "💡".blue(), package);
                } else {
                    println!("  {} Would fail to apply {}: {}", "✖".red(), package, stderr.trim());
                }
            }
        } else {
            println!("  {} Stowing {}...", "→".cyan(), package);

            let output = Command::new("stow")
                .args(&["-v", package])
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .output()?;

            if output.status.success() {
                println!("  {} {} applied successfully", "✓".green(), package);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);

                if stderr.contains("would cause conflicts") || stderr.contains("existing target") {
                    println!("  {} {} has conflicts with existing files", "⚠".yellow(), package);
                    println!("    {} To resolve: backup existing files or use 'stow --adopt {}'", "💡".blue(), package);
                    println!("    Skipping {} for now...", package);
                } else {
                    println!("  {} Failed to apply {}: {}", "✖".red(), package, stderr.trim());
                }
            }
        }
    }

    println!("{} All dotfiles applied successfully!", "🎉".green());
    Ok(())
}

pub async fn setup_dotfiles(dotfiles: &Dotfiles) -> Result<(), Box<dyn Error>> {
    println!("\n{} Setting up dotfiles...", "📁".bold());

    if !is_stow_installed() {
        install_stow().await?;
    } else {
        println!("{} Stow is already installed", "✓".green());
    }

    clone_dotfiles(dotfiles).await?;

    apply_dotfiles(dotfiles).await?;

    Ok(())
}

pub async fn get_system_status(config: &Config) -> Result<SystemStatus, Box<dyn Error>> {
    let os_description = check_current_os_name().await?;
    let os_key = detect_os_key(&os_description);

    let Some(os_config) = config.os_entries.get(os_key) else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("No configuration found for OS: {}", os_description),
        )));
    };

    let configured_apps = get_all_configured_apps(config, os_config);
    let (installed, missing) = check_applications_status(&configured_apps).await?;

    let installed_packages = categorize_apps(&installed, os_config, config);
    let missing_packages = categorize_apps(&missing, os_config, config);

    let dotfiles_status = if let Some(dotfiles) = &os_config.dotfiles {
        let target_dir = dotfiles.target_directory
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("dotfiles");

        let home_dir = std::env::var("HOME")?;
        let dotfiles_path = format!("{}/{}", home_dir, target_dir);
        let cloned = Path::new(&dotfiles_path).exists();

        let packages = match &dotfiles.packages {
            Some(pkgs) => pkgs.clone(),
            None => vec![],
        };

        DotfilesStatus {
            cloned,
            applied: false,
            packages,
        }
    } else {
        DotfilesStatus {
            cloned: false,
            applied: false,
            packages: vec![],
        }
    };

    Ok(SystemStatus {
        os_name: os_description,
        installed_packages,
        missing_packages,
        dotfiles_status,
    })
}

pub fn format_status_json(status: &SystemStatus) -> Result<String, Box<dyn Error>> {
    serde_json::to_string_pretty(status).map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub async fn get_config_diff(config: &Config) -> Result<ConfigDiff, Box<dyn Error>> {
    let os_description = check_current_os_name().await?;
    let os_key = detect_os_key(&os_description);

    let Some(os_config) = config.os_entries.get(os_key) else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("No configuration found for OS: {}", os_description),
        )));
    };

    let configured_apps = get_all_configured_apps(config, os_config);
    let (_installed, missing) = check_applications_status(&configured_apps).await?;

    let dotfiles_diff = if let Some(dotfiles) = &os_config.dotfiles {

        let target_dir = dotfiles.target_directory
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("dotfiles");

        let home_dir = std::env::var("HOME")?;
        let dotfiles_path = format!("{}/{}", home_dir, target_dir);
        let needs_clone = !Path::new(&dotfiles_path).exists();

        let packages = match &dotfiles.packages {
            Some(pkgs) => pkgs.clone(),
            None => {
                let mut pkgs = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&dotfiles_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_dir() {
                                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                    if !name.starts_with('.') && name != "README.md" {
                                        pkgs.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                pkgs
            }
        };

        DotfilesDiff {
            needs_clone,
            packages_to_apply: packages.clone(),
            packages_already_applied: vec![],
        }
    } else {
        DotfilesDiff {
            needs_clone: false,
            packages_to_apply: vec![],
            packages_already_applied: vec![],
        }
    };

    let mut missing_categorized = categorize_apps(&missing, os_config, config);
    let tasks_to_run = missing_categorized.remove("tasks").unwrap_or_default();
    let packages_to_install = missing_categorized;

    Ok(ConfigDiff {
        os_name: os_description,
        packages_to_install,
        tasks_to_run,
        dotfiles_diff,
    })
}

pub fn format_diff_json(diff: &ConfigDiff) -> Result<String, Box<dyn Error>> {
    serde_json::to_string_pretty(diff).map_err(|e| Box::new(e) as Box<dyn Error>)
}
