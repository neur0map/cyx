use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub struct OnnxLibraryFixer;

impl OnnxLibraryFixer {
    /// Check if ONNX library is available and working
    pub fn check_onnx_library() -> Result<bool> {
        // Try to load the ONNX runtime library
        // The ort crate will attempt to load it automatically
        // If it fails, we can catch it here

        // Try to detect common ONNX library errors by checking if library exists
        let binary_dir = Self::get_binary_directory()?;
        let lib_name = Self::get_library_name();

        let lib_path = binary_dir.join(&lib_name);

        Ok(lib_path.exists())
    }

    /// Auto-detect and fix ONNX library issues
    pub fn auto_fix() -> Result<bool> {
        println!("{}", "🔍 Checking ONNX Runtime library...".cyan());

        if Self::check_onnx_library()? {
            println!("{}", "✓ ONNX Runtime library found".green());
            return Ok(true);
        }

        println!(
            "{}",
            "⚠ ONNX Runtime library not found in binary directory".yellow()
        );
        println!("{}", "  Attempting to fix automatically...".cyan());

        // Try to find and copy the library
        if let Ok(true) = Self::find_and_copy_library() {
            println!("{}", "✓ ONNX Runtime library fixed successfully!".green());
            return Ok(true);
        }

        // If auto-fix fails, provide manual instructions
        Self::show_manual_fix_instructions();
        Ok(false)
    }

    /// Find the ONNX library and copy it to the binary directory
    fn find_and_copy_library() -> Result<bool> {
        let binary_dir = Self::get_binary_directory()?;
        let lib_name = Self::get_library_name();
        let target_path = binary_dir.join(&lib_name);

        // Strategy 1: Check if we're running from a cargo build directory
        if let Ok(source_path) = Self::find_in_cargo_build() {
            println!("  → Found in cargo build: {}", source_path.display());
            return Self::copy_library(&source_path, &target_path);
        }

        // Strategy 2: Check cargo registry cache
        if let Ok(source_path) = Self::find_in_cargo_cache() {
            println!("  → Found in cargo cache: {}", source_path.display());
            return Self::copy_library(&source_path, &target_path);
        }

        // Strategy 3: Check system library directories
        if let Ok(source_path) = Self::find_in_system_libs() {
            println!("  → Found in system libs: {}", source_path.display());
            return Self::copy_library(&source_path, &target_path);
        }

        Ok(false)
    }

    /// Find library in cargo build directory (for development builds)
    fn find_in_cargo_build() -> Result<PathBuf> {
        let binary_path = std::env::current_exe()?;
        let binary_dir = binary_path.parent().context("No parent directory")?;

        // Check if we're in a target directory structure
        let lib_name = Self::get_library_name();
        let lib_path = binary_dir.join(&lib_name);

        if lib_path.exists() {
            return Ok(lib_path);
        }

        // Try target/release or target/debug
        if let Some(target_dir) = binary_dir.parent() {
            if target_dir.file_name() == Some(std::ffi::OsStr::new("release"))
                || target_dir.file_name() == Some(std::ffi::OsStr::new("debug"))
            {
                let release_lib = target_dir.join("release").join(&lib_name);
                if release_lib.exists() {
                    return Ok(release_lib);
                }

                let debug_lib = target_dir.join("debug").join(&lib_name);
                if debug_lib.exists() {
                    return Ok(debug_lib);
                }
            }
        }

        anyhow::bail!("Not found in cargo build directory")
    }

    /// Find library in cargo cache
    fn find_in_cargo_cache() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Cannot determine home directory")?;
        let cargo_home = std::env::var("CARGO_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".cargo"));

        let lib_name = Self::get_library_name();

        // Search in cargo registry
        let registry_path = cargo_home.join("registry").join("src");

        if registry_path.exists() {
            // Search for onnxruntime library in downloaded crates
            for entry in walkdir::WalkDir::new(registry_path)
                .max_depth(4)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_name() == lib_name.as_str() {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }

        anyhow::bail!("Not found in cargo cache")
    }

    /// Find library in system library directories
    fn find_in_system_libs() -> Result<PathBuf> {
        let lib_name = Self::get_library_name();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let search_paths = vec![
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/usr/lib"),
            PathBuf::from("/lib"),
        ];

        #[cfg(target_os = "windows")]
        let search_paths = vec![
            PathBuf::from("C:\\Windows\\System32"),
            PathBuf::from("C:\\Windows\\SysWOW64"),
            std::env::var("PROGRAMFILES")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("C:\\Program Files")),
        ];

        for path in search_paths {
            let lib_path = path.join(&lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }
        }

        anyhow::bail!("Not found in system library directories")
    }

