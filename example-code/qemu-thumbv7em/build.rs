use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    // put memory layout (linker script) in the linker search path as the
    // package root isn't always searched
    fs::copy("memory.x", out_dir.join("memory.x"))?;
    fs::copy("device.x", out_dir.join("device.x"))?;
    // important - if the file changes, re-run the build
    println!("cargo::rerun-if-changed=device.x");
    println!("cargo::rerun-if-changed=memory.x");
    // tell the linker where to find them
    println!("cargo::rustc-link-search={}", out_dir.display());
    Ok(())
}
