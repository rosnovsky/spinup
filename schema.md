# SpinUp TOML Configuration Schema

SpinUp now uses TOML for configuration files. This document describes the schema and provides examples.

## Quick Example

```toml
version = 6

[fedora]
description = "Fedora Linux development environment"

[fedora.applications]
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"

[fedora.applications.k9s]
install = "brew install derailed/k9s/k9s"
depends = ["brew"]

[fedora.dependencies]
zsh = "sudo dnf install -y zsh"
brew = "/bin/zsh -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""

[fedora.dotfiles]
repository = "git@forge.unfuck.cloud:rosnovsky/dotfiles.git"
target = "dotfiles"
```

## Schema Reference

### Root Level

| Field       | Type    | Required | Description                                |
| ----------- | ------- | -------- | ------------------------------------------ |
| `version`   | integer | Yes      | Configuration version number (currently 6) |
| `[os_name]` | table   | Yes      | Operating system configuration section     |

### OS Configuration Table (`[os_name]`)

Each OS section supports these fields:

| Field          | Type   | Required | Description                                  |
| -------------- | ------ | -------- | -------------------------------------------- |
| `description`  | string | No       | Human-readable description of this OS config |
| `applications` | table  | No       | Applications to install (see below)          |
| `dependencies` | table  | No       | System dependencies to install first         |
| `dotfiles`     | table  | No       | Dotfiles configuration (see below)           |

### Applications Table (`[os_name.applications]`)

The applications table uses package names as keys with two formats:

**Simple Format (string):**

```toml
[fedora.applications]
git = "sudo dnf install -y git"
neovim = "sudo dnf install -y neovim"
```

**Extended Format (table) - for packages with dependencies:**

```toml
[fedora.applications.k9s]
install = "brew install derailed/k9s/k9s"
depends = ["brew"]
```

| Extended Field | Type   | Description                               |
| -------------- | ------ | ----------------------------------------- |
| `install`      | string | Installation command                      |
| `depends`      | array  | List of package names this app depends on |

### Dependencies Table (`[os_name.dependencies]`)

Dependencies are packages that should be installed first, before applications.

```toml
[fedora.dependencies]
zsh = "sudo dnf install -y zsh"
node = "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | zsh"
```

### Dotfiles Configuration (`[os_name.dotfiles]`)

| Field        | Type    | Required | Description                                                 |
| ------------ | ------- | -------- | ----------------------------------------------------------- |
| `repository` | string  | Yes      | Git URL or local path to dotfiles                           |
| `target`     | string  | No       | Directory name for cloned dotfiles (default: "dotfiles")    |
| `dry-run`    | boolean | No       | Preview changes without applying (default: false)           |
| `packages`   | array   | No       | Specific stow packages to apply. If omitted, auto-discovers |

```toml
[fedora.dotfiles]
repository = "https://github.com/username/dotfiles.git"
target = "dotfiles"
# packages = ["zsh", "tmux", "nvim"]  # optional, auto-discovers if omitted
```

## Complete Example

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
eza = "sudo dnf install -y eza"
fastfetch = "sudo dnf install -y fastfetch"
btop = "sudo dnf install -y btop"
docker = "sudo dnf install -y docker"

[fedora.applications.kubectl]
install = """cat <<EOF | sudo tee /etc/yum.repos.d/kubernetes.repo
[kubernetes]
name=Kubernetes
baseurl=https://pkgs.k8s.io/core:/stable:/v1.30/rpm/
enabled=1
gpgcheck=1
gpgkey=https://pkgs.k8s.io/core:/stable:/v1.30/rpm/repodata/repomd.xml.key
EOF && sudo dnf install -y kubectl"""

[fedora.dependencies]
zsh = "sudo dnf install -y zsh"
node = "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | zsh && nvm install latest"
brew = "/bin/zsh -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""

[fedora.dotfiles]
repository = "https://github.com/username/dotfiles.git"
target = "dotfiles"

[macos]
description = "macOS development environment"

[macos.applications]
k9s = "brew install derailed/k9s/k9s"
neovim = "brew install neovim"
tmux = "brew install tmux"
fzf = "brew install fzf"
ripgrep = "brew install ripgrep"
bat = "brew install bat"
eza = "brew install eza"

[macos.dependencies]
brew = "/bin/zsh -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""

[macos.dotfiles]
repository = "https://github.com/username/dotfiles.git"
target = "dotfiles"
```

## Notes

- Fonts are now treated as regular applications (e.g., `firacode = "sudo dnf install -y firacode"`)
- Package names are used as keys, eliminating the need for separate `name` and `package` fields
- Multi-line strings (using `"""`) are supported for complex install commands
- The `depends` field allows specifying dependencies between packages
