#!/usr/bin/env bash
# Launcher script for cyx that checks for required libraries
# This provides helpful error messages if the ONNX Runtime library is missing

set -euo pipefail

# Color output
RED='\033[0;31m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Find the actual cyx binary
# It could be in the same directory, parent directory, or in PATH
if [ -f "$SCRIPT_DIR/cyx" ]; then
    CYX_BINARY="$SCRIPT_DIR/cyx"
elif [ -f "$SCRIPT_DIR/../cyx" ]; then
    CYX_BINARY="$SCRIPT_DIR/../cyx"
elif command -v cyx &> /dev/null; then
    CYX_BINARY=$(command -v cyx)
else
    echo -e "${RED}Error: Could not find cyx binary${RESET}" >&2
    echo "Looked in:" >&2
    echo "  - $SCRIPT_DIR/cyx" >&2
    echo "  - $SCRIPT_DIR/../cyx" >&2
    echo "  - PATH" >&2
    exit 1
fi

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Check for the required library
check_library() {
    # Get the directory of the actual binary
    BIN_DIR="$(dirname "$CYX_BINARY")"

    case "$OS" in
        linux)
            LIB_NAME="libonnxruntime.so.1.16.0"
            # Check common locations
            if [ -f "$BIN_DIR/$LIB_NAME" ] || \
               [ -f "/usr/local/lib/$LIB_NAME" ] || \
               [ -f "/usr/lib/$LIB_NAME" ] || \
               ldconfig -p 2>/dev/null | grep -q "$LIB_NAME"; then
                return 0
            fi
            ;;
        darwin)
            LIB_NAME="libonnxruntime.1.16.0.dylib"
            # Check common locations
            if [ -f "$BIN_DIR/$LIB_NAME" ] || \
               [ -f "/usr/local/lib/$LIB_NAME" ] || \
               [ -f "/opt/homebrew/lib/$LIB_NAME" ]; then
                return 0
            fi
            ;;
        mingw*|msys*|cygwin*)
            LIB_NAME="onnxruntime.dll"
            if [ -f "$BIN_DIR/$LIB_NAME" ] || \
               command -v "$LIB_NAME" &> /dev/null; then
                return 0
            fi
            ;;
    esac
    return 1
}

# Show error message with instructions
show_library_error() {
    echo -e "\n${RED}╭─────────────────────────────────────────────────────────────────╮${RESET}"
    echo -e "${RED}│ ⚠️  ONNX Runtime Library Not Found                              │${RESET}"
    echo -e "${RED}╰─────────────────────────────────────────────────────────────────╯${RESET}\n"

    echo "The ONNX Runtime shared library is required but not installed."
    echo "This typically happens when installing via 'cargo install'."
    echo ""
    echo -e "${BLUE}📦 SOLUTION:${RESET}"
    echo ""
    echo "Download the full release package from:"
    echo "https://github.com/neur0map/cyx/releases"
    echo ""

    case "$OS" in
        linux)
            echo "For Linux, the release includes libonnxruntime.so.1.16.0"
            echo ""
            echo "Install it with:"
            echo "  sudo cp libonnxruntime.so.1.16.0 /usr/local/lib/"
            echo "  sudo ldconfig"
            echo ""
            echo "Or place it in the same directory as the cyx binary:"
            echo "  cp libonnxruntime.so.1.16.0 \$(dirname \$(which cyx))/"
            ;;
        darwin)
            echo "For macOS, the release includes libonnxruntime.1.16.0.dylib"
            echo ""
            echo "Install it with:"
            echo "  sudo cp libonnxruntime.1.16.0.dylib /usr/local/lib/"
            echo ""
            echo "Or place it in the same directory as the cyx binary:"
            echo "  cp libonnxruntime.1.16.0.dylib \$(dirname \$(which cyx))/"
            ;;
        *)
            echo "Extract the release package and copy the library file"
            echo "to the same directory as the cyx binary."
            ;;
    esac

    echo ""
    echo "─────────────────────────────────────────────────────────────────"
    echo "For more help, see: https://github.com/neur0map/cyx#troubleshooting"
    echo ""
}

# Main logic
if ! check_library; then
    show_library_error
    exit 1
fi

# If we get here, the library should be available, run the actual binary
exec "$CYX_BINARY" "$@"
