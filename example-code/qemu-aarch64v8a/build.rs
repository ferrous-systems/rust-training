//! # Build script for the QEMU Ferrocene demo project
//!
//! This script only executes when using `cargo` to build the project.

use std::io::Write;

fn main() {
    // Put `memory.ld` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("memory.ld"))
        .unwrap()
        .write_all(include_bytes!("memory.ld"))
        .unwrap();
    println!("cargo:rerun-if-changed=memory.ld");
    println!("cargo:rustc-link-arg=-Tmemory.ld");
    println!("cargo:rustc-link-arg=-Timage.ld");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-link-search={}", out.display());
}
