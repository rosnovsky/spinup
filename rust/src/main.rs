mod auth;
mod config;
mod helpers;
mod structs;
use helpers::install_applications;
use tokio;

#[tokio::main]
async fn main() {
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
        }
        Err(e) => {
            eprintln!("Error fetching config: {}", e);
        }
    }
}
