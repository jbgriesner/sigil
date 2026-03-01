# Valt

A terminal password manager that stores everything encrypted on your machine — no cloud, no network, no telemetry.

## Features

- All secrets stored in a single encrypted file on disk
- AES-256-GCM encryption with Argon2id key derivation (via [serdevault](https://github.com/jbgriesner/serdevault))
- Fuzzy search across names, URLs, usernames and tags
- Built-in password generator
- Clipboard auto-clear after 30 seconds
- Keyboard-driven TUI (vim-style navigation)

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

Vault is created on first launch at `~/.local/share/valt/vault.svlt`.

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
