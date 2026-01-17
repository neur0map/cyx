# Building from Source

This document explains how to build cyx from source and publish to crates.io.

## Overview

Cyx uses TF-IDF based vector similarity for cache matching. All dependencies are managed by cargo.

## Development Build

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run directly
cargo run -- "test query"

# Or use Makefile shortcuts
make build      # Build release + create symlink
make build-dev  # Build debug mode
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Or use Makefile
make test
```

## Code Quality

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Or use Makefile for both
make check
```

## Local Installation

Install from source to test before publishing:

```bash
# Install from current directory
cargo install --path .

# Or use Makefile
make install
```

## Publishing to Crates.io

### Prerequisites

1. **Crates.io account**: Sign up at [crates.io](https://crates.io)
2. **Login**: `cargo login` (one-time setup)
3. **Clean build**: Ensure all tests pass

### Pre-publish Checklist

```bash
# 1. Update version in Cargo.toml
vim Cargo.toml  # Bump version

# 2. Update CHANGELOG
vim docs/CHANGELOG.md

# 3. Run quality checks
make check

# 4. Run all tests
cargo test

# 5. Test local install
cargo install --path .
cyx "test query"  # Verify it works

# 6. Dry run publish (check for issues)
cargo publish --dry-run

# 7. Review package contents
cargo package --list
```

### Publish

```bash
# Publish to crates.io
cargo publish

# Tag the release
git tag v0.2.2
git push origin v0.2.2
```

### Post-Publish

1. Verify on crates.io: https://crates.io/crates/cyx
2. Test install: `cargo install cyx`
3. Update README badge if version changed
4. Announce in changelog/release notes

## Version Management

Follow semantic versioning:
- **Major** (1.0.0): Breaking API changes
- **Minor** (0.2.0): New features, backward compatible
- **Patch** (0.2.1): Bug fixes, backward compatible

Update version in:
- `Cargo.toml` (line 3)
- `docs/CHANGELOG.md` (add entry)

## Troubleshooting

### Publish Errors

Common issues:

1. **Version already exists**: Bump version in Cargo.toml
2. **Missing metadata**: Ensure Cargo.toml has description, license, repository
3. **Large package**: Check `cargo package --list` for unexpected files

### Package Size

Current package size: ~50KB (source only, no binaries)

All dependencies are managed by cargo during installation.

## Makefile Commands

```bash
make help        # Show all commands
make build       # Release build + symlink
make build-dev   # Debug build
make install     # Install to system
make uninstall   # Remove from system
make test        # Run tests
make check       # Format + clippy
make clean       # Clean build artifacts
make setup       # Run setup wizard
```

## Development Workflow

```bash
# 1. Make changes
vim src/...

# 2. Format and check
make check

# 3. Test
make test

# 4. Build and test locally
make build
./bin/cyx "test query"

# 5. Commit
git add -A
git commit -m "Description"
git push
```

## Release Workflow

```bash
# 1. Update version
vim Cargo.toml docs/CHANGELOG.md

# 2. Quality checks
make check && cargo test

# 3. Test local install
cargo install --path .
cyx "test query"

# 4. Commit version bump
git add Cargo.toml docs/CHANGELOG.md
git commit -m "Bump version to v0.2.2"
git push

# 5. Publish to crates.io
cargo publish

# 6. Tag release
git tag v0.2.2
git push origin v0.2.2
```

## Dependencies

Key runtime dependencies managed in `Cargo.toml`:
- `reqwest` - HTTP client for API calls
- `rusqlite` - SQLite for cache (bundled)
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `serde` - Serialization
- `bincode` - Binary encoding for cache

All dependencies are automatically handled by cargo.

## License

MIT - See LICENSE file for details.
