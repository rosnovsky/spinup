mod config;
mod helpers;
use tokio;

#[tokio::main]
async fn main() {
    helpers::clear_console();
    helpers::print_banner();
    helpers::display_system_info().await;
    let url = "https://gist.githubusercontent.com/rosnovsky/0ce4dfd64e11a5f1167b83a72530905d/raw/0fa35a12b687f3c2eb0e62bc8fa138980f8ea682/config.json";

    match config::fetch_config_from_gist(url).await {
        Ok(config) => {
            if let Err(e) = helpers::check_applications(&config.applications).await {
                eprintln!("Error checking applications: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error fetching config: {}", e);
        }
    }
}