    /// Copy library to target location
    fn copy_library(source: &Path, target: &Path) -> Result<bool> {
        println!("  → Copying to: {}", target.display().to_string().dimmed());

        fs::copy(source, target).context("Failed to copy library")?;

        // Also copy symlinks if they exist
        Self::copy_library_symlinks(source)?;

        Ok(true)
    }

    /// Copy library symlinks (e.g., libonnxruntime.so -> libonnxruntime.so.1.16.0)
    fn copy_library_symlinks(source: &Path) -> Result<()> {
        let binary_dir = Self::get_binary_directory()?;
        let source_dir = source.parent().context("No source parent directory")?;

        #[cfg(target_os = "linux")]
        {
            let symlink = source_dir.join("libonnxruntime.so");
            if symlink.exists() {
                let target = binary_dir.join("libonnxruntime.so");
                fs::copy(&symlink, &target).ok();
            }
        }

        #[cfg(target_os = "macos")]
        {
            let symlink = source_dir.join("libonnxruntime.dylib");
            if symlink.exists() {
                let target = binary_dir.join("libonnxruntime.dylib");
                fs::copy(&symlink, &target).ok();
            }
        }

        Ok(())
    }

    /// Get the expected library name for the current platform
    fn get_library_name() -> String {
        #[cfg(target_os = "linux")]
        return "libonnxruntime.so.1.16.0".to_string();

        #[cfg(target_os = "macos")]
        return "libonnxruntime.1.16.0.dylib".to_string();

        #[cfg(target_os = "windows")]
        return "onnxruntime.dll".to_string();
    }

    /// Get the directory containing the current binary
    fn get_binary_directory() -> Result<PathBuf> {
        let binary_path = std::env::current_exe()?;
        let binary_dir = binary_path
            .parent()
            .context("Cannot determine binary directory")?
            .to_path_buf();
        Ok(binary_dir)
    }

    /// Show manual fix instructions if auto-fix fails
    fn show_manual_fix_instructions() {
        println!();
        println!(
            "{}",
            "❌ Could not automatically fix the ONNX library issue.".red()
        );
        println!();
        println!("{}", "Manual Fix Instructions:".bold().yellow());
        println!();

        #[cfg(target_os = "linux")]
        {
            println!("  If you installed via cargo, run:");
            println!("{}", "  $ cargo build --release".cyan());
            println!(
                "{}",
                "  $ cp target/release/libonnxruntime.so.1.16.0 $(dirname $(which cyx))/".cyan()
            );
            println!();
            println!("  Or install system-wide:");
            println!(
                "{}",
                "  $ sudo cp target/release/libonnxruntime.so.1.16.0 /usr/local/lib/".cyan()
            );
            println!("{}", "  $ sudo ldconfig".cyan());
        }

        #[cfg(target_os = "macos")]
        {
            println!("  If you installed via cargo, run:");
            println!("{}", "  $ cargo build --release".cyan());
            println!(
                "{}",
                "  $ cp target/release/libonnxruntime.1.16.0.dylib $(dirname $(which cyx))/".cyan()
            );
            println!();
            println!("  Or install system-wide:");
            println!(
                "{}",
                "  $ sudo cp target/release/libonnxruntime.1.16.0.dylib /usr/local/lib/".cyan()
            );
        }

        #[cfg(target_os = "windows")]
        {
            println!("  If you installed via cargo, run:");
            println!("{}", "  > cargo build --release".cyan());
            println!(
                "{}",
                "  > copy target\\release\\onnxruntime.dll %USERPROFILE%\\.cargo\\bin\\".cyan()
            );
            println!();
            println!("  Or in PowerShell:");
            println!("{}", "  PS> cargo build --release".cyan());
            println!(
                "{}",
                "  PS> Copy-Item target\\release\\onnxruntime.dll $env:USERPROFILE\\.cargo\\bin\\"
                    .cyan()
            );
        }

        println!();
        println!("{}", "For more help, see:".dimmed());
        println!(
            "{}",
            "  https://github.com/neur0map/cyx#troubleshooting".dimmed()
        );
    }
}
