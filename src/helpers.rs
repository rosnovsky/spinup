use crate::auth::{self, poll_for_token};
use crate::structs::{GistList, Os, Software};
use colored::*;
use figlet_rs::FIGfont;
use prettytable::{format, Cell, Row, Table};
use reqwest::{header, Client};
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::{error::Error, path::Path};
use sysinfo::{Cpu, System, IS_SUPPORTED_SYSTEM};

/// Check applications installation status.
/// 
/// ## Arguments
/// 
/// * `applications` - A vector of `Software` structs.
///
/// ## Returns
/// This function returns a tuple of two vectors, where the first vector contains the installed applications and the second vector contains the missing applications.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn check_applications_status(
    applications: &[Software],
) -> Result<(Vec<Software>, Vec<Software>), Box<dyn Error>> {
    fn is_installed(app_package_name: &str) -> bool {
        let output = Command::new("which").arg(app_package_name).output();
        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    let mut installed = vec![];
    let mut missing = vec![];

    for app in applications {
        if is_installed(&app.package) {
            installed.push(app.clone());
        } else {
            missing.push(app.clone());
        }
    }

    Ok((installed, missing))
}

/// Displays application installation status.
///
/// ## Arguments
///
/// * `installed` - A vector of `Software` structs representing installed applications.
/// * `missing` - A vector of `Software` structs representing missing applications.
///
/// ## Returns
/// This function returns nothing.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
fn display_installation_status(installed: &Vec<Software>, missing: &Vec<Software>) {
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
            .map_or(Cell::new(""), |Software { name, .. }| {
                Cell::new(format!("{} {}", "✔".green(), name).as_str())
            });

        let missing_cell = missing
            .get(i)
            .map_or(Cell::new(""), |Software { name, .. }| {
                Cell::new(format!("{} {}", "✖".red(), name).as_str())
            });
        table.add_row(Row::new(vec![installed_cell, missing_cell]));
    }
    table.printstd();
}

/// Check current operating system.
///
/// ## Returns
/// This function returns a `System` struct representing the current operating system.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn check_current_os(os: &Vec<Os>) -> Result<Os, Box<dyn Error>> {
    let mut system = System::new_all();
    system.refresh_all();

    let os_name = System::long_os_version().unwrap();

    fn check_os_keywords(system_description: &str, keywords: &[&str]) -> bool {
        keywords
            .iter()
            .any(|keyword| system_description.contains(keyword))
    }

    let fedora = ["Linux", "Fedora"];
    let macos = ["MacOS"];

    for opsys in os {
        if check_os_keywords(&os_name, &fedora) || check_os_keywords(&os_name, &macos) {
            return Ok(opsys.clone());
        }
    }

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to check current OS",
    )))
}

/// Check applications.
/// 
/// ## Arguments
/// 
/// * `applications` - A vector of `Software` structs.
///
/// ## Returns
/// This function returns a `Result` containing either the `Software` structs or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn check_applications(applications: &[Software]) -> Result<(), Box<dyn Error>> {
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

/// Clears the console.
///
/// ## Example
///
/// ```
/// clear_console();
/// ```
/// ## Returns
/// This function returns nothing.
pub fn clear_console() {
    print!("\x1B[2J\x1B[3J\x1B[H");
}

/// Displays system information.
/// 
/// ## Arguments
/// 
/// * `os` - A vector of `Os` structs representing the operating systems.
///
/// ## Returns
/// This function returns nothing.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
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

    // Adding custom column settings and characters can be done via the format API
    table.set_titles(Row::new(vec![
        Cell::new(&format!("{}", "Operating System".blue())),
        Cell::new(&format!("{}", os_release.bright_green())),
    ]));

    table.add_row(Row::new(vec![
        Cell::new(&format!("{}", "Processor".blue())),
        Cell::new(&format!(
            "{} {}",
            brand.bright_green(),
            model.bright_green()
        )),
    ]));

    table.printstd();
}

/// Prints a banner.
///
/// ## Returns
/// This function returns nothing.
pub fn print_banner() {
    let author_name = "Art Rosnovsky".to_string();
    let author_email = "art@rosnovsky.us".to_string();
    let current_version = "1.0.2";

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("SpinUp");
    // assert!(figure.is_some());
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

/// Gets the user's GitHub gists.
///
/// ## Returns
/// This function returns a `Result` containing a `GistList` struct or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn get_user_gists() -> Result<GistList, Box<dyn Error>> {
    fn read_token_from_file(path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    if !Path::new("./.token").exists() {
        let device_code = auth::request_device_code(
            "Iv23lig5KG3cqmAJZy4E",
            "openid profile email offline_access",
            "https://github.com/login/device/code",
        )
        .await
        .unwrap();
        println!(
            "Device code: {:?}\nURL: {}",
            device_code.user_code, device_code.verification_uri
        );
        let interval = 5; // polling interval in seconds
        poll_for_token(device_code.device_code.as_str(), interval).await;
    }

    let client = Client::new();
    let request_url = "https://api.github.com/gists";
    let token = read_token_from_file("./.token")?;

    let response = client
        .get(request_url)
        .header(header::AUTHORIZATION, format!("token {}", token))
        .header(header::USER_AGENT, "reqwest")
        .send()
        .await?;

    if response.status().is_success() {
        let gists = response.json::<GistList>().await?;
        fs::remove_file("./.token")?;
        Ok(gists)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to fetch gists: {}", response.status()),
        )))
    }
}

/// Installs dependencies for an application
///
/// ## Arguments
///
/// * `app` - The application for which to install dependencies.
/// * `missing` - The missing applications.
///
/// ## Returns
/// This function returns a `Result` containing either the `Software` structs or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
fn install_dependencies(app: &Software, missing: &Vec<Software>) -> Result<(), Box<dyn Error>> {
    let mut missing_dependencies = missing.clone();
    missing_dependencies.retain(|x| x.dependencies != app.dependencies);
    if missing_dependencies.len() > 0 {
        println!("Installing dependencies for {}...", app.package);
        let mut dependencies = missing_dependencies.clone();
        dependencies.retain(|x| x.package != app.package);
        install_applications(&dependencies);
    }
    Ok(())
}

/// Installs applications.
///
/// ## Arguments
///
/// * `applications` - A vector of `Software` structs.
///
/// ## Returns
/// This function returns a `Result` containing either the `Software` structs or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn install_applications(applications: &[Software]) -> Result<(), Box<dyn Error>> {
    let apps_status = check_applications_status(applications).await?;
    let (_installed, missing) = apps_status;

    for app in missing.iter() {
        if let Err(e) = install_dependencies(app, &missing) {
            println!("Failed to install dependencies for {}: {}", app.name, e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to install dependencies for {}: {}", app.name, e),
            )));
        }
        if let Err(e) = intsall_app(app) {
            println!("Failed to install {}: {}", app.name, e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to install {}: {}", app.name, e),
            )));
        } else {
            println!("{} installed successfully.", app.name);
        }
    }

    fn intsall_app(app: &Software) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = app.install.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        let output = Command::new(command)
            .args(args)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .status();

        if output.is_ok() {
            println!("{} installed successfully.", app.name);
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to install {}: {}", app.name, output.unwrap_err()),
            )))
        }
    }
    Ok(())
}
