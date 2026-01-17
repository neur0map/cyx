#!/usr/bin/env bash
# Cyx Installer - Simplified installation script
# Usage: curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
RESET='\033[0m'

# Configuration
GITHUB_REPO="neur0map/cyx"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
TEMP_DIR=$(mktemp -d)

# Cleanup on exit
trap 'rm -rf "$TEMP_DIR"' EXIT

print_header() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                  CYX Installer                             ║"
    echo "║          LLM-Powered Security Tool                         ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${RESET}"
}

print_step() {
    echo -e "${BLUE}==>${RESET} ${1}"
}

print_success() {
    echo -e "${GREEN}✓${RESET} ${1}"
}

print_warning() {
    echo -e "${YELLOW}⚠${RESET}  ${1}"
}

print_error() {
    echo -e "${RED}✗${RESET} ${1}"
}

detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$arch" in
        x86_64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac

    case "$os" in
        linux)
            PLATFORM="linux-${arch}"
            LIB_NAME="libonnxruntime.so.1.16.0"
            BINARY_NAME="cyx"
            ;;
        darwin)
            PLATFORM="macos-${arch}"
            LIB_NAME="libonnxruntime.1.16.0.dylib"
            BINARY_NAME="cyx"
            ;;
        *)
            print_error "Unsupported OS: $os"
            exit 1
            ;;
    esac
}

check_dependencies() {
    print_step "Checking dependencies..."
    
    local missing_deps=()
    
    if ! command -v curl &> /dev/null; then
        missing_deps+=("curl")
    fi
    
    if ! command -v tar &> /dev/null; then
        missing_deps+=("tar")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        echo "Please install them and try again."
        exit 1
    fi
    
    print_success "All dependencies found"
}

get_latest_version() {
    print_step "Fetching latest version..."
    
    VERSION=$(curl -sSL "https://api.github.com/repos/${GITHUB_REPO}/releases/latest" | \
              grep '"tag_name":' | \
              sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        print_error "Failed to fetch latest version"
        exit 1
    fi
    
    print_success "Latest version: ${VERSION}"
}

download_release() {
    print_step "Downloading cyx ${VERSION} for ${PLATFORM}..."
    
    local archive_name="cyx-${VERSION}-${PLATFORM}.tar.gz"
    local download_url="https://github.com/${GITHUB_REPO}/releases/download/${VERSION}/${archive_name}"
    
    cd "$TEMP_DIR"
    
    if ! curl -sSL -f -o "${archive_name}" "${download_url}"; then
        print_error "Failed to download release"
        echo "URL: ${download_url}"
        exit 1
    fi
    
    print_success "Download complete"
}

install_binary() {
    print_step "Installing cyx..."
    
    # Extract archive
    local archive_name="cyx-${VERSION}-${PLATFORM}.tar.gz"
    tar -xzf "${archive_name}"
    
    local extracted_dir="cyx-${VERSION}-${PLATFORM}"
    
    if [ ! -d "$extracted_dir" ]; then
        print_error "Extraction failed"
        exit 1
    fi
    
    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    # Copy binary and library
    cp "${extracted_dir}/${BINARY_NAME}" "$INSTALL_DIR/"
    cp "${extracted_dir}/${LIB_NAME}" "$INSTALL_DIR/"
    
    # Copy symlink if exists
    if [ -f "${extracted_dir}/libonnxruntime.so" ] || [ -f "${extracted_dir}/libonnxruntime.dylib" ]; then
        cp "${extracted_dir}"/libonnxruntime.* "$INSTALL_DIR/" 2>/dev/null || true
    fi
    
    # Make binary executable
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    print_success "Installed to ${INSTALL_DIR}"
}

setup_path() {
    # Check if install dir is in PATH
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        print_warning "${INSTALL_DIR} is not in your PATH"
        
        local shell_rc=""
        if [ -n "${BASH_VERSION:-}" ]; then
            shell_rc="$HOME/.bashrc"
        elif [ -n "${ZSH_VERSION:-}" ]; then
            shell_rc="$HOME/.zshrc"
        else
            shell_rc="$HOME/.profile"
        fi
        
        echo ""
        echo -e "${YELLOW}Add this line to your ${shell_rc}:${RESET}"
        echo -e "${CYAN}export PATH=\"${INSTALL_DIR}:\$PATH\"${RESET}"
        echo ""
        echo -e "${YELLOW}Then reload your shell:${RESET}"
        echo -e "${CYAN}source ${shell_rc}${RESET}"
        echo ""
        
        read -p "Would you like to add it automatically? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "" >> "$shell_rc"
            echo "# Added by cyx installer" >> "$shell_rc"
            echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "$shell_rc"
            print_success "Added to ${shell_rc}"
            echo -e "${YELLOW}Please run: ${CYAN}source ${shell_rc}${RESET}"
        fi
    else
        print_success "Already in PATH"
    fi
}

run_setup() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${RESET}"
    print_success "Installation complete!"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${RESET}"
    echo ""
    
    if [[ ":$PATH:" == *":${INSTALL_DIR}:"* ]]; then
        read -p "Would you like to run the setup wizard now? [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            "${INSTALL_DIR}/cyx" setup
        else
            echo ""
            echo -e "${YELLOW}Run '${CYAN}cyx setup${YELLOW}' when you're ready to configure your API keys.${RESET}"
        fi
    else
        echo -e "${YELLOW}After adding cyx to your PATH, run:${RESET}"
        echo -e "${CYAN}  cyx setup${RESET}"
    fi
    
    echo ""
    echo -e "${MAGENTA}Get started:${RESET}"
    echo -e "  ${CYAN}cyx setup${RESET}              # Configure API keys"
    echo -e "  ${CYAN}cyx \"nmap scan\"${RESET}       # Get security commands"
    echo -e "  ${CYAN}cyx --learn \"privilege escalation\"${RESET}"
    echo ""
    echo -e "${BLUE}Documentation:${RESET} https://github.com/${GITHUB_REPO}"
    echo ""
}

# Main installation flow
main() {
    print_header
    
    detect_platform
    check_dependencies
    get_latest_version
    download_release
    install_binary
    setup_path
    run_setup
}

main
