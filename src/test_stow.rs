use crate::helpers::{is_stow_installed, setup_dotfiles};
use crate::structs::Dotfiles;

pub async fn test_stow_detection() {
    println!("Testing stow detection...");
    let stow_installed = is_stow_installed();
    println!("Stow installed: {}", stow_installed);
}

pub async fn test_dotfiles_setup() {
    println!("Testing dotfiles setup with specific packages...");
    
    let test_dotfiles = Dotfiles {
        repository: "/home/rosnovsky/code/dotfiles".to_string(),
        packages: Some(vec!["alacritty".to_string(), "kitty".to_string()]),
        target_directory: Some("test-dotfiles-spinup".to_string()),
        dry_run: Some(false),
    };
    
    match setup_dotfiles(&test_dotfiles).await {
        Ok(()) => println!("✓ Dotfiles setup with specific packages completed successfully"),
        Err(e) => println!("✗ Dotfiles setup failed: {}", e),
    }
}

pub async fn test_auto_discovery() {
    println!("Testing auto-discovery of packages...");
    
    let test_dotfiles = Dotfiles {
        repository: "/home/rosnovsky/code/dotfiles".to_string(),
        packages: None, // Should auto-discover all packages
        target_directory: Some("test-dotfiles-auto".to_string()),
        dry_run: Some(false),
    };
    
    match setup_dotfiles(&test_dotfiles).await {
        Ok(()) => println!("✓ Auto-discovery setup completed successfully"),
        Err(e) => println!("✗ Auto-discovery setup failed: {}", e),
    }
}