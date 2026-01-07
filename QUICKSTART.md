# SpinUp Quick Start Guide

SpinUp is a CLI tool for automatically setting up new Linux computers with your applications, dependencies, and dotfiles configurations.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- A `config.json` "secret" gist in your GitHub account following the [schema](./src/schema.json)
- Git (for dotfiles functionality)

## Basic Usage

### 1. Build and Run SpinUp

```bash
# Clone the repository
git clone https://github.com/rosnovsky/spinup.git
cd spinup

# Build the project
cargo build --release

# Run SpinUp
./target/release/spinup
```

SpinUp will:
- Authenticate with GitHub to access your config gist
- Check your system information
- Install missing applications
- Set up your dotfiles (if configured)

### 2. Configuration Structure

Your GitHub gist should contain a `config.json` file with this structure:

```json
{
  "$schema": "./src/schema.json",
  "version": 2,
  "os": [
    {
      "name": "Fedora Linux",
      "applications": [
        {
          "name": "Git",
          "package": "git",
          "install": "sudo dnf install -y git"
        },
        {
          "name": "Tmux",
          "package": "tmux",
          "install": "sudo dnf install -y tmux"
        }
      ],
      "fonts": [
        {
          "name": "Fira Code",
          "package": "fira-code-fonts",
          "install": "sudo dnf install -y fira-code-fonts"
        }
      ],
      "dependencies": [
        {
          "name": "stow",
          "package": "stow",
          "install": "sudo dnf install -y stow"
        }
      ],
      "dotfiles": {
        "repository": "https://github.com/yourusername/dotfiles.git",
        "packages": ["zsh", "tmux", "nvim", "git"],
        "target_directory": "dotfiles",
        "dry_run": false
      }
    }
  ]
}
```

## Dotfiles Integration (New!)

SpinUp now includes powerful dotfiles management using GNU Stow.

### Setting Up Dotfiles

1. **Structure your dotfiles repository** using stow packages:
   ```
   dotfiles/
   ├── zsh/
   │   └── .zshrc
   ├── tmux/
   │   └── .tmux.conf
   ├── nvim/
   │   └── .config/
   │       └── nvim/
   │           └── init.vim
   └── git/
       └── .gitconfig
   ```

2. **Configure dotfiles in your config.json**:
   ```json
   "dotfiles": {
     "repository": "https://github.com/yourusername/dotfiles.git",
     "packages": ["zsh", "tmux", "nvim", "git"],
     "target_directory": "dotfiles",
     "dry_run": false
   }
   ```

### Dotfiles Configuration Options

- **`repository`** (required): Git URL or local path to your dotfiles
- **`packages`** (optional): Specific stow packages to apply. If omitted, auto-discovers all packages
- **`target_directory`** (optional): Directory name for cloned dotfiles (default: "dotfiles")
- **`dry_run`** (optional): Preview changes without applying them (default: false)

### Auto-Discovery Feature

If you don't specify `packages`, SpinUp will automatically discover all stow packages in your dotfiles repository:

```json
"dotfiles": {
  "repository": "https://github.com/yourusername/dotfiles.git"
}
```

This will apply ALL packages found in your dotfiles repo (excluding hidden directories like `.git`).

## Dry-Run Mode

Test your configuration safely before making changes:

### Method 1: Config-based
```json
"dotfiles": {
  "repository": "https://github.com/yourusername/dotfiles.git",
  "packages": ["zsh", "tmux"],
  "dry_run": true
}
```

### Method 2: Command-line (for testing)
```bash
./target/debug/spinup test-config my-config.json --dry-run
```

Dry-run mode will show you exactly what would happen without making any changes to your system.

## Conflict Handling

If SpinUp detects conflicts with existing files, it will:
- Show you which files conflict
- Provide guidance on how to resolve them
- Continue with non-conflicting packages

Example output:
```
⚠ tmux has conflicts with existing files
💡 To resolve: backup existing files or use 'stow --adopt tmux'
```

## What SpinUp Does Automatically

1. **System Check**: Identifies your OS and architecture
2. **Application Check**: Shows installed vs missing applications
3. **Application Installation**: Installs missing applications using your specified commands
4. **Stow Installation**: Automatically installs GNU Stow if needed
5. **Dotfiles Clone**: Clones your dotfiles repository
6. **Configuration Apply**: Uses stow to symlink your configurations
7. **Conflict Management**: Handles existing file conflicts gracefully

## Supported Operating Systems

- ✅ Fedora Linux
- ✅ Ubuntu/Debian (partial support)
- ✅ Arch Linux (partial support)  
- ✅ macOS (partial support)

## Examples

### Minimal Configuration
```json
{
  "version": 2,
  "os": [{
    "name": "Fedora Linux",
    "applications": [
      {"name": "Git", "package": "git", "install": "sudo dnf install -y git"}
    ],
    "dotfiles": {
      "repository": "https://github.com/yourusername/dotfiles.git"
    }
  }]
}
```

### Full Configuration with Dry-Run
```json
{
  "version": 2,
  "os": [{
    "name": "Fedora Linux",
    "applications": [
      {"name": "Git", "package": "git", "install": "sudo dnf install -y git"},
      {"name": "Tmux", "package": "tmux", "install": "sudo dnf install -y tmux"},
      {"name": "Neovim", "package": "nvim", "install": "sudo dnf install -y neovim"}
    ],
    "fonts": [
      {"name": "FiraCode", "package": "fira-code-fonts", "install": "sudo dnf install -y fira-code-fonts"}
    ],
    "dependencies": [
      {"name": "Stow", "package": "stow", "install": "sudo dnf install -y stow"}
    ],
    "dotfiles": {
      "repository": "https://github.com/yourusername/dotfiles.git",
      "packages": ["zsh", "tmux", "nvim", "git", "alacritty"],
      "target_directory": "my-dotfiles",
      "dry_run": true
    }
  }]
}
```

## Troubleshooting

### Common Issues

1. **Stow conflicts**: Backup existing config files or use `stow --adopt package-name`
2. **Missing applications**: Check your package manager and install commands
3. **Permission errors**: Ensure you have sudo access for system package installation
4. **Git authentication**: Make sure you can access your dotfiles repository

### Testing Your Configuration

Use the built-in test commands:
```bash
# Test stow integration
cargo run -- test-stow

# Test with your config file
cargo run -- test-config your-config.json

# Test with dry-run
cargo run -- test-config your-config.json --dry-run
```

## Next Steps

1. Create your dotfiles repository using stow structure
2. Set up your `config.json` gist on GitHub
3. Test with dry-run mode first
4. Run SpinUp on your new machine!

For more details, see the [schema documentation](./src/schema.json) and [examples](./test-*.json).