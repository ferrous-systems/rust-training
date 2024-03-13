//! # Build script for the QEMU Ferrocene demo project
//!
//! This script only executes when using `cargo` to build the project.

use std::io::Write;

fn main() {
    // Find the right tools.
    let linker = std::env::var("RUSTC_LINKER").unwrap();
    let aarch64_as = linker.replace("gcc", "as");
    let aarch64_ar = linker.replace("gcc", "ar");

    // Put `linker.ld` file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("linker.ld"))
        .unwrap()
        .write_all(include_bytes!("linker.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Assembly src/boot.S
    let boot_object = out.join("boot.o");
    let output = std::process::Command::new(&aarch64_as)
        .arg("src/boot.S")
        .arg("-o")
        .arg(&boot_object)
        .output()
        .map(|h| h.status.success());
    match output {
        Ok(true) => {
            // Ran OK
        }
        Ok(false) => {
            // Didn't launch
            panic!("Failed to launch {aarch64_as}");
        }
        Err(e) => {
            // Failed to run
            panic!("Failed to run {aarch64_as}: {e:?}");
        }
    }
    let libboot_file = out.join("libboot.a");
    let output = std::process::Command::new(&aarch64_ar)
        .arg("rcs")
        .arg(&libboot_file)
        .arg(&boot_object)
        .output()
        .map(|h| h.status.success());
    match output {
        Ok(true) => {
            // Ran OK
        }
        Ok(false) => {
            // Didn't launch
            panic!("Failed to launch {aarch64_ar}");
        }
        Err(e) => {
            // Failed to run
            panic!("Failed to run {aarch64_ar}: {e:?}");
        }
    }
    // Link against our libboot.a library
    println!("cargo:rustc-link-lib=static=boot");
}
