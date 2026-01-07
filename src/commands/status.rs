use crate::helpers::{format_status_json, get_system_status};
use crate::structs::Config;
use colored::*;
use prettytable::{format, Cell, Row, Table};

pub async fn run_status(config: &Config, json_output: bool) -> Result<(), Box<dyn std::error::Error>> {
    let status = get_system_status(config).await?;

    if json_output {
        let json = format_status_json(&status)?;
        println!("{}", json);
        return Ok(());
    }

    println!("{}", "System Status".bold().underline());
    println!("Operating System: {}", status.os_name.blue());
    println!();

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

    let installed_flat: Vec<String> = status.installed_packages.values().flatten().cloned().collect();
    let missing_flat: Vec<String> = status.missing_packages.values().flatten().cloned().collect();

    let max_len = std::cmp::max(installed_flat.len(), missing_flat.len());
    for i in 0..max_len {
        let installed_cell = installed_flat
            .get(i)
            .map_or(Cell::new(""), |name| Cell::new(format!("{} {}", "✔".green(), name).as_str()));

        let missing_cell = missing_flat
            .get(i)
            .map_or(Cell::new(""), |name| Cell::new(format!("{} {}", "✖".red(), name).as_str()));

        table.add_row(Row::new(vec![installed_cell, missing_cell]));
    }
    table.printstd();

    println!();
    println!("{}", "Dotfiles Status".bold().underline());
    let df_status = &status.dotfiles_status;
    println!("  Cloned: {}", if df_status.cloned { "✓ Yes".green() } else { "✖ No".red() });
    println!("  Packages: {}", if df_status.packages.is_empty() {
        "Auto-discovery".to_string()
    } else {
        df_status.packages.join(", ")
    });

    println!();
    println!("{}", "Summary".bold().underline());
    let total_installed: usize = status.installed_packages.values().map(|v| v.len()).sum();
    let total_missing: usize = status.missing_packages.values().map(|v| v.len()).sum();

    println!("  Total configured apps: {}", total_installed + total_missing);
    println!("  Installed: {}", total_installed.to_string().green());
    println!("  Missing: {}", total_missing.to_string().red());

    Ok(())
}
