# Valt

[![CI](https://github.com/jbgriesner/valt/actions/workflows/ci.yml/badge.svg)](https://github.com/jbgriesner/valt/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/jbgriesner/valt)](https://github.com/jbgriesner/valt/releases/latest)
[![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-informational)](https://github.com/jbgriesner/valt/releases/latest)

A terminal password manager that stores everything encrypted on your machine — no cloud, no network, no telemetry.

## Features

- All secrets stored in a single encrypted file on disk
- AES-256-GCM encryption with Argon2id key derivation (via [serdevault](https://github.com/jbgriesner/serdevault))
- Fuzzy search across names, URLs, usernames and tags
- Built-in password generator with interactive popup
- Clipboard auto-clear after 30 seconds
- Keyboard-driven TUI (vim-style navigation)
- Non-interactive CLI for scripting and shell integration

## Installation

### Linux / macOS (one-liner)

```sh
curl -sSf https://raw.githubusercontent.com/jbgriesner/valt/master/install.sh | sh
```

To install a specific version or to a custom directory:

```sh
VALT_VERSION=v0.1.3 VALT_INSTALL_DIR=/usr/local/bin \
  curl -sSf https://raw.githubusercontent.com/jbgriesner/valt/master/install.sh | sh
```

### macOS (Homebrew)

```sh
brew install jbgriesner/valt/valt
```

### Arch Linux (AUR)

```sh
yay -S valt-bin
```

### Debian / Ubuntu

Download the `.deb` from the [latest release](https://github.com/jbgriesner/valt/releases/latest) and install it:

```sh
sudo dpkg -i valt_*.deb
```

### Windows

Download the `.zip` from the [latest release](https://github.com/jbgriesner/valt/releases/latest), extract it, and add the folder to your `PATH`.

### Build from source

```sh
cargo install --git https://github.com/jbgriesner/valt
```

## Usage

The vault is stored at `~/.local/share/valt/vault.svlt` and created on first use.

### TUI

Launch the interactive interface:

```sh
valt
```

### CLI

All CLI commands prompt for the vault password interactively (no echo).

```sh
# List all secrets
valt list

# Filter with a fuzzy query
valt list github

# Print the password of the best match to stdout
valt get github

# Scriptable — only the password reaches stdout
export TOKEN=$(valt get myapi)

# Add a secret (prompts for password + confirmation)
valt add "GitHub perso" --username jb@example.com --url github.com

# Add a secret with a generated password
valt add "AWS root" -u admin@company.com -g

# Add a secret with tags
valt add "Server SSH" -u root --tags "linux,ops"

# Delete a secret (asks for confirmation)
valt rm github

# Delete without confirmation
valt rm github -y
```

Use `valt <command> --help` for details on any command.

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `↵` | Open detail |
| `n` | New secret |
| `e` | Edit secret |
| `d` | Delete secret |
| `c` | Copy password (auto-clears in 30s) |
| `Space` | Toggle password visibility |
| `g` | Generate password (in password field) |
| `?` | Help |
| `q` / `Ctrl+C` | Quit |

## License

GPL-3.0-or-later — see [LICENSE](LICENSE).
