# Changelog

## [Unreleased]

## v0.2.2 - 2025-01-17

### Changed - Major Installation Simplification

**Installation**: Simplified to cargo-only installation for extreme simplicity and ease of use
- **Removed**: GitHub shell installer (`install.sh`), release workflow, binary distribution
- **Single method**: `cargo install cyx` is now the only installation method
- **Setup time**: Reduced from 5-10 minutes to ~30 seconds for new users

**Setup Wizard**: Streamlined from 6 steps to 2 steps
- Removed dependency checking display
- Removed Ollama auto-installation prompts
- Auto-enabled cache with smart defaults (no prompts)
- Provider priority: Groq first (recommended), Perplexity second, Ollama last (advanced)

**Update System**: Simplified to message-only
- Removed binary downloader and installer modules
- Update command now displays: `cargo install cyx --force`
- Removed GitHub release checking (crates.io only)

**Provider Priority**: Cloud-first approach
- Groq marked as `[RECOMMENDED]` in setup wizard
- Perplexity as second option
- Ollama de-emphasized to "advanced, requires manual setup"
- Doctor command shows "no dependencies needed" for cloud providers

### Removed

- **Installation infrastructure**:
  - `scripts/install.sh` (250 lines)
  - `scripts/package-release.sh` (239 lines)
  - `scripts/cyx-launcher.sh` (4.4KB)
  - `.github/workflows/release.yml` (220 lines)
- **Update system modules**:
  - `src/update/installer.rs` (entire file)
  - `src/update/downloader.rs` (entire file)
  - `InstallSource` enum from metadata
- **Unused data files**:
  - `data/dependencies.json`
  - `data/ollama_models.json`
- **Dependencies**: tar, flate2, sha2 (no longer needed)

### Technical

- ONNX Runtime auto-handled by `ort` crate's `download-binaries` feature
- Cache embedding data (normalization, stopwords) still embedded in binary
- Config format unchanged (fully backward compatible)
- Ollama commands kept but marked as advanced features

### Documentation

- **README.md**: Complete installation section rewrite (cargo-only)
- **INSTALLATION.md**: Simplified from 155 to 120 lines
- **BUILDING.md**: Rewritten to focus on crates.io publishing (234 lines)
- **DEVELOPMENT.md**: Updated release workflow for cargo publishing
- All docs updated to reflect cloud-first, cargo-only approach

### Migration for Existing Users

Existing installations continue to work (config compatible). To migrate:
```bash
cargo install cyx  # New cargo-based installation
# Existing config at ~/.config/cyx/config.toml works unchanged
```

### Impact

- **New users**: Install to first query in ~60 seconds
- **Simplicity**: One command installation, two-step setup
- **Maintenance**: Eliminated ~1000 lines of installation/packaging code
- **Updates**: Simpler update path via cargo
- **Focus**: Cloud providers (Groq/Perplexity) prioritized over local models

## v0.2.1 - 2025-11-03

### Changed
- **Major dependency updates** (21 packages updated):
  - tokio: 1.40 → 1.48.0 (security fixes, performance improvements)
  - reqwest: 0.12 → 0.12.24 (HTTP client improvements)
  - clap: 4.5 → 4.5.51 (CLI framework updates)
  - serde: 1.0 → 1.0.228 (serialization improvements)
  - rusqlite: 0.31 → 0.37.0 (SQLite wrapper updates)
  - html2text: 0.12 → 0.16.0 (HTML parsing improvements)
  - tokenizers: 0.19 → 0.22.1 (HuggingFace tokenizers update)
  - dialoguer: 0.11 → 0.12.0 (interactive prompts)
  - indicatif: 0.17 → 0.18.2 (progress bars)
  - comfy-table: 7.1 → 7.2.1 (table rendering)
  - toml: 0.8 → 0.9.8 (config parsing)
  - anyhow: 1.0 → 1.0.100 (error handling)
  - regex: 1.10 → 1.12.2 (pattern matching)
  - chrono: 0.4 → 0.4.42 (datetime handling)
  - And 7 more minor updates

### Removed
- Unused dependencies: `thiserror`, `url`, `unicode-width` (3 packages removed)

### Fixed
- **ONNX Runtime library distribution**: Resolved critical issue where `libonnxruntime.so.1.16.0` was not bundled with releases
  - Added smart error detection with platform-specific installation instructions
  - Binary now shows helpful guidance instead of cryptic "library not found" errors
  - Enhanced error messages direct users to release downloads with exact installation commands
- html2text API compatibility: Updated to handle new Result-based API in v0.16

### Added
- Release packaging script (`scripts/package-release.sh`) that bundles binary with ONNX Runtime library
- GitHub Actions workflow for automated multi-platform releases (Linux x86_64/aarch64, macOS x86_64/aarch64)
- `BUILDING.md` - comprehensive guide for building, distributing, and troubleshooting
- Optional launcher script (`scripts/cyx-launcher.sh`) for pre-flight library checks
- Platform-specific installation instructions in release packages

### Technical
- **Makefile improvements**:
  - `make package` - creates distributable release packages with bundled libraries
  - `make install` - now automatically copies ONNX Runtime library alongside binary
  - `make release` - updated to include packaging step
- **README.md**: Added prominent warnings and instructions for `cargo install` users
- Release packages now include clear warnings that binary and library must stay together

### Documentation
- Updated README with pre-built binary installation as recommended method
- Added troubleshooting section for library loading errors
- Enhanced installation instructions with both system-wide and local options
- Added detailed build/distribution documentation

### Notes
- ndarray kept at 0.15 (ort 1.16 requires 0.15.x)
- ort kept at 1.16 (2.0 is RC, waiting for stable)
- bincode kept at 1.3 (2.0 has breaking cache format changes)

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
