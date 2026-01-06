mod auth;
mod config;
mod helpers;
mod structs;
mod test_stow;
use helpers::{install_applications, setup_dotfiles};
use serde_json;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    
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
            Ok(content) => {
                match serde_json::from_str::<structs::Config>(&content) {
                    Ok(config) => {
                        let os = helpers::check_current_os(&config.os).await.unwrap();
                        
                        if let Err(e) = helpers::check_applications(&os.applications).await {
                            eprintln!("Error checking applications: {}", e);
                        }

                        if let Some(mut dotfiles) = os.dotfiles.clone() {
                            // Override dry_run from command line
                            if dry_run {
                                dotfiles.dry_run = Some(true);
                                println!("🔍 DRY RUN MODE: No changes will be made\n");
                            }
                            
                            if let Err(e) = setup_dotfiles(&dotfiles).await {
                                eprintln!("Error setting up dotfiles: {}", e);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error parsing config: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading config file: {}", e),
        }
        return;
    }

    helpers::clear_console();
    helpers::print_banner();
    helpers::display_system_info().await;

    let gists = helpers::get_user_gists().await.unwrap();
    let gist = gists.find_by_file_name("config.json").unwrap();
    let url = &gist.files.iter().next().unwrap().1.raw_url;

    match config::fetch_config_from_gist(&url).await {
        Ok(config) => {
            let os = helpers::check_current_os(&config.os).await.unwrap();

            if let Err(e) = helpers::check_applications(&os.applications).await {
                eprintln!("Error checking applications: {}", e);
            }

            install_applications(&os.applications).await.unwrap();

            // Setup dotfiles if configured
            if let Some(dotfiles) = &os.dotfiles {
                if let Err(e) = setup_dotfiles(dotfiles).await {
                    eprintln!("Error setting up dotfiles: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching config: {}", e);
        }
    }
}
