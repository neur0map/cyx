# Cyx: Your Cybersecurity Companion in Command

**Cyx** is a fast, terminal-based cybersecurity companion that instantly retrieves commands, documentation, and practical techniques for hacking, penetration testing, and security research. It searches trusted sources like HackTricks, PayloadsAllTheThings, OWASP, and official tool documentation to provide concise answers, real-world examples, and recommended workflows for common offensive security tasks.

## Features

- **[X] Smart Search**: Searches DuckDuckGo and prioritizes trusted cybersecurity sources
- **[X] AI-Powered**: Uses LLMs (Groq, Perplexity) to analyze results and provide precise answers
- **[X] Trusted Sources**: Fetches content from HackTricks, PayloadsAllTheThings, OWASP, GTFOBins, and more
- **[X] Interactive Mode**: Conversational interface with context awareness and follow-up questions
- **[X] Fast**: Written in Rust for maximum performance
- **[X] Beautiful CLI**: Clean tables and formatted output for easy reading
- **[X] Secure**: API keys stored with 600 permissions in local config

## Trusted Sources

Cyx prioritizes content from these trusted cybersecurity resources:

- [HackTricks](https://book.hacktricks.xyz)
- [PayloadsAllTheThings](https://github.com/swisskyrepo/PayloadsAllTheThings)
- [OWASP](https://owasp.org)
- [GTFOBins](https://gtfobins.github.io)
- [LOLBAS](https://lolbas-project.github.io)
- [Exploit Database](https://exploit-db.com)
- [PentestMonkey](https://pentestmonkey.net)
- [HackingArticles](https://hackingarticles.in)
- [Red Team Notes](https://ired.team)

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
cyx> /sources   # Show recently fetched sources
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
[provider]
provider = "groq"  # or "perplexity"

[api_keys]
groq = "your-groq-api-key"
perplexity = "your-perplexity-api-key"

[search]
max_results = 5
timeout_seconds = 30
trusted_sources = [
    "book.hacktricks.xyz",
    "github.com/swisskyrepo/PayloadsAllTheThings",
    # ... more sources
]
```

## How It Works

1. **Search**: User query is sent to DuckDuckGo API
2. **Filter**: Results are analyzed and trusted sources are identified
3. **Fetch**: Content from trusted sources is downloaded and converted to markdown
4. **Analyze**: LLM reads the content and synthesizes a comprehensive answer
5. **Display**: Results are formatted and displayed in a clean, organized manner

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
│   ├── llm/          # LLM provider implementations
│   ├── search/       # DuckDuckGo search and content fetching
│   ├── session/      # Interactive session handler
│   ├── ui/           # Display and formatting
│   ├── lib.rs
│   └── main.rs
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

- HackTricks, PayloadsAllTheThings, OWASP, and all the amazing cybersecurity community resources
- Groq and Perplexity for their LLM APIs
- The Rust community for excellent crates and tools

---

**Stay curious, stay ethical, stay secure.** [X][X]
