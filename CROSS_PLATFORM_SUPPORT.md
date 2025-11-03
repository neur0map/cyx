# Cross-Platform Support for ONNX Auto-Fix

This document details how the ONNX library auto-fix feature works across different operating systems.

## Overview

The ONNX auto-fix feature is **fully cross-platform** and supports:
- ✅ **Linux** (x86_64, aarch64)
- ✅ **macOS** (x86_64, aarch64/Apple Silicon)
- ✅ **Windows** (x86_64)

## Platform-Specific Implementation

### Library Names

The auto-fix automatically detects the correct library name for each platform:

| Platform | Library Name |
|----------|--------------|
| Linux    | `libonnxruntime.so.1.16.0` |
| macOS    | `libonnxruntime.1.16.0.dylib` |
| Windows  | `onnxruntime.dll` |

### Search Strategies

All three strategies work cross-platform:

#### 1. Cargo Build Directory Search
- **Cross-platform**: ✅ Works on all OS
- Searches `target/release/` or `target/debug/`
- Uses Rust's standard library path handling
- Platform-agnostic implementation

#### 2. Cargo Cache Search  
- **Cross-platform**: ✅ Works on all OS
- Searches `$CARGO_HOME/registry/src/` (Unix) or `%CARGO_HOME%\registry\src\` (Windows)
- Uses `dirs` crate for cross-platform home directory detection
- Uses `walkdir` crate for cross-platform directory traversal

#### 3. System Library Search
- **Platform-specific**: Paths differ by OS

**Linux/macOS**:
```
/usr/local/lib
/usr/lib
/lib
```

**Windows**:
```
C:\Windows\System32
C:\Windows\SysWOW64
%PROGRAMFILES%
```

### Symlink Handling

On Unix systems (Linux/macOS), the auto-fix also copies version symlinks:

**Linux**:
- Copies `libonnxruntime.so` → `libonnxruntime.so.1.16.0`

**macOS**:
- Copies `libonnxruntime.dylib` → `libonnxruntime.1.16.0.dylib`

**Windows**:
- No symlinks (DLLs don't use symlinks)

### Manual Fix Instructions

If auto-fix fails, platform-specific instructions are shown:

#### Linux
```bash
$ cargo build --release
$ cp target/release/libonnxruntime.so.1.16.0 $(dirname $(which cyx))/

# Or system-wide:
$ sudo cp target/release/libonnxruntime.so.1.16.0 /usr/local/lib/
$ sudo ldconfig
```

#### macOS
```bash
$ cargo build --release
$ cp target/release/libonnxruntime.1.16.0.dylib $(dirname $(which cyx))/

# Or system-wide:
$ sudo cp target/release/libonnxruntime.1.16.0.dylib /usr/local/lib/
```

#### Windows (Command Prompt)
```cmd
> cargo build --release
> copy target\release\onnxruntime.dll %USERPROFILE%\.cargo\bin\
```

#### Windows (PowerShell)
```powershell
PS> cargo build --release
PS> Copy-Item target\release\onnxruntime.dll $env:USERPROFILE\.cargo\bin\
```

## Testing Cross-Platform Support

### On Linux
```bash
# Test with missing library
rm ~/.cargo/bin/libonnxruntime.so.1.16.0
cyx setup
# Should auto-detect and fix
```

### On macOS
```bash
# Test with missing library
rm ~/.cargo/bin/libonnxruntime.1.16.0.dylib
cyx setup
# Should auto-detect and fix
```

### On Windows
```powershell
# Test with missing library
Remove-Item $env:USERPROFILE\.cargo\bin\onnxruntime.dll
cyx setup
# Should auto-detect and fix
```

## Implementation Details

### Conditional Compilation

The code uses Rust's `cfg` attributes for platform-specific behavior:

```rust
#[cfg(target_os = "linux")]
fn get_library_name() -> String {
    "libonnxruntime.so.1.16.0".to_string()
}

#[cfg(target_os = "macos")]
fn get_library_name() -> String {
    "libonnxruntime.1.16.0.dylib".to_string()
}

#[cfg(target_os = "windows")]
fn get_library_name() -> String {
    "onnxruntime.dll".to_string()
}
```

### Cross-Platform Dependencies

The implementation uses only cross-platform crates:
- `std::path::PathBuf` - Cross-platform path handling
- `std::env::current_exe()` - Works on all platforms
- `dirs` crate - Cross-platform directory detection
- `walkdir` crate - Cross-platform directory traversal
- `colored` crate - Works on all platforms (with Windows terminal support)

## Limitations

### Windows-Specific Notes
1. **No symlinks**: Windows DLLs don't use symlink versioning
2. **PATH handling**: Windows uses different PATH separators (`;` vs `:`)
3. **Admin rights**: May need elevated privileges for system-wide installation

### Unix-Specific Notes
1. **ldconfig**: Linux requires `ldconfig` after system library installation
2. **Permissions**: May need sudo for `/usr/local/lib` installation

## Future Enhancements

Potential improvements for even better cross-platform support:

1. **Windows MSI Installer**: Create Windows-specific installer
2. **macOS .app Bundle**: Package as macOS application
3. **Linux Package Managers**: Add support for apt, yum, pacman
4. **Portable Mode**: Detect and handle portable installations
5. **Environment Detection**: Better detection of WSL vs native Windows

## Conclusion

The ONNX auto-fix feature is designed from the ground up to be cross-platform:
- ✅ Uses platform-agnostic Rust standard library
- ✅ Conditional compilation for platform-specific code
- ✅ Cross-platform crates (dirs, walkdir)
- ✅ Platform-specific manual instructions as fallback
- ✅ Tested on Linux, macOS, and Windows

**Result**: Users on any supported platform get the same automatic fix experience!
