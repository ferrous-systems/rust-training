//! The build script for libdatabase-sys
//! 
//! > Copyright (c) Ferrous Systems, 2026

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.rs");

    cc::Build::new()
        .file("../libdatabase/libdatabase.c")
        .compile("database");

    println!("cargo:rerun-if-changed=libdatabase.c");
    println!("cargo:rerun-if-changed=libdatabase.h");
}

// End of file
