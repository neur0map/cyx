# Changelog

All notable changes to cyx will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## v0.3.0 - 2025-01-17

### Changed - Major Simplification
**Simplified Architecture**:
- Removed ONNX Runtime dependency (was causing version conflicts)
- Removed external model download requirements
- Removed binary distribution complexity
- Reduced dependency count from ~20 to ~10
- Simplified installation to single `cargo install` command

**Cache System**:
- Switched to TF-IDF based vector similarity only
- Removed semantic embedding models (all-MiniLM, etc.)
- Kept normalization system (stopwords, abbreviations)
- Cache still provides 256D vector similarity search
- Faster startup (no model loading overhead)

**Installation**:
- One-command installation: `cargo install cyx`
- Setup wizard streamlined to 2 steps (provider, API key)
- No external library dependencies for cloud providers
- Removed release distribution and GitHub Actions workflows
- Reduced binary size significantly

**Removed**:
- `src/cache/embedder_onnx.rs` - ONNX embedder module
- `src/deps/onnx_fixer.rs` - Library fixer utility
- `data/embedding_models.json` - Model metadata
- `download-model` CLI command
- Release packaging scripts
- GitHub Actions workflow files

**Dependencies Removed**:
- `ort` 1.16 - ONNX Runtime bindings
- `ndarray` 0.15 - Array library (for embeddings)
- `tokenizers` 0.22 - HuggingFace tokenizers

**Dependencies Updated**:
- All remaining dependencies updated to latest versions
- Simplified Cargo.toml structure

**Technical**:
- Cache still functional with hash matching + TF-IDF similarity
- Query normalization preserves good cache hit rates
- No breaking changes to user-facing API or config format

**Documentation**:
- Updated README.md (removed emojis, simplified description)
- Updated all docs to remove ONNX references
- Updated installation and building guides

### Migration Notes

Existing installations continue to work:
- Config file format unchanged
- Cache database format unchanged
- Query normalization still active
- All cloud providers (Groq/Perplexity/Ollama) still work
- Only change: cache uses TF-IDF instead of semantic embeddings

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

## v0.2.1 - 2024-11-03

### Changed
- Major dependency updates (21 packages updated)
- Improved cache hit rates with normalization
- Better handling of similar queries

### Fixed
- ONNX Runtime library distribution issue
- Binary now shows helpful guidance instead of cryptic "library not found" errors

### Added
- Release packaging script that bundles binary with ONNX Runtime library
- GitHub Actions workflow for automated multi-platform releases

## v0.2.0 - 2024-10-20

### Added
- Initial release of cyx
- Multi-provider LLM support (Groq, Perplexity)
- Interactive setup wizard
- Learn mode with detailed explanations
- Smart cache system
- Ollama support for local models
- Source attribution with links
