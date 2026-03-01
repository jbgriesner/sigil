# Valt

A terminal password manager that stores everything encrypted on your machine — no cloud, no network, no telemetry.

## Features

- All secrets stored in a single encrypted file on disk
- AES-256-GCM encryption with Argon2id key derivation (via [serdevault](https://github.com/jbgriesner/serdevault))
- Fuzzy search across names, URLs, usernames and tags
- Built-in password generator
- Clipboard auto-clear after 30 seconds
- Keyboard-driven TUI (vim-style navigation)

## Usage

```
cargo run --release
```

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

See [LICENSE](LICENSE).
