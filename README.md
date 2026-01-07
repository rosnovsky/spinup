# SpinUp - Automated Linux System Setup

SpinUp is a command line tool to automatically set up new Linux computers with your applications, dependencies, fonts, and **dotfiles configurations**. It's designed to get you from a fresh install to a fully configured development environment in minutes.

## 🚀 New: Dotfiles Integration

SpinUp now includes **comprehensive dotfiles management** using GNU Stow:
- **Automatic stow installation** and setup
- **Repository cloning** (GitHub, local paths)
- **Smart package management** with conflict detection
- **Dry-run mode** for safe configuration testing
- **Auto-discovery** of stow packages

## Quick Start

**👉 See the [Quick Start Guide](./QUICKSTART.md) for complete setup instructions and examples.**

## Features

- ✅ **Application Management**: Install missing applications and dependencies
- ✅ **Font Installation**: Set up development fonts automatically  
- ✅ **Dotfiles Integration**: Full stow-based configuration management
- ✅ **Dry-Run Mode**: Preview changes before applying them
- ✅ **Conflict Detection**: Smart handling of existing configurations
- ✅ **Multi-OS Support**: Fedora, Ubuntu, Arch, macOS
- ✅ **GitHub Integration**: Configuration via private gists

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- A `config.json` "secret" gist in your GitHub account following the [schema](./src/schema.json)
- Git (for dotfiles functionality)

## Basic Usage

```bash
# Clone and build
git clone https://github.com/rosnovsky/spinup.git
cd spinup
cargo build --release

# Run SpinUp
./target/release/spinup
```

For detailed configuration examples and dotfiles setup, see the [Quick Start Guide](./QUICKSTART.md).
