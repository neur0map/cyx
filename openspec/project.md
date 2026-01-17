# Project Context

## Purpose
Cyx is an LLM-powered terminal tool for security command lookup, designed for professional penetration testers, security students, and CTF competitors. It provides command-first output with educational modes, smart caching using ONNX semantic search, and support for multiple LLM providers (Perplexity, Groq, Ollama).

## Tech Stack
- **Language**: Rust (2021 edition)
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest
- **CLI Framework**: Clap (derive features)
- **Serialization**: Serde, TOML, JSON
- **Cache/DB**: SQLite (rusqlite with bundled features)
- **ML/Embeddings**: ONNX Runtime, tokenizers, ndarray for vector similarity search
- **UI/Display**: colored, comfy-table, dialoguer, indicatif
- **Build System**: Makefile with comprehensive commands

## Project Conventions

### Code Style
- **Formatter**: `cargo fmt` with standard Rust conventions
- **Linter**: `cargo clippy` with pedantic checks enabled
- Run `make check` before commits to ensure formatting and linting pass
- Run `make fmt` to auto-fix formatting issues
- Module structure: separate directories for cache, cli, config, llm, search, session, ui

### Architecture Patterns
- **Modular Design**: Clear separation of concerns (CLI, LLM providers, cache, config, UI)
- **Provider Pattern**: Abstract LLM provider interface supporting Perplexity, Groq, and Ollama
- **Semantic Caching**: ONNX-powered vector embeddings for intelligent cache lookups
- **Data Embedding**: Critical data files (ONNX model, tokenizer) embedded in binary at compile time for portable execution
- **Error Handling**: anyhow for ergonomic error propagation

### Testing Strategy
- Test files in `examples/` directory (e.g., test_normalizer.rs)
- Focus on integration testing for LLM interactions and cache behavior
- Manual testing through `make build` and local execution
- Quality gates: `make check` runs fmt and clippy before merging

### Git Workflow
- **Main Branch**: `master`
- **Commit Style**: Descriptive messages with context (see recent commits)
- **Release Process**: Tag-based releases (e.g., `v0.2.1`) trigger GitHub Actions for multi-platform builds
- **Pre-merge**: Run `make check` to validate code quality
- Clean working directory preferred (current status shows clean state)

## Domain Context
- **Security Focus**: Tool designed for authorized security testing, CTF challenges, and educational purposes only
- **Command-First Philosophy**: Prioritize executable commands over explanations in normal mode
- **Learn Mode**: Educational mode (`--learn`/`-l`) provides detailed breakdowns and alternatives
- **Provider Knowledge**: Understanding of Perplexity Sonar, Groq, and Ollama API patterns
- **Semantic Search**: Vector similarity using ONNX embeddings to match user queries with cached responses

## Important Constraints
- **Security & Ethics**: For authorized testing only - includes explicit disclaimer
- **API Key Security**: Stored with 600 permissions in `~/.config/cyx/config.toml`
- **Read-Only**: Tool provides commands but never executes them
- **Timeout Protection**: All API calls timeout after 120 seconds
- **Local-First**: All sensitive data remains on user's machine
- **Binary Portability**: ONNX Runtime library must accompany the binary

## External Dependencies
- **LLM Providers**:
  - Perplexity API (sonar-pro model)
  - Groq API
  - Ollama (local LLM support)
- **ONNX Runtime**: Version 1.16 for semantic embedding inference
- **Build Dependencies**: Rust toolchain 1.70+
- **Runtime**: SQLite for cache storage (bundled)
