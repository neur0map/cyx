# Changelog

## v0.2.0

### Added
- Smart cache system with query normalization
- ONNX-powered semantic search (384D transformer embeddings using ort 1.16)
- Ollama local LLM support
- Interactive setup wizard with dependency management
- Cache commands: stats, list, clear, cleanup, remove
- Ollama commands: list, pull, remove
- System health check (doctor command)
- Embedding model download command
- SQLite storage with hit/miss tracking

### Changed
- Enhanced config management with get/set commands
- Improved CLI argument structure
- Updated dependencies for cache and ML support

### Technical
- Using ort 1.16 (stable) with ONNX Runtime 1.16.0
- ort 2.0 upgrade planned for future release

## v0.1.0 - Initial Release

### Features
- Command lookup via Perplexity and Groq
- Learn mode for detailed explanations
- Prompt injection protection
- Secure configuration storage
- CLI flags: learn, quiet, verbose, no-tty, no-search
