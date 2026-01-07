# SpinUp Quick Start Guide

SpinUp is a CLI tool for automatically configuring systems with applications, dependencies, and dotfiles.

## What's New in v0.2

- **TOML Configuration**: Cleaner, simpler config format
- **New Commands**: `status`, `diff`, and `add` subcommands
- **JSON Output**: `--json` flag for scripting integration

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- A `config.toml` or `config.json` secret gist on GitHub
- Git (for dotfiles functionality)

## Quick Start

### 1. Build SpinUp

```bash
git clone https://github.com/rosnovsky/spinup.git
cd spinup
cargo build --release
```

### 2. Create Your Configuration

Create a `config.toml` file in a GitHub gist:

```toml
version = 6

[fedora]
description = "Fedora Linux development environment"

[fedora.applications]
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"
tmux = "sudo dnf install -y tmux"
fzf = "sudo dnf install -y fzf"
ripgrep = "sudo dnf install -y ripgrep"
bat = "sudo dnf install -y bat"

[fedora.dependencies]
zsh = "sudo dnf install -y zsh"
brew = "/bin/zsh -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""

[fedora.dotfiles]
repository = "https://github.com/yourusername/dotfiles.git"
target = "dotfiles"
```

### 3. Run SpinUp

```bash
./target/release/spinup
```

SpinUp will:

- Authenticate with GitHub to access your config gist
- Check your system information
- Show installed vs missing applications
- Install missing applications
- Set up your dotfiles (if configured)

## New Commands

### Check System Status

```bash
# Human-readable output
spinup status

# JSON output for scripting
spinup status --json
```

### See What Would Change

```bash
# Show diff with preview
spinup diff

# JSON output
spinup diff --json
```

The `diff` command shows:

- Applications that need to be installed
- Extra applications installed but not in config
- Dependencies to install
- Dotfiles status and preview

### Add to Configuration

```bash
# Add an application
spinup add app git "sudo dnf install -y git"

# Add application with dependencies
spinup add app k9s "brew install derailed/k9s/k9s" --depends brew

# Add dotfiles configuration
spinup add dotfiles --repo https://github.com/user/dotfiles.git
spinup add dotfiles --repo https://github.com/user/dotfiles.git --target mydots

# Specify OS explicitly
spinup add app git "sudo apt install -y git" --os ubuntu
```

### Test Configuration

```bash
# Test with local config file
spinup test-config config.toml
spinup test-config config.toml --dry-run

# Test stow integration
spinup test-stow
```

## Configuration Reference

### Root Level

| Field       | Type    | Required | Description                         |
| ----------- | ------- | -------- | ----------------------------------- |
| `version`   | integer | Yes      | Configuration version (currently 6) |
| `[os_name]` | table   | Yes      | OS-specific configuration           |

### OS Configuration

```toml
[fedora]
description = "Fedora Linux development environment"

[fedora.applications]
# Simple format: package = "install command"
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"

# Extended format for dependencies
[fedora.applications.k9s]
install = "brew install derailed/k9s/k9s"
depends = ["brew"]

[fedora.dependencies]
zsh = "sudo dnf install -y zsh"

[fedora.dotfiles]
repository = "https://github.com/user/dotfiles.git"
target = "dotfiles"
```

### Dotfiles Configuration

| Field        | Type   | Description                                        |
| ------------ | ------ | -------------------------------------------------- |
| `repository` | string | Git URL to dotfiles repository                     |
| `target`     | string | Directory name (default: "dotfiles")               |
| `packages`   | array  | Specific stow packages (auto-discovers if omitted) |

## Dotfiles Repository Structure

Structure your dotfiles repository with stow packages:

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

## Supported Operating Systems

- Fedora Linux
- Ubuntu/Debian
- Arch Linux
- macOS

## Troubleshooting

### Stow Conflicts

```bash
# Backup and resolve conflicts
stow --adopt package-name
```

### Testing Configuration

```bash
# Dry-run mode
spinup test-config config.toml --dry-run

# Check what would happen
spinup diff
```

### Git Authentication

Ensure your dotfiles repository is accessible:

- Use HTTPS with a personal access token, or
- Set up SSH keys for GitHub

## Next Steps

1. Create your dotfiles repository with stow structure
2. Create a `config.toml` gist on GitHub
3. Test with `spinup diff` to preview
4. Run SpinUp on your new machine!

## More Information

- [TOML Schema Reference](./src/schema.toml)
- [Example Configuration](./src/config.toml)
