//! # Build script for the QEMU Ferrocene demo project
//!
//! This script only executes when using `cargo` to build the project.

fn main() {
    // Put `memory.ld` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out.join("memory.x"), include_bytes!("memory.x")).expect("Writing memory.x");
    println!("cargo::rerun-if-changed=memory.x");
    std::fs::write(out.join("device.x"), include_bytes!("device.x")).expect("Writing device.x");
    println!("cargo::rerun-if-changed=device.x");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-link-search={}", out.display());
}
