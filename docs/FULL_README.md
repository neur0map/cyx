# Cyx: Your Cybersecurity Companion in Command

**Cyx** is a fast, terminal-based cybersecurity companion that instantly provides commands, techniques, and practical guidance for hacking, penetration testing, and security research. Powered by advanced language models, Cyx delivers concise, command-first responses optimized for security practitioners who need answers fast.

## Features

- **[X] LLM-Powered**: Uses advanced language models (Perplexity, Groq) for instant command lookup
- **[X] Command-First**: Executable commands before explanations - zero fluff
- **[X] Interactive Mode**: Conversational interface with context awareness and follow-up questions
- **[X] Fast**: Written in Rust for maximum performance (2-5 second average response time)
- **[X] Pentester-Optimized**: System prompt assumes authorization, no ethics disclaimers
- **[X] Beautiful CLI**: Clean formatted output for easy reading
- **[X] Secure**: API keys stored with 600 permissions in local config

## LLM Providers

Cyx supports two high-performance LLM providers:

- **Perplexity** (`sonar-pro`) - Fast responses with excellent cybersecurity knowledge
- **Groq** (`llama-3.3-70b-versatile`) - Very fast inference with free tier available

## Installation

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- An API key for one of the supported LLM providers:
  - [Groq](https://console.groq.com) (Free tier available)
  - [Perplexity](https://www.perplexity.ai/settings/api)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/neur0map/cyx.git
cd cyx

# Build release version
cargo build --release

# Optional: Install globally
cargo install --path .
```

## Quick Start

### Initial Setup

Run the setup wizard on first use:

```bash
cyx setup
```

You'll be prompted to:
1. Choose your LLM provider (Groq or Perplexity)
2. Enter your API key
3. Confirm configuration

Your config will be saved to `~/.config/cyx/config.toml` with secure permissions (600).

### Usage Examples

**Interactive Mode** (default):

```bash
cyx
# or
cyx interactive
```

**One-Shot Queries**:

```bash
cyx "how to crack WPA2 wifi"
cyx "SMB enumeration techniques"
cyx "privilege escalation with sudo"
```

**Interactive Commands**:

```
cyx> /help      # Show available commands
cyx> /clear     # Clear conversation history
cyx> /exit      # Exit the session
```

## Configuration

### View Current Config

```bash
cyx config show
```

### Set Configuration Values

```bash
# Change provider
cyx config set provider groq

# Update API keys
cyx config set groq_api_key YOUR_API_KEY
cyx config set perplexity_api_key YOUR_API_KEY
```

### Get Specific Values

```bash
cyx config get provider
cyx config get config_path
```

### Manual Configuration

Edit `~/.config/cyx/config.toml`:

```toml
provider = "groq"  # or "perplexity"

[api_keys]
groq_api_key = "your-groq-api-key"
perplexity_api_key = "your-perplexity-api-key"
```

## How It Works

1. **User Query**: Your question is sent to Cyx
2. **System Prompt**: Query is augmented with pentester-optimized instructions
3. **LLM Processing**: Provider (Perplexity or Groq) generates command-first response
4. **Format & Display**: Results are formatted and displayed in a clean manner

The system prompt explicitly:
- Assumes user authorization (no disclaimers needed)
- Enforces COMMAND-FIRST responses (executable code before explanations)
- Targets professional pentesters and security students
- Prioritizes practical execution over theory

## Example Queries

```bash
# Pentesting
cyx "nmap stealth scan techniques"
cyx "SQL injection payloads"
cyx "reverse shell cheatsheet"

# Web Hacking
cyx "XSS bypass WAF"
cyx "SSRF exploitation techniques"
cyx "how to find subdomain takeover"

# Privilege Escalation
cyx "linux privilege escalation checklist"
cyx "windows UAC bypass methods"
cyx "SUID exploitation"

# Forensics & Recon
cyx "active directory enumeration"
cyx "memory forensics with volatility"
cyx "OSINT tools for pentesting"
```

## Development

### Project Structure

```
cyx/
├── src/
│   ├── cli/          # CLI argument parsing and command handling
│   ├── config/       # Configuration management
│   ├── llm/          # LLM provider implementations (Perplexity, Groq)
│   ├── session/      # Interactive session handler
│   ├── ui/           # Display and formatting
│   ├── lib.rs
│   └── main.rs
├── docs/
│   ├── FEATURES.md
│   ├── TESTING.md
│   └── FULL_README.md
├── Cargo.toml
├── LICENSE
└── README.md
```

### Run in Development

```bash
cargo run -- "your query"
cargo run -- interactive
cargo run -- setup
```

### Run Tests

```bash
cargo test
```

## Disclaimer

Cyx is designed for **authorized security testing, education, and defensive purposes only**. All techniques and commands provided should only be used:

- On systems you own or have explicit permission to test
- In controlled lab environments
- For educational purposes (CTF challenges, courses)
- As part of authorized penetration testing engagements

**Never use this tool for unauthorized access, malicious purposes, or illegal activities.**

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:

- Bug fixes
- New features
- Documentation improvements
- Additional trusted sources

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

**neur0map**

## Acknowledgments

- Groq and Perplexity for their excellent LLM APIs
- The Rust community for outstanding crates and tools
- The cybersecurity community for continuous knowledge sharing

---

**Cyx: Fast. Direct. Built for pentesters.**
