mod auth;
mod commands;
mod config;
mod crypto;
mod helpers;
mod structs;
mod test_stow;
mod tests_v7;
use commands::{run_diff, run_status};
use helpers::{check_current_os_name, find_matching_os};
use structs::Config;
use tokio;

fn print_help() {
    println!(
        r#"SpinUp - Automated System Configuration

USAGE:
    spinup [SUBCOMMAND] [OPTIONS]

SUBCOMMANDS:
    status          Show current system status compared to config
    diff            Show differences between system and config (with preview)
    add             Add new applications or dotfiles to config
    test-stow       Run stow integration tests
    test-config     Test configuration file parsing
    run             Run the full setup (default)

OPTIONS:
    --json          Output in JSON format (for status/diff)
    --dry-run       Preview changes without applying (for add)
    --gist-id ID    Gist ID to update (for add subcommand)
    --help          Show this help message

EXAMPLES:
    spinup status              # Show installed vs missing apps
    spinup status --json       # JSON output for scripting
    spinup diff                # Show what would be installed
    spinup add app git "sudo dnf install -y git"
    spinup add app k9s "brew install k9s" --depends brew
    spinup add dotfiles --repo https://github.com/user/dotfiles.git
"#
    );
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return;
    }

    if args.len() > 1 && args[1] == "test-stow" {
        println!("🧪 Running stow integration tests...\n");
        test_stow::test_stow_detection().await;
        println!();
        test_stow::test_dotfiles_setup().await;
        println!();
        test_stow::test_auto_discovery().await;
        return;
    }

    if args.len() > 2 && args[1] == "test-config" {
        println!("🧪 Running full integration test with config file...\n");
        let config_file = &args[2];
        let dry_run = args.len() > 3 && args[3] == "--dry-run";

        match std::fs::read_to_string(config_file) {
            Ok(content) => match config::parse_config(&content) {
                Ok(config) => {
                    let os_description = check_current_os_name().await.unwrap();
                    let Some((_, os)) = find_matching_os(&config, &os_description).await else {
                        eprintln!("No matching OS found for: {}", os_description);
                        return;
                    };

                    let apps = helpers::get_all_configured_apps(&config, &os);
                    if let Err(e) = helpers::check_applications(&apps).await {
                        eprintln!("Error checking applications: {}", e);
                    }

                    if let Some(mut dotfiles) = os.dotfiles.clone() {
                        if dry_run {
                            dotfiles.dry_run = Some(true);
                            println!("🔍 DRY RUN MODE: No changes will be made\n");
                        }

                        if let Err(e) = helpers::setup_dotfiles(&dotfiles).await {
                            eprintln!("Error setting up dotfiles: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Error parsing config: {}", e),
            },
            Err(e) => eprintln!("Error reading config file: {}", e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "status" {
        let json_output = args.contains(&"--json".to_string());
        let config = load_config().await;
        match config {
            Ok(config) => {
                if let Err(e) = run_status(&config, json_output).await {
                    eprintln!("Error showing status: {}", e);
                }
            }
            Err(e) => eprintln!("Error loading config: {}", e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "diff" {
        let json_output = args.contains(&"--json".to_string());
        let config = load_config().await;
        match config {
            Ok(config) => {
                if let Err(e) = run_diff(&config, json_output).await {
                    eprintln!("Error showing diff: {}", e);
                }
            }
            Err(e) => eprintln!("Error loading config: {}", e),
        }
        return;
    }

    if args.len() > 1 && args[1] == "add" {
        println!("Add subcommand - See --help for usage");
        return;
    }

    helpers::clear_console();
    helpers::print_banner();
    helpers::display_system_info().await;

    let token = match auth::authenticate_with_caching().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Authentication failed: {}", e);
            return;
        }
    };

    let gists = match helpers::get_gists(&token).await {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error fetching gists: {}", e);
            return;
        }
    };

    let gist = match gists.find_by_file_name("config.toml") {
        Some(g) => g,
        None => {
            eprintln!("Error: No config.toml found in your gists");
            eprintln!("Please create a config.toml file in a private gist");
            eprintln!("See docs/schema.toml for the configuration format");
            return;
        }
    };

    let url = &gist.files.iter().next().unwrap().1.raw_url;

    match config::fetch_config_from_gist(&url).await {
        Ok(config) => {
            let os_description = check_current_os_name().await.unwrap();
            let Some((_, os)) = find_matching_os(&config, &os_description).await else {
                eprintln!("No matching OS found for: {}", os_description);
                return;
            };

            let apps = helpers::get_all_configured_apps(&config, &os);
            if let Err(e) = helpers::check_applications(&apps).await {
                eprintln!("Error checking applications: {}", e);
            }

            if let Err(e) = helpers::install_applications(&apps, &config).await {
                eprintln!("Error installing applications: {}", e);
            }

            if let Some(dotfiles) = &os.dotfiles {
                if let Err(e) = helpers::setup_dotfiles(dotfiles).await {
                    eprintln!("Error setting up dotfiles: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching config: {}", e);
        }
    }
}

async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let token = auth::authenticate_with_caching().await?;

    let gists = helpers::get_gists(&token).await?;
    let gist = gists.find_by_file_name("config.toml");

    match gist {
        Some(gist) => {
            let url = &gist.files.iter().next().unwrap().1.raw_url;
            config::fetch_config_from_gist(url).await
        }
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No config.toml found in gists",
        )) as Box<dyn std::error::Error>),
    }
}
