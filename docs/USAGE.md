# Usage Guide

## Smart Cache System

The cache works automatically - no configuration needed. Repeated queries return instantly without API calls.

### Query Normalization

Cyx normalizes queries to match similar questions:

```bash
"Show me nmap SYN scan!!!"  -> "network mapper nmap stealth synchronize scan"
"show me nmap syn scan"     -> "network mapper nmap stealth synchronize scan"
"NMAP SYN SCAN"             -> "network mapper nmap stealth synchronize scan"
```

Features:
- Lowercase conversion
- Abbreviation expansion (nmap -> network mapper nmap)
- Stopword removal (show me, how to)
- Hash-based exact matching
- Vector similarity search (ONNX embeddings)

### Cache Commands

```bash
cyx cache stats                # View statistics
cyx cache list                 # Show cached queries
cyx cache list --limit 20      # Show 20 entries
cyx cache clear                # Clear all cache
cyx cache cleanup --days 30    # Remove entries older than 30 days
cyx cache remove <hash>        # Remove specific entry
```

### Statistics Output

```
Cache Statistics
────────────────────────────────────────
Total entries: 45
Cache size: 1.23 MB
Hit count: 32
Miss count: 13
Hit rate: 71.1%
Cache location: /Users/you/Library/Caches/cyx
```

### Cache Storage

- Location: `~/.cache/cyx/` (Linux/macOS) or `%LOCALAPPDATA%\cyx\` (Windows)
- Format: SQLite database
- TTL: 30 days (configurable via `cyx config set cache.ttl_days 60`)

## Ollama Local Models (Advanced)

**Note**: For most users, cloud providers (Groq/Perplexity) are recommended. Ollama requires manual installation and setup.

Run LLMs locally with zero API costs:

```bash
# Install Ollama first: https://ollama.com
# Then configure cyx
cyx config set provider ollama
cyx config set ollama_model "mistral:7b-instruct"

# List installed models
cyx ollama list

# Pull a model
cyx ollama pull mistral:7b-instruct

# Remove a model
cyx ollama remove llama3.2:3b

# Use with Ollama
cyx "nmap stealth scan"  # Uses local model
```

### Recommended Models

- `llama3.2:3b` - Fast (2 GB)
- `mistral:7b-instruct` - Balanced (4.1 GB)
- `mixtral:8x7b` - High quality (26 GB)
- `codellama:7b-instruct` - Code-focused (3.8 GB)

## Configuration

### Config File

Location: `~/.config/cyx/config.toml` (600 permissions)

```toml
provider = "perplexity"  # or "groq", "ollama"

[api_keys]
groq = "gsk_..."
perplexity = "ppl_..."

[ollama]
base_url = "http://localhost:11434"
model = "mistral:7b-instruct"

[cache]
enabled = true
ttl_days = 30
embedding_model = "small"  # small, medium, large
```

### Config Commands

```bash
cyx config show                          # View all settings
cyx config get provider                  # Get specific value
cyx config set provider ollama           # Change provider
cyx config set cache.enabled false       # Disable cache
cyx config set cache.ttl_days 60         # Cache lifetime
```

## ONNX Embedding Models

Vector similarity search uses transformer models for semantic matching:

```bash
cyx download-model small    # 86 MB, 384D (recommended)
cyx download-model medium   # 400 MB, 768D
cyx download-model large    # 1.3 GB, 1024D
```

Models are downloaded to `~/.cache/cyx/models/` and loaded automatically.

### How It Works

1. Query normalization creates hash
2. Check for exact hash match (instant)
3. If no match, compute ONNX embedding vector
4. Search for similar cached queries (cosine similarity > 0.80)
5. If cache miss, call API and store response with embedding

## System Health

```bash
cyx doctor
```

Output:
```
System Status
────────────────────────────────────────
[+] SQLite (bundled v3.45.0)
[+] ONNX Runtime (1.16.0)
[+] Ollama (v0.1.48)
    Service: Running
    Models: 2 installed
[+] Cache (small model, 45 MB, 142 entries)
```

## Advanced Usage

### Scripting Mode

```bash
# Disable TTY features for piping
cyx --no-tty "reverse shell" | grep bash

# Quiet mode (response only)
cyx -q "hydra ssh brute force" > command.txt

# Combine flags
cyx -q --no-tty "nmap scan" | tee scan-cmd.txt
```

### Disable Web Search

```bash
# Force LLM knowledge only (faster, no sources)
cyx --no-search "sql injection basics"
```



## Performance

- Cache hit latency: < 10ms (hash match)
- Vector search: 50-100ms (ONNX similarity)
- API calls: 2-5 seconds
- Storage: ~100-500 KB per cached query

Hit rate depends on query patterns and similarity threshold.

## Troubleshooting

### Cache Not Working

```bash
cyx cache stats    # Check if enabled and has entries
cyx config show    # Verify cache.enabled = true
```

### Ollama Connection Failed

```bash
# Check if Ollama is running
curl http://localhost:11434

# Start Ollama service (macOS/Linux)
ollama serve

# Verify models installed
ollama list
```

### ONNX Model Loading Error

```bash
# Re-download model
rm -rf ~/.cache/cyx/models/small
cyx download-model small

# Check model files exist
ls -lh ~/.cache/cyx/models/small/
```

### M3 Mac dylib Error

```bash
# Fix ONNX Runtime rpath
install_name_tool -add_rpath "@executable_path" target/release/cyx
```
