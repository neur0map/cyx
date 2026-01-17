# Development Guide

Guide for contributors and developers working on cyx.

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git
- Make (optional, for convenience commands)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/neur0map/cyx.git
cd cyx

# Build the project
make build

# Or use cargo directly
cargo build --release
```

## Project Structure

```
cyx/
├── src/              # Source code
│   ├── cache/        # SQLite caching & semantic search
│   ├── cli/          # Command-line argument parsing
│   ├── config/       # User configuration management
│   ├── deps/         # Dependency/LLM provider detection
│   ├── llm/          # LLM provider implementations
│   ├── search/       # Query normalization & semantic matching
│   ├── session/      # Interactive session management
│   └── ui/           # Terminal UI & formatting
├── data/             # Embedded data files (normalization, models)
├── tests/            # Integration tests
├── scripts/          # Build and installation scripts
├── docs/             # Documentation
└── openspec/         # Spec-driven development workflow
```

## Code Quality

### Formatting

```bash
# Check formatting
cargo fmt -- --check

# Auto-fix formatting
make fmt
# or: cargo fmt
```

### Linting

```bash
# Run clippy
make check
# or: cargo clippy -- -D warnings

# Verbose checks
make check-verbose
```

### Before Committing

Always run quality checks before committing:

```bash
make check  # Runs fmt check + clippy
```

## Code Style Conventions

- **Formatter**: `cargo fmt` with standard Rust conventions
- **Linter**: `cargo clippy` with pedantic checks enabled
- **Module Structure**: Separate directories for each major component
- **Error Handling**: Use `anyhow` for ergonomic error propagation
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test normalizer

# Run with output
cargo test -- --nocapture
```

### Test Files

- Integration tests are in `tests/` directory
- Example files are in `examples/` directory (runnable binaries)

## Building

### Development Build

```bash
make build
# Creates release build and symlinks to ./cyx
```

### Release Build

```bash
cargo build --release
# Output: target/release/cyx
```

### Cross-Platform Builds

GitHub Actions automatically builds for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)

See `.github/workflows/release.yml` for details.

## Architecture Patterns

### Provider Pattern

LLM providers (Perplexity, Groq, Ollama) implement a common interface:
- Abstract provider interface for flexibility
- Easy to add new providers
- Consistent error handling across providers

### Semantic Caching

- ONNX-powered vector embeddings for intelligent cache lookups
- SQLite for persistent cache storage
- Query normalization for better matching

### Data Embedding

Critical data files (ONNX model, tokenizer, normalization data) are embedded in the binary at compile time for portable execution.

## Creating a Release

To create a new release with automated binary builds for all platforms:

```bash
# Update version in Cargo.toml
vim Cargo.toml  # Update version field

# Commit the version bump
git add Cargo.toml
git commit -m "Bump version to v0.2.2"

# Create and push a version tag
git tag v0.2.2
git push origin master
git push origin v0.2.2
```

This triggers the GitHub Actions workflow which will:
- Build binaries for Linux (x86_64, aarch64) and macOS (x86_64, aarch64)
- Bundle each binary with the ONNX Runtime library
- Include the install.sh script in release packages
- Create a GitHub Release with all artifacts
- Generate SHA256 checksums for verification

## Makefile Commands

```bash
make help          # Show all available commands
make build         # Build release and create symlink
make check         # Run fmt + clippy checks
make fmt           # Auto-format code
make install       # Install to system
make setup         # Run setup wizard
make clean         # Clean build artifacts
```

## Git Workflow

- **Main Branch**: `master`
- **Commit Style**: Descriptive messages with context
- **Pre-merge**: Run `make check` to validate code quality
- **Releases**: Tag-based (e.g., `v0.2.1`) trigger automated builds

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run `make check` to ensure code quality
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

MIT License - See [LICENSE](../LICENSE) for details.
