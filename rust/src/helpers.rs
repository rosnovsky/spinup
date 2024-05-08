//! This module contains helper functions for the `spinup` CLI.
//! It provides functions for checking the system, displaying system information, and displaying installed and missing applications.

// TODO: Is it customary in Rust to split up the helper functions into separate modules?

use crate::config::Software;
use colored::*;
use figlet_rs::FIGfont;
use prettytable::{format, Cell, Row, Table};
use std::error::Error;
use std::process::Command;
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
