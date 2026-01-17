# Installation Guide

Complete installation instructions for cyx.

## Quick Install (Recommended)

**One-line installer** for Linux/macOS:

```bash
curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/scripts/install.sh | bash
```

This will:
- Download the latest release for your platform
- Install to `~/.local/bin` (or custom location via `INSTALL_DIR`)
- Handle the ONNX Runtime library automatically
- Add to PATH if needed
- Run the setup wizard

**Custom installation directory:**
```bash
INSTALL_DIR=/usr/local/bin curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/scripts/install.sh | bash
```

## Manual Installation

**Note**: Pre-built binaries are available in GitHub Releases.

Download the latest release from [Releases page](https://github.com/neur0map/cyx/releases) and run:

```bash
# Extract the archive
tar -xzf cyx-v0.2.1-linux-x86_64.tar.gz
cd cyx-v0.2.1-linux-x86_64

# Run the installer script
bash install.sh
```

Or manually copy both files:
```bash
mkdir -p ~/.local/bin
cp cyx ~/.local/bin/
cp libonnxruntime* ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
cyx setup
```

**Important**: The `cyx` binary requires the ONNX Runtime library. Both files must be kept together in the same directory.

## Build from Source

### Prerequisites

- Rust 1.70 or higher
- API key from [Perplexity](https://www.perplexity.ai/settings/api) or [Groq](https://console.groq.com)

### Quick Build

```bash
git clone https://github.com/neur0map/cyx.git
cd cyx
make install  # Builds, installs binary and library
cyx setup
```

### Using Cargo Install

```bash
# Clone and navigate to the repository
git clone https://github.com/neur0map/cyx.git
cd cyx

# Install the binary
cargo install --path .

# Run setup - it will auto-detect and fix ONNX library issues
cyx setup
```

**Auto-Fix Feature**: When you run `cyx setup` or any command, cyx will automatically detect if the ONNX Runtime library is missing and attempt to fix it by:
1. Locating the library in your cargo build cache
2. Copying it to the binary directory
3. Providing manual instructions if auto-fix fails

**Manual Copy** (if auto-fix doesn't work):
```bash
INSTALL_DIR=$(dirname $(which cyx))
cp target/release/libonnxruntime* $INSTALL_DIR/
```

### Makefile Commands

```bash
make build    # Build and create symlink for development
make check    # Run fmt + clippy
make install  # Install to system (includes library)
make setup    # Run setup wizard
make help     # Show all commands
```

## Initial Configuration

After installation, run the interactive setup wizard:

```bash
cyx setup
```

This creates `~/.config/cyx/config.toml` with your API key (stored with 600 permissions).

## Troubleshooting

### Error: libonnxruntime.so.1.16.0: cannot open shared object file

This error means the ONNX Runtime library is not in the library search path.

**Automatic Fix:**

Simply run:
```bash
cyx setup
```

Cyx will automatically detect the missing library and attempt to fix it by:
- Searching for the library in your cargo build cache
- Copying it to the correct location
- Providing manual instructions if needed

**Manual Solutions:**

If auto-fix doesn't work:

1. **For cargo install users**:
   ```bash
   # Copy from build cache
   INSTALL_DIR=$(dirname $(which cyx))
   cp target/release/libonnxruntime* $INSTALL_DIR/
   ```

2. **For release downloads**: Extract the full archive and ensure both files are together:
   ```bash
   # Both files should be in the same directory
   ls -l
   # Should show: cyx and libonnxruntime.so.1.16.0 (or .dylib on macOS)
   ```

3. **System-wide installation**:
   ```bash
   sudo cp libonnxruntime* /usr/local/lib/
   sudo ldconfig  # Linux only
   ```

For more detailed troubleshooting, see [BUILDING.md](BUILDING.md).
