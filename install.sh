#!/usr/bin/env bash
set -euo pipefail
trap 'die "Command failed at line $LINENO: $BASH_COMMAND"' ERR

# --- config ---
REPO="MintyDoggo/miri"
INSTALL_DIR="$HOME/.local/bin"
SERVICE_DIR="$HOME/.config/systemd/user"
WANTS_DIR="$SERVICE_DIR/niri.service.wants"
BINARY_ASSET="PLACEHOLDER_BINARY_NAME"
SERVICE_ASSET="miri.service"

# --- helpers ---
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[34m'
MAGENTA='\033[35m'
CYAN='\033[36m'
WHITE='\033[37m'
BOLD='\033[1m'
DIM='\033[2m'
RESET='\033[0m'

info() { echo -e "${BLUE}[Info]:${RESET} $*"; }
die()  { echo -e "${RED}[Error]:${RESET} $*" >&2; exit 1; }

# --- checks ---
command -v curl >/dev/null || die "curl is required"
command -v systemctl >/dev/null || die "systemctl is required"
systemctl --user show-environment >/dev/null 2>&1 || die "systemd user session not available"

# --- fetch release manifest ---
info "Fetching latest release info from $REPO"
RELEASE_JSON=$(curl -sf "https://api.github.com/repos/$REPO/releases/latest") \
  || die "Failed to fetch release info. Check if $REPO has published releases"

get_asset_url() {
  local asset_name="$1"
  local url
  url=$(echo "$RELEASE_JSON" \
    | grep -o '"browser_download_url": *"[^"]*'"$asset_name"'[^"]*"' \
    | cut -d'"' -f4)
  [[ -n "$url" ]] || die "Asset '$asset_name' not found in latest release — check the asset name"
  echo "$url"
}

# --- install binary ---
BINARY_URL=$(get_asset_url "$BINARY_ASSET")
mkdir -p "$INSTALL_DIR"
info "Downloading binary to $INSTALL_DIR/miri"
curl -sL "$BINARY_URL" -o "$INSTALL_DIR/miri"
chmod +x "$INSTALL_DIR/miri"

# --- install service ---
SERVICE_URL=$(get_asset_url "$SERVICE_ASSET")
mkdir -p "$WANTS_DIR"
info "Downloading miri.service"
curl -sL "$SERVICE_URL" -o "$SERVICE_DIR/miri.service"

info "Symlinking into niri.service.wants"
ln -sf "$SERVICE_DIR/miri.service" "$WANTS_DIR/miri.service"

# --- reload ---
info "Reloading systemd user daemon"
systemctl --user daemon-reload

info "Done. Miri will start with niri next login"
info "To start it now: systemctl --user start miri.service"