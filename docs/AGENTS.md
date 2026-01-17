# AI Assistant Guide

Guide for AI coding assistants working on the cyx project.

> **For OpenSpec Workflow**: This project uses OpenSpec for spec-driven development.
> For the complete workflow guide, see [/openspec/AGENTS.md](../openspec/AGENTS.md).

## Project Overview

Cyx is an LLM-powered terminal tool for security command lookup designed for professional penetration testers, security students, and CTF competitors. It provides command-first output with educational modes, smart caching using ONNX semantic search, and support for multiple LLM providers.

## Key Information

### Project Context

See [/openspec/project.md](../openspec/project.md) for comprehensive details on:
- Tech stack (Rust, Tokio, ONNX Runtime, SQLite)
- Code style conventions
- Architecture patterns
- Testing strategy
- Git workflow

### Documentation Structure

- **[INSTALLATION.md](INSTALLATION.md)** - Installation and setup instructions
- **[USAGE.md](USAGE.md)** - Usage examples and CLI options
- **[BUILDING.md](BUILDING.md)** - Build from source and distribution
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Contributing and development workflow
- **[CHANGELOG.md](CHANGELOG.md)** - Version history
- **[DATA_NORMALIZATION.md](DATA_NORMALIZATION.md)** - Technical deep-dive into normalization data

## Quick Reference

### Before Starting Work

1. Read relevant documentation in docs/
2. Check [openspec/project.md](../openspec/project.md) for conventions
3. Review existing code patterns
4. Understand the security-focused domain context

### Code Quality Standards

- Run `make check` before committing (fmt + clippy)
- Follow Rust naming conventions
- Use `anyhow` for error handling
- Keep modules focused and separated

### Important Constraints

- **Security & Ethics**: For authorized testing only
- **API Key Security**: Never commit API keys
- **Read-Only Tool**: Provides commands but never executes them
- **Binary Portability**: ONNX Runtime library must accompany binary

## Working with This Project

### Simple Changes

For bug fixes, typos, or small improvements:
- Make the changes directly
- Run `make check` to validate
- Test with `cargo test` and `cargo build`

### Feature Development

For new features or significant changes:
- **Use OpenSpec workflow** (see [/openspec/AGENTS.md](../openspec/AGENTS.md))
- Create proposals in `openspec/changes/`
- Write specifications before implementing
- Follow the three-stage workflow (Create → Implement → Archive)

### Testing

```bash
cargo test          # Run all tests
make check          # Fmt + clippy
make build          # Build release binary
```

## Domain-Specific Knowledge

- **Command-First Philosophy**: Prioritize executable commands over explanations
- **Learn Mode**: Educational mode (`--learn`) for detailed breakdowns
- **Provider Support**: Perplexity Sonar, Groq, and Ollama
- **Semantic Cache**: Vector similarity using ONNX embeddings
- **Security Context**: Tool designed for professional security work

## File Locations

```
cyx/
├── src/              # Rust source code
├── data/             # Embedded data (normalization, models)
├── tests/            # Integration tests
├── docs/             # All documentation (you are here)
├── scripts/          # Build and install scripts
└── openspec/         # Spec-driven development system
```

## Getting Help

- Read the [README.md](../README.md) for project overview
- Check [BUILDING.md](BUILDING.md) for build issues
- Review [openspec/project.md](../openspec/project.md) for conventions
- For OpenSpec workflow, see [openspec/AGENTS.md](../openspec/AGENTS.md)
