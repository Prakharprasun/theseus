#!/bin/sh

set -e

# Theseus (tsus) Installer Script
# https://github.com/Prakharprasun/theseus
#
# This script determines the OS and architecture, downloads the latest release binary,
# and installs it into ~/.local/bin or /usr/local/bin.

REPO="Prakharprasun/theseus"
BINARY_NAME="tsus"

echo "Installing Theseus..."

# 1. Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$OS" in
    darwin*) OS="macos" ;;
    linux*) OS="linux" ;;
    *) echo "❌ Unsupported OS: $OS"; exit 1 ;;
esac

# 2. Detect Architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
esac

# 3. Formulate the Binary Name
ASSET_NAME="${BINARY_NAME}-${OS}-${ARCH}"

# 4. Find the latest release URL
echo "Fetching latest release information..."
LATEST_URL=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep "browser_download_url.*${ASSET_NAME}\"" | cut -d '"' -f 4)

if [ -z "$LATEST_URL" ]; then
    echo "❌ Could not find a binary for ${OS} ${ARCH}."
    echo "Please check the releases page manually: https://github.com/${REPO}/releases"
    exit 1
fi

# 5. Download the binary
TMP_FILE=$(mktemp)
echo "Downloading ${ASSET_NAME}..."
curl -fsSL "$LATEST_URL" -o "$TMP_FILE"

# 6. Determine Install Location
INSTALL_DIR="$HOME/.local/bin"

if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
fi

echo "Installing to ${INSTALL_DIR}..."

# Ensure the directory exists
mkdir -p "$INSTALL_DIR"

# Move and make executable
mv "$TMP_FILE" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo ""
echo "✅ Theseus cleanly installed to ${INSTALL_DIR}/${BINARY_NAME}"

# 7. Check if it's on PATH
if ! command -v "$BINARY_NAME" >/dev/null 2>&1; then
    echo ""
    echo "⚠️  Wait! ${INSTALL_DIR} is not in your PATH."
    echo "Please add the following line to your ~/.bashrc, ~/.zshrc, or profile:"
    echo ""
    echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
    echo ""
fi

echo "🎉 Success! Run 'tsus doctor' to verify."
