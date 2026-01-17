# Development Guide

Guide for contributors and developers working on cyx.

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git
- Make (optional, for convenience commands)

### Getting Started

```bash
# Clone repository
git clone https://github.com/neur0map/cyx.git
cd cyx

# Build project
make build

# Or use cargo directly
cargo build --release
```

## Project Structure

```
cyx/
├── src/              # Source code
│   ├── cache/        # SQLite caching & vector similarity
│   ├── cli/          # Command-line argument parsing
│   ├── config/       # User configuration management
│   ├── deps/         # Dependency/LLM provider detection
│   ├── llm/          # LLM provider implementations
│   ├── search/       # Query normalization & similarity matching
│   ├── session/      # Interactive session management
│   └── ui/           # Terminal UI & formatting
├── data/             # Embedded data files (normalization)
├── tests/            # Integration tests
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
```

### Before Committing

Always run quality checks before committing:

```bash
make check  # Runs fmt check + clippy
```

## Code Style Conventions

- **Formatter**: `cargo fmt` with standard Rust conventions
- **Line width**: 100 characters (configured in .editorconfig)
- **Error handling**: Use `anyhow::Result<T>` for functions
- **Async**: Use `tokio` for async operations
- **CLI**: Use `clap` with derive macros

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_cache_similarity

# Or use Makefile
make test
```

### Test Organization

- Unit tests: Inside modules with `#[cfg(test)]`
- Integration tests: In `tests/` directory
- Use `tempfile` crate for temporary directories in tests

## Release Process

### Before Release

1. Update version in `Cargo.toml`
2. Update `docs/CHANGELOG.md`
3. Run `make check`
4. Run `cargo test`
5. Test local install: `cargo install --path .`

### Creating Release

```bash
# Build release
make build

# Test functionality
./target/release/cyx "test query"

# Commit version bump
git add Cargo.toml docs/CHANGELOG.md
git commit -m "Bump version to v0.2.2"
git push

# Create tag
git tag v0.2.2
git push origin v0.2.2

# Publish to crates.io
cargo publish
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `make check` before committing
5. Submit a pull request

## Data Files

Critical data files (normalization) are embedded in the binary at compile time for portable execution:
- `data/normalization/stopwords.json` - Common words to remove
- `data/normalization/abbreviations.json` - Security term expansions

## Adding Features

### New LLM Provider

1. Create new module in `src/llm/`
2. Implement `LLMProvider` trait
3. Add variant to `LLMProvider` enum in `src/config/mod.rs`
4. Update CLI args and config handling
5. Add tests

### New Cache Features

1. Modify `src/cache/storage.rs` for database schema
2. Update migration logic
3. Add tests
4. Update documentation

## Performance Considerations

- Cache hit latency: < 10ms (hash match)
- Vector similarity: 50-100ms
- API calls: 2-5 seconds
- Storage: ~100-500 KB per cached query

## Troubleshooting

### Build Errors

- Ensure Rust 1.70+ is installed
- Run `cargo clean && cargo build` for clean build
- Check `Cargo.lock` is up to date

### Test Failures

- Ensure no API keys are hardcoded in tests
- Use mocks for external services
- Run `cargo test -- --nocapture` for verbose output

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [clap Documentation](https://docs.rs/clap/)
- [tokio Documentation](https://docs.rs/tokio/)
- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/cargo/package-layouts.html)
