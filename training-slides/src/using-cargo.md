# Using Cargo

# Crates and Packages

* Rust code is arranged into packages
* a package is described by a `Cargo.toml` file
* building a package can produce a single library, and 0 or more executables
  * these are called *crates*
  * unlike C/C++ compilers that compile code file by file, `rustc` treat all files for a crate as a single compilation unit
* Cargo calls `rustc` to build each crate in the package.

## Cargo

* standard build toolchain for Rust projects
* shipped with `rustc`

## What Cargo does

* resolves and installs project dependencies
* runs `rustc` to compile your code
* runs a linker to produce libraries and executables
* runs tests and benchmarks
* builds documentation and runs documentation tests
* runs additional tools like code formatter and linter
* can be extended with additional custom commands

## Cargo does Everything!

## Cargo commands

* `cargo new my-app`
* `cargo run` - runs a debug build of your program, builds it if necessary
* `cargo fmt` - formats your code
* `cargo check` - only reports errors, doesn't actually compile your code
* `cargo clippy` - runs a linter
* `cargo test` - builds your project if necessary and runs tests
  * by default runs unit tests, integration tests, and documentation tests
  * you can select which tests to run
* `cargo build --release` - produces an optimized version of your application or library

## Cargo commands (cont)

There are many more!

* `cargo bench` - builds an optimized version of your project and runs benchmarks
* `cargo doc --open` - builds documentation for your project *and all its dependencies* and opens it in a browser
* `cargo run --example ...` - runs an example from your `examples/` directory

See [Cargo Book](https://doc.rust-lang.org/cargo/commands/index.html) for more.

## Cargo command arguments

Most cargo commands accept a few common arguments:

* `+toolchain`
* `--target`
* `--features`, `--all-features`, and `--no-default-features`
* `--timings`

## Putting it all together:

`cargo +nightly run --target x86_64-apple-darwin --features "a b c dependency/feature" --timings`

* use nightly Rust
* enable features `a`, `b`, `c`, and a `feature` feature of a `dependency` crate
* (assuming we use Apple Silicon computer) build a macOS executable for x86 processor and run it using built-in emulation (Rosetta2)
* collect statistics during the build process and generate a report

## Features

* allows conditional compilation
  * support for different operating systems
  * adapters for different libraries
  * optional extensions
* can expose features from transitive dependencies

## Using Features

* in code:

    ```text
    #[cfg(feature = "json")]
    mod json_support;
    ```

* in `Cargo.toml`

    ```toml
    [features]
    json = [] # list of features that this feature depends on
    default = [] # "json" feature is not enabled by default
    ```

* when someone uses your dependency

    ```toml
    my-lib = { version: "1.0.0", features = ["json"] }
    ```

## Anatomy of Rust package

```shell
cargo new hello-world
```

```text
├── Cargo.lock
├── Cargo.toml
└── src/
    └── main.rs
```

## Anatomy of Rust package 2

```text
├── Cargo.lock
├── Cargo.toml
├── build.rs
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── ...
│   └── bin/
│       ├── additional-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── ...
├── benches/
│   └── ...
├── examples/
│   └── ...
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── ...
```

## Cargo.toml - A manifest file

```toml
[package]
name = "tcp-mailbox"
version = "0.1.0"

[dependencies]
async-std = "1" # would also choose 1.5
clap = "2.2" # would also choose 2.3
```
