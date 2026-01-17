# Installation Guide

Simple installation instructions for cyx.

## Installation

```bash
cargo install cyx
cyx setup
```

That's it! Takes about 30 seconds.

## Requirements

- **Rust toolchain** - Install from [rustup.rs](https://rustup.rs)
- **API key** - From [Groq](https://console.groq.com/) (recommended) or [Perplexity](https://www.perplexity.ai/settings/api)

## Setup Wizard

The setup wizard will guide you through:

1. **Provider Selection**: Choose Groq (recommended) or Perplexity
2. **API Key**: Enter your API key
3. **Validation**: Tests connection and saves config

The wizard auto-enables smart caching with default settings for optimal performance.

## Configuration

Your configuration is saved to `~/.config/cyx/config.toml` with secure permissions (600).

### View Configuration

```bash
cyx config show
```

### Change Provider

```bash
cyx config set provider perplexity
cyx config set perplexity_api_key "pplx_..."
```

### Re-run Setup

```bash
cyx setup
```

## Advanced: Ollama (Local Models)

For users who want to run local models:

1. **Install Ollama**: Download from [ollama.com](https://ollama.com)
2. **Download a model**: `ollama pull mistral:7b-instruct`
3. **Configure cyx**:
    ```bash
    cyx config set provider ollama
    cyx config set ollama_model "mistral:7b-instruct"
    ```

Note: Ollama is for advanced users and requires manual setup.

## Build from Source

If you want to build from source:

```bash
git clone https://github.com/neur0map/cyx.git
cd cyx
cargo install --path .
cyx setup
```

## Troubleshooting

### API Key Issues

If provider connection fails during setup:
- Verify your API key is correct
- Check network connectivity
- Try running: `cyx "test query"` to test manually

### System Dependencies

For cloud providers (Groq/Perplexity), no system dependencies are required!

Check your system status:
```bash
cyx doctor
```

## Uninstall

```bash
cargo uninstall cyx
rm -rf ~/.config/cyx ~/.cache/cyx
```

## Next Steps

See [USAGE.md](USAGE.md) for examples and advanced features.
