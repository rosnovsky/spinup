# SpinUp - Automated System Configuration

SpinUp is a command line tool to automatically set up new systems with your applications, dependencies, and **dotfiles configurations**. It's designed to get you from a fresh install to a fully configured development environment in minutes.

## Features

- **TOML Configuration**: Clean, human-readable config format
- **Application Management**: Install missing applications and dependencies
- **Dotfiles Integration**: Full stow-based configuration management
- **Status & Diff Commands**: Preview what will be installed before applying
- **Multi-OS Support**: Fedora, Ubuntu, Arch, macOS
- **GitHub Gist Integration**: Configuration via private gists

## Quick Start

**👉 See the [Quick Start Guide](./QUICKSTART.md) for complete setup instructions and examples.**

## Installation

```bash
# Clone and build
git clone https://github.com/rosnovsky/spinup.git
cd spinup
cargo build --release

# Run SpinUp
./target/release/spinup
```

## New CLI Commands

```bash
# Check system status
spinup status
spinup status --json       # Machine-readable output

# See what would be installed
spinup diff                # Shows diff with preview
spinup diff --json         # JSON output for scripting

# Add to configuration
spinup add app git "sudo dnf install -y git"
spinup add app k9s "brew install k9s" --depends brew
spinup add dotfiles --repo https://github.com/user/dotfiles.git

# Test configuration
spinup test-config config.toml    # Test local config file
spinup test-stow                  # Test stow integration
```

## Configuration

SpinUp now uses **TOML** for configuration. Create a `config.toml` file in a GitHub gist:

```toml
version = 6

[fedora]
description = "Fedora Linux development environment"

[fedora.applications]
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"
tmux = "sudo dnf install -y tmux"

[fedora.applications.k9s]
install = "brew install derailed/k9s/k9s"
depends = ["brew"]

[fedora.dependencies]
zsh = "sudo dnf install -y zsh"

[fedora.dotfiles]
repository = "https://github.com/yourusername/dotfiles.git"
target = "dotfiles"
```

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- A `config.toml` or `config.json` "secret" gist in your GitHub account
- Git (for dotfiles functionality)

## Documentation

- [Quick Start Guide](./QUICKSTART.md) - Getting started
- [TOML Schema](./src/schema.toml) - Complete schema reference
- [Example Config](./src/config.toml) - Real-world configuration
