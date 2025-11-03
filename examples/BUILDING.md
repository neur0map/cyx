# Building and Distributing CYX

This document explains how to build and distribute CYX, including handling of runtime dependencies.

## Overview

CYX uses ONNX Runtime for semantic search capabilities. This library is distributed as a shared library (`.so` on Linux, `.dylib` on macOS, `.dll` on Windows) that must be available at runtime.

## Runtime Dependency: ONNX Runtime

The project uses the `ort` crate with the `download-binaries` feature, which automatically downloads pre-built ONNX Runtime binaries during the build process. These binaries are:

- **Linux**: `libonnxruntime.so.1.16.0`
- **macOS**: `libonnxruntime.1.16.0.dylib`
- **Windows**: `onnxruntime.dll`

The `build.rs` file sets the runtime library search path (rpath) to look for libraries in the same directory as the binary (`$ORIGIN` on Linux, `@executable_path` on macOS).

## Building from Source

### Development Build

For local development:

```bash
# Build in debug mode (faster compilation)
make build-dev

# Build in release mode and create system symlink
make build

# Run directly from source
cargo run
```

### Release Build

For creating a distributable release package:

```bash
# Full release build with tests, checks, and packaging
make release

# Or just create the package
make package
```

This will create a distributable archive in the `dist/` directory containing:
- The `cyx` binary
- The ONNX Runtime shared library
- Installation instructions

## Distribution

### Automated Releases (GitHub Actions)

Releases are automatically built for multiple platforms when you push a version tag:

```bash
git tag v0.2.1
git push origin v0.2.1
```

The GitHub Actions workflow will:
1. Build binaries for Linux (x86_64, aarch64) and macOS (x86_64, aarch64)
2. Bundle each binary with its required ONNX Runtime library
3. Create release packages (`.tar.gz`)
4. Generate SHA256 checksums
5. Upload to GitHub Releases

### Manual Release Package Creation

To manually create a release package:

```bash
# Build the release binary
cargo build --release

# Create the package
bash scripts/package-release.sh
```

The package will be created in `dist/cyx-v{VERSION}-{PLATFORM}.tar.gz`

## Installation Methods

### For Users - Pre-built Binaries

Users downloading pre-built binaries should extract the archive and follow the included `README.txt`. The library must be kept in the same directory as the binary or installed to a system library directory.

**System-wide installation (Linux/macOS):**

```bash
tar -xzf cyx-v0.2.0-linux-x86_64.tar.gz
cd cyx-v0.2.0-linux-x86_64
sudo cp cyx /usr/local/bin/
sudo cp libonnxruntime.so.1.16.0 /usr/local/lib/
sudo ldconfig  # Linux only
```

**Local installation (Linux/macOS):**

```bash
tar -xzf cyx-v0.2.0-linux-x86_64.tar.gz
cd cyx-v0.2.0-linux-x86_64
mkdir -p ~/.local/bin
cp cyx ~/.local/bin/
cp libonnxruntime* ~/.local/bin/

# Add to PATH if not already there
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### For Developers - Building from Source

Developers can use `cargo install`:

```bash
cargo install --path .
```

**Important**: When using `cargo install`, you need to manually copy the ONNX Runtime library to the same directory as the installed binary:

```bash
# Find where cargo installs binaries (usually ~/.cargo/bin)
INSTALL_DIR=$(dirname $(which cyx))

# Copy the library
cp target/release/libonnxruntime* $INSTALL_DIR/
```

Or use the Makefile for a simpler installation:

```bash
make install
```

This will build and install using `cargo install`, then attempt to copy the required library.

## Troubleshooting

### Error: libonnxruntime.so.1.16.0: cannot open shared object file

This error means the ONNX Runtime library is not in the library search path. Solutions:

1. **Ensure the library is in the same directory as the binary** (recommended for portable installations)
2. **Install the library system-wide**:
   ```bash
   sudo cp libonnxruntime.so.1.16.0 /usr/local/lib/
   sudo ldconfig
   ```
3. **Add the library directory to `LD_LIBRARY_PATH`** (not recommended):
   ```bash
   export LD_LIBRARY_PATH=/path/to/library:$LD_LIBRARY_PATH
   ```

### Verifying Library Dependencies

**Linux:**
```bash
ldd ./cyx
```

**macOS:**
```bash
otool -L ./cyx
```

Both commands should show the ONNX Runtime library with either a relative path or `@rpath`.

## Cross-Compilation

The GitHub Actions workflow demonstrates cross-compilation for different platforms. For manual cross-compilation:

```bash
# Install target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation toolchain (Linux ARM64 example)
sudo apt-get install gcc-aarch64-linux-gnu

# Build
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
  cargo build --release --target aarch64-unknown-linux-gnu
```

Note: Cross-compilation with the `ort` crate requires the appropriate ONNX Runtime binaries for the target platform.

## Static Linking Alternative

If you want to avoid the shared library dependency entirely, you would need to:

1. Build ONNX Runtime from source with static linking
2. Use the `ort` crate's build-from-source feature
3. Configure static linking in `build.rs`

This is more complex and increases binary size significantly (~60MB+), so the current dynamic linking approach is preferred.

## License

MIT - See LICENSE file for details.
