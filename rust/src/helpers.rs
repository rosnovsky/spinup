//! This module contains helper functions for the `spinup` CLI.
//! It provides functions for checking the system, displaying system information, and displaying installed and missing applications.

// TODO: Is it customary in Rust to split up the helper functions into separate modules?

use crate::config::Software;
use colored::*;
use figlet_rs::FIGfont;
use futures::stream::{self, StreamExt};
use prettytable::{format, Cell, Row, Table};
use std::error::Error;
use std::process::Command;
use sys_info;

/// Checks whether applications are installed on the system.
///
/// ## Arguments
///
/// * `applications` - A vector of `Software` structs representing the applications to check.
///
/// ## Example
///
/// ```
/// let applications = vec![
///     Software {
///         name: "make".to_string(),
///         package: "make".to_string(),
///         install: vec![InstallCommand {
///             distro: "Fedora Linux".to_string(),
///             command: "sudo dnf install -y make".to_string(),
///         }],
///         dependencies: None,
///     },
///     Software {
///         name: "gcc".to_string(),
///         package: "gcc".to_string(),
///         install: vec![InstallCommand {
///             distro: "Fedora Linux".to_string(),
///             command: "sudo dnf install -y gcc".to_string(),
///         }],
///         dependencies: None,
///     },
/// ];
/// let installed = check_applications(&applications).await?;
/// ```
///
/// ## Returns
/// This function returns a `Result` containing either the `Config` struct or an `Error` if the request fails.
///
/// ## Errors
/// If the request fails, this function returns an `Error` with a descriptive message.
pub async fn check_applications(
    applications: &[Software],
) -> Result<Vec<Software>, Box<dyn Error>> {
    fn is_installed(app_name: &str) -> bool {
        let output = Command::new("which").arg(app_name).output();

        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn display_installation_status(installed: &[String], missing: &[String]) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP);

        table.set_titles(Row::new(vec![
            Cell::new("        Installed        "),
            Cell::new("           Missing          "),
        ]));

        let max_len = std::cmp::max(installed.len(), missing.len());
        for i in 0..max_len {
            let installed_cell = installed
                .get(i)
                .map_or(Cell::new(""), |name| Cell::new(name));
            let missing_cell = missing.get(i).map_or(Cell::new(""), |name| Cell::new(name));

            table.add_row(Row::new(vec![installed_cell, missing_cell]));
        }
        table.printstd();
    }

    let mut installed = vec![];
    let mut missing = vec![];

    for app in applications {
        let check = is_installed(&app.name);
        if check {
            installed.push(format!("{} {}", "✔".green(), app.name));
        } else {
            missing.push(format!("{} {}", "✖".red(), app.name));
        }
    }

    display_installation_status(&installed, &missing);

    let missing_apps = stream::iter(applications.iter())
        .filter_map(|app| async move {
            let check = is_installed(&app.name);
            if !check {
                Some(app.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .await;

    Ok(missing_apps)
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

pub async fn display_system_info() {
    let cpu_speed = sys_info::cpu_speed().unwrap();
    let os_release = sys_info::linux_os_release()
        .unwrap()
        .pretty_name()
        .to_owned();
    // let kernel = sys_info::os_release().unwrap().to_owned();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);

    // Adding custom column settings and characters can be done via the format API
    table.set_titles(Row::new(vec![
        Cell::new(&format!("{}", "Operating System".blue())),
        Cell::new(&format!("{}", os_release.bright_green())),
    ]));

    table.add_row(Row::new(vec![
        Cell::new(&format!("{}", "Processor".blue())),
        Cell::new(&format!("{} MHz", cpu_speed.to_string().bright_green())),
    ]));

    table.printstd(); // Prints the table to stdout
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
