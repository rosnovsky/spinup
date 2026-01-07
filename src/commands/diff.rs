use crate::helpers::{format_diff_json, get_config_diff};
use crate::structs::Config;
use colored::*;

pub async fn run_diff(config: &Config, json_output: bool) -> Result<(), Box<dyn std::error::Error>> {
    let diff = get_config_diff(config).await?;

    if json_output {
        let json = format_diff_json(&diff)?;
        println!("{}", json);
        return Ok(());
    }

    println!("{}", "Configuration Diff".bold().underline());
    println!("Operating System: {}", diff.os_name.blue());
    println!();

    println!("{}", "Applications".bold().underline());
    if diff.packages_to_install.is_empty() && diff.tasks_to_run.is_empty() {
        println!("  {} All applications are installed", "✓".green());
    } else {
        for (manager, packages) in &diff.packages_to_install {
            if !packages.is_empty() {
                println!("  {} Install via {} ({}):", "→".yellow(), manager, packages.len());
                for app in packages {
                    println!("    - {}", app.red());
                }
            }
        }

        if !diff.tasks_to_run.is_empty() {
             println!("  {} Run Tasks ({}):", "→".yellow(), diff.tasks_to_run.len());
             for task in &diff.tasks_to_run {
                 println!("    - {}", task.red());
             }
        }
    }

    println!();
    println!("{}", "Dotfiles".bold().underline());
    let df_diff = &diff.dotfiles_diff;

    if df_diff.needs_clone {
        println!("  {} Dotfiles need to be cloned", "→".yellow());
        println!("    Repository will be cloned and packages applied");
    } else {
        println!("  {} Dotfiles already cloned", "✓".green());
    }

    if !df_diff.packages_to_apply.is_empty() {
        println!();
        println!("  {} Packages to apply ({}):", "→".yellow(), df_diff.packages_to_apply.len());
        for pkg in &df_diff.packages_to_apply {
            println!("    - {}", pkg.cyan());
        }
    }

    println!();
    println!("{}", "Preview: What 'spinup apply' would do".bold().underline());

    if !diff.packages_to_install.is_empty() {
        println!("  1. Install applications:");
        for (manager, packages) in &diff.packages_to_install {
             for app in packages {
                 println!("     $ {} install {}", manager, app); // Simplified preview
             }
        }
    }
    
    if !diff.tasks_to_run.is_empty() {
        println!("  2. Run tasks:");
        for task in &diff.tasks_to_run {
            println!("     $ execute task: {}", task);
        }
    }

    if df_diff.needs_clone || !df_diff.packages_to_apply.is_empty() {
        println!();
        println!("  3. Setup dotfiles:");
        if df_diff.needs_clone {
            println!("     $ Clone dotfiles repository");
        }
        if !df_diff.packages_to_apply.is_empty() {
            for pkg in &df_diff.packages_to_apply {
                println!("     $ stow {}", pkg);
            }
        }
    }

    if diff.packages_to_install.is_empty()
        && diff.tasks_to_run.is_empty()
        && !df_diff.needs_clone
        && df_diff.packages_to_apply.is_empty()
    {
        println!("  {} System is up to date!", "✓".green());
    }

    Ok(())
}
