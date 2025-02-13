fn main() {
    // Filters are extracted from `libc` filters
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Missing CARGO_CFG_TARGET_OS envvar");
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("Missing CARGO_CFG_TARGET_ARCH envvar");
    if !["android", "linux", "l4re"].contains(&target_os.as_str()) && !"wasm32".eq(&target_arch) {
        eprintln!("Building {} on an for a unsupported platform. Currently only linux and android are supported", env!("CARGO_PKG_NAME"));
        eprintln!("(Your current target_os is {})", target_os);
        std::process::exit(1)
    }
}
