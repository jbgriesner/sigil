#!/usr/bin/env sh
# valt installer — https://github.com/jbgriesner/valt
#
# Usage:
#   curl -sSf https://raw.githubusercontent.com/jbgriesner/valt/master/install.sh | sh
#
# Environment variables:
#   VALT_VERSION     — install a specific tag (e.g. v0.1.3); defaults to latest
#   VALT_INSTALL_DIR — install directory; defaults to ~/.local/bin

set -e

REPO="jbgriesner/valt"
BIN="valt"
RELEASES="https://github.com/${REPO}/releases"

err()  { printf '\033[31merror\033[0m: %s\n' "$1" >&2; exit 1; }
info() { printf '\033[32m  ok\033[0m  %s\n' "$1"; }
step() { printf '\033[34m  --\033[0m  %s\n' "$1"; }
need() { command -v "$1" >/dev/null 2>&1 || err "required tool not found: $1"; }

need curl
need tar

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
  Linux)
    case "${ARCH}" in
      x86_64)         TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64|arm64)  err "Linux arm64 builds are not yet available. Follow ${RELEASES} for updates." ;;
      *)              err "Unsupported Linux architecture: ${ARCH}" ;;
    esac
    ;;
  Darwin)
    case "${ARCH}" in
      arm64|aarch64)  TARGET="aarch64-apple-darwin" ;;
      x86_64)         err "macOS Intel is not supported by the prebuilt binary.\nInstall via Homebrew instead:\n  brew install jbgriesner/valt/valt" ;;
      *)              err "Unsupported macOS architecture: ${ARCH}" ;;
    esac
    ;;
  *)
    err "Unsupported OS: ${OS}. Windows users: download the .zip from ${RELEASES}/latest"
    ;;
esac

if [ -n "${VALT_VERSION:-}" ]; then
  TAG="${VALT_VERSION}"
else
  step "Fetching latest release…"
  TAG="$(curl -sf "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' \
    | sed 's/.*"tag_name"[[:space:]]*:[[:space:]]*"//;s/".*//')"
  [ -n "${TAG}" ] || err "Could not determine latest version (GitHub API unreachable?)"
fi

info "Version: ${TAG}  target: ${TARGET}"

ASSET="${BIN}-${TAG}-${TARGET}.tar.gz"
URL="${RELEASES}/download/${TAG}/${ASSET}"

TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT INT TERM

step "Downloading ${ASSET}…"
curl -fL --progress-bar -o "${TMP}/${ASSET}" "${URL}" \
  || err "Download failed.\nURL: ${URL}\nCheck that the release exists at ${RELEASES}"

step "Extracting…"
tar -xzf "${TMP}/${ASSET}" -C "${TMP}"

INSTALL_DIR="${VALT_INSTALL_DIR:-${HOME}/.local/bin}"
mkdir -p "${INSTALL_DIR}"
mv "${TMP}/${BIN}" "${INSTALL_DIR}/${BIN}"
chmod 755 "${INSTALL_DIR}/${BIN}"

info "Installed: ${INSTALL_DIR}/${BIN}"

case ":${PATH}:" in
  *:"${INSTALL_DIR}":*)
    ;;
  *)
    printf '\n\033[33mNote\033[0m: %s is not in your PATH.\n' "${INSTALL_DIR}"
    printf 'Add the following line to your shell config (~/.bashrc, ~/.zshrc, etc.):\n'
    printf '  export PATH="%s:$PATH"\n\n' "${INSTALL_DIR}"
    ;;
esac

info "Done! Run: valt"
