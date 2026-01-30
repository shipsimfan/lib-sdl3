use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").expect("TARGET not set");
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    if target.contains("windows") {
        // Check for platform
        if !target.contains("windows-msvc") {
            panic!("building for Windows only supports MSVC");
        }

        if !target.starts_with("x86_64-") {
            panic!("building for Windows only supports x86-64");
        }

        // Build directory
        let lib_dir = manifest_dir
            .join("deps")
            .join("windows-msvc")
            .join("x86-64");
        if !lib_dir.exists() {
            panic!("missing SDL3 library folder at \"{}\"", lib_dir.display());
        }

        // Tell cargo to use it for libraries
        println!("cargo:rerun-if-changed={}", lib_dir.display());
        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        // Copy the .dll to the target directory
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let bin_dir = out_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .expect("unexpected OUT_DIR layout");

        let dll_src = lib_dir.join("SDL3.dll");
        let dll_dst = bin_dir.join("SDL3.dll");

        if dll_src.exists() {
            std::fs::copy(&dll_src, &dll_dst).ok();
        } else {
            println!("cargo:warning=SDL3.dll not found at {}", dll_src.display());
        }
    }
}
