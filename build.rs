fn main() {
    // Set rpath for macOS to find ONNX Runtime library in the same directory as the binary
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
    }

    // For Linux, set rpath to $ORIGIN
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }
}
