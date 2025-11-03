#!/usr/bin/env bash
# Release packaging script for cyx
# Creates a distributable package with binary and required shared libraries

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Normalize architecture names
case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${RESET}"
        exit 1
        ;;
esac

# Get version from Cargo.toml
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)

# Set platform-specific variables
case "$OS" in
    linux)
        PLATFORM="linux-$ARCH"
        LIB_EXT="so"
        LIB_NAME="libonnxruntime.so.1.16.0"
        BINARY_NAME="cyx"
        ARCHIVE_EXT="tar.gz"
        ;;
    darwin)
        PLATFORM="macos-$ARCH"
        LIB_EXT="dylib"
        LIB_NAME="libonnxruntime.1.16.0.dylib"
        BINARY_NAME="cyx"
        ARCHIVE_EXT="tar.gz"
        ;;
    mingw*|msys*|cygwin*)
        PLATFORM="windows-$ARCH"
        LIB_EXT="dll"
        LIB_NAME="onnxruntime.dll"
        BINARY_NAME="cyx.exe"
        ARCHIVE_EXT="zip"
        ;;
    *)
        echo -e "${RED}Unsupported OS: $OS${RESET}"
        exit 1
        ;;
esac

RELEASE_NAME="cyx-v${VERSION}-${PLATFORM}"
RELEASE_DIR="dist/${RELEASE_NAME}"
ARCHIVE_NAME="${RELEASE_NAME}.${ARCHIVE_EXT}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${BLUE}║${RESET}  ${GREEN}Packaging cyx Release${RESET}                              ${BLUE}║${RESET}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${RESET}"
echo ""
echo -e "${BLUE}Version:${RESET}  ${GREEN}${VERSION}${RESET}"
echo -e "${BLUE}Platform:${RESET} ${GREEN}${PLATFORM}${RESET}"
echo -e "${BLUE}Output:${RESET}   ${GREEN}${ARCHIVE_NAME}${RESET}"
echo ""

# Clean and create dist directory
echo -e "${YELLOW}Cleaning previous builds...${RESET}"
rm -rf dist
mkdir -p "${RELEASE_DIR}"

# Check if binary exists
BINARY_PATH="target/release/${BINARY_NAME}"
if [[ ! -f "${BINARY_PATH}" ]]; then
    echo -e "${RED}Error: Binary not found at ${BINARY_PATH}${RESET}"
    echo -e "${YELLOW}Run 'cargo build --release' first${RESET}"
    exit 1
fi

# Check if library exists
LIB_PATH="target/release/${LIB_NAME}"
if [[ ! -f "${LIB_PATH}" ]]; then
    echo -e "${RED}Error: ONNX Runtime library not found at ${LIB_PATH}${RESET}"
    echo -e "${YELLOW}The library should be automatically downloaded during build.${RESET}"
    echo -e "${YELLOW}Try running 'cargo clean && cargo build --release'${RESET}"
    exit 1
fi

# Copy binary
echo -e "${YELLOW}Copying binary...${RESET}"
cp "${BINARY_PATH}" "${RELEASE_DIR}/"
chmod +x "${RELEASE_DIR}/${BINARY_NAME}"

# Copy library
echo -e "${YELLOW}Copying ONNX Runtime library...${RESET}"
cp "${LIB_PATH}" "${RELEASE_DIR}/"

# Copy symlink if it exists (for macOS/Linux)
if [[ -f "target/release/libonnxruntime.${LIB_EXT}" ]] && [[ "${OS}" != "mingw"* ]]; then
    cp "target/release/libonnxruntime.${LIB_EXT}" "${RELEASE_DIR}/" 2>/dev/null || true
fi

# Copy installer script
echo -e "${YELLOW}Copying installer script...${RESET}"
if [[ -f "install.sh" ]]; then
    cp "install.sh" "${RELEASE_DIR}/"
    chmod +x "${RELEASE_DIR}/install.sh"
fi

# Create README for the package
echo -e "${YELLOW}Creating installation instructions...${RESET}"
cat > "${RELEASE_DIR}/README.txt" << 'EOF'
CYX - Cybersecurity Companion
==============================

Quick Installation
------------------

Run the included installer script (recommended):

    bash install.sh

This will guide you through the installation process and handle
the ONNX Runtime library automatically.


Manual Installation
-------------------

⚠️  IMPORTANT: Both cyx and libonnxruntime.* must be kept together!

The cyx binary requires the ONNX Runtime library to function.
Installing only the binary will result in a "library not found" error.

Package contents:
- cyx: The main executable
- libonnxruntime.*: Required runtime library
- install.sh: Automated installer script

Installation options:

Option A - Local installation (recommended):
--------------------------------------------
mkdir -p ~/.local/bin
cp cyx ~/.local/bin/
cp libonnxruntime* ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"

Add the export line to your ~/.bashrc or ~/.zshrc to make it permanent.

Then run: cyx setup


Option B - System-wide installation:
------------------------------------
sudo cp cyx /usr/local/bin/
sudo cp libonnxruntime* /usr/local/lib/
sudo ldconfig  # Linux only

Then run: cyx setup


First Time Setup
----------------
After installation, configure your API keys:

    cyx setup

You'll need an API key from Perplexity or Groq.


Documentation
-------------
https://github.com/neur0map/cyx

Note: The ONNX Runtime library must be in the same directory as the
binary or in a system library directory (like /usr/local/lib).
EOF

# Create archive
echo -e "${YELLOW}Creating archive...${RESET}"
cd dist

if [[ "${ARCHIVE_EXT}" == "tar.gz" ]]; then
    tar -czf "${ARCHIVE_NAME}" "${RELEASE_NAME}"
else
    zip -r "${ARCHIVE_NAME}" "${RELEASE_NAME}"
fi

cd ..

# Calculate checksums
echo -e "${YELLOW}Calculating checksums...${RESET}"
if command -v sha256sum &> /dev/null; then
    sha256sum "dist/${ARCHIVE_NAME}" > "dist/${ARCHIVE_NAME}.sha256"
elif command -v shasum &> /dev/null; then
    shasum -a 256 "dist/${ARCHIVE_NAME}" > "dist/${ARCHIVE_NAME}.sha256"
else
    echo -e "${YELLOW}Warning: sha256sum not found, skipping checksum${RESET}"
fi

# Display results
echo ""
echo -e "${GREEN}✓ Release package created successfully!${RESET}"
echo ""
echo -e "${BLUE}Package contents:${RESET}"
echo "  - ${BINARY_NAME}"
echo "  - ${LIB_NAME}"
echo "  - install.sh"
echo "  - README.txt"
echo ""
echo -e "${BLUE}Output:${RESET}"
echo "  dist/${ARCHIVE_NAME}"
if [[ -f "dist/${ARCHIVE_NAME}.sha256" ]]; then
    echo "  dist/${ARCHIVE_NAME}.sha256"
fi
echo ""
echo -e "${BLUE}Archive size:${RESET}"
ls -lh "dist/${ARCHIVE_NAME}" | awk '{print "  " $5}'
echo ""

if [[ -f "dist/${ARCHIVE_NAME}.sha256" ]]; then
    echo -e "${BLUE}SHA256:${RESET}"
    cat "dist/${ARCHIVE_NAME}.sha256"
    echo ""
fi

echo -e "${GREEN}Ready to distribute!${RESET}"
echo ""
