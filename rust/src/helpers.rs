use crate::auth::{self, poll_for_token};
use crate::structs::{GistList, Os, Software};
use colored::*;
use figlet_rs::FIGfont;
use prettytable::{format, Cell, Row, Table};
use reqwest::{header, Client};
use std::fs;
use std::process::Command;
use std::{error::Error, path::Path};
use sysinfo::{Cpu, System, IS_SUPPORTED_SYSTEM};

pub async fn check_applications_status(
    applications: &[Software],
) -> Result<(Vec<Software>, Vec<Software>), Box<dyn Error>> {
    fn is_installed(app_name: &str) -> bool {
        let output = Command::new("which").arg(app_name).output();
        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    let mut installed = vec![];
    let mut missing = vec![];

    for app in applications {
        if is_installed(&app.name) {
            installed.push(app.clone());
        } else {
            missing.push(app.clone());
        }
    }

    Ok((installed, missing))
}

fn display_installation_status(installed: &Vec<Software>, missing: &Vec<Software>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP);

    let installed_header = format!(
        "{}{}{}",
        "        ".bold(),
        "Installed".underline().bold().green(),
        "        "
    );
    let missing_header = format!(
        "{}{}{}",
        "        ".bold(),
        "Missing".underline().bold().red(),
        "        "
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

// TODO: Should take in os name and cpu name as arguments and only be responsible for displaying the system info
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

pub fn print_banner() {
    let author_name = "Art Rosnovsky".to_string();
    let author_email = "art@rosnovsky.us".to_string();
    let current_version = "1.0.1";

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
