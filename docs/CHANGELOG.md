# Changelog

## v0.1.0 - Initial Release

### Features
- Fast cybersecurity command lookup via LLM
- Support for Perplexity and Groq providers
- Web search integration with DuckDuckGo
- Trusted source prioritization (HackTricks, PayloadsAllTheThings, etc.)
- Interactive and one-shot query modes
- Secure API key storage with 600 permissions
- Prompt injection protection
- Content sanitization and size limits
- CLI flags for quiet, verbose, no-tty, and no-search modes

### Security
- Prompt injection pattern detection and removal
- HTTP timeout and redirect limits
- Token limits (8000 max per request)
- Content size limits (12k chars per source)
- Secure configuration file permissions

### UI/UX
- Clean, professional output format
- No emojis - uses [+] [!] [*] [~] indicators
- Color-coded messages (green=success, red=error, cyan=info, yellow=warning)
- Formatted tables for search results
- Code block syntax highlighting

### Documentation
- Concise README with Mermaid diagram
- Comprehensive feature documentation
- Security testing report
- Full installation and usage guide

### Performance
- Binary size: 6.2MB (release, stripped, LTO)
- Average response time: 2-5 seconds
- Memory usage: ~50MB
- Fast startup: <100ms

## Project Structure Changes

### Documentation Organization
```
docs/
├── FEATURES.md      - Detailed feature breakdown
├── TESTING.md       - Security testing and validation
├── FULL_README.md   - Complete documentation
└── CHANGELOG.md     - This file
```

### Clean Output
Replaced all emojis with standard indicators:
- [+] Success/positive actions
- [!] Errors and warnings
- [*] Information
- [~] Loading/processing

### Mermaid Diagram
Added flow diagram in README.md showing:
- User query flow
- Search and LLM decision paths
- Content sanitization (highlighted in red)
- Provider selection
- Response formatting

## Dependencies

### Core
- tokio (async runtime)
- reqwest (HTTP client)
- clap (CLI framework)
- serde/toml (configuration)

### UI
- comfy-table (tables)
- colored (terminal colors)
- dialoguer (interactive prompts)
- indicatif (progress indicators)

### Content Processing
- html2text (HTML to Markdown)
- regex (pattern matching)
- html-escape (sanitization)

### Security
- Content sanitization
- Prompt injection detection
- Resource limits
- Secure file permissions

## License

MIT License - See LICENSE file
