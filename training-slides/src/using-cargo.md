# Cargo Dependencies and Workspaces

# Crates and Packages

* Rust code is arranged into packages
* A package is described by a `Cargo.toml` file
* Building a package can produce a single library, and 0 or more executables
    * these build artifacts are called "crates"
    * unlike C/C++ compilers that compile code file by file, `rustc` treat all files for a crate as a single compilation unit
* Cargo calls `rustc` to build each crate in the package.

## Cargo

* Standard build toolchain for Rust projects
* Shipped with `rustc`

## What Cargo does

* Resolves and installs project dependencies
* Runs `rustc` to compile your code
* Runs a linker to produce libraries and executables
* Runs tests and benchmarks
* Builds documentation and runs documentation tests
* Runs additional tools like code formatter and linter
* Can be extended with additional custom commands

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

* There are many more!
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

## Anatomy of Rust package

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

## Manifest file

```toml
[package]
name = "tcp-mailbox"
version = "0.1.0"

[dependencies]
async-std = "1" # would also choose 1.5
clap = "2.2" # would also choose 2.3
```

## Lock file

* contains a list of all project dependencies, de-facto versions and hashes of downloaded dependencies
* when a version is *yanked* from `Crates.io` but you have the correct hash for it in a lock file Cargo will still let you download it and use it.
    * still gives you warning about that version being problematic
* should be committed to your repository for applications

## Dependency resolution

* uses "Zero-aware" SemVer for versioning
    * `1.3.5` is compatible with versions `>= 1.3.5` and `< 2.0.0`
    * `0.3.5` is compatible with versions `>= 0.3.5` and `< 0.4.0`
    * `0.0.3` only allows `0.0.3`
* allows version-incompatible transitive dependencies
    * except C/C++ dependencies
* combines dependencies with compatible requirements as much as possible
* allows path, git, and custom registry dependencies

## How a dependency version is selected

* for every requirement Cargo selects acceptable version intervals
    * `[1.1.0; 1.6.0)`, `[1.3.5, 2.0.0)`, `[2.0.0; 3.0.0)`
* Cargo checks for interval intersections to reduce the number of unique intervals
    * `[1.3.5; 1.6.0)`, `[2.0.0; 3.0.0)`
* for every unique interval it selects the most recent available version
    * `=1.5.18`, `=2.7.11`
* selected versions and corresponding package hashes are written into `Cargo.lock`

## Dependency resolution: Example

```text
└── my-app                      May install:
    ├── A = "1"
    │   ├── X = "1"             A = "1.0.17"
    │   └── Y = "1.3"     =>    B = "1.5.0"
    └── B = "1"                 X = "2.0.3"
        ├── X = "2"             X = "1.2.14"
        └── Y = "1.5"           Y = "1.8.5"
```

## Crates.io

* default package registry
    * 100k crates and counting
    * **every Rust Beta release is tested against all of them every week**
* packages aren't deleted, but *yanked*
    * if you have a correct hash for a yanked version in your `Cargo.lock` your build won't break (you still get a warning)

## Docs.rs

* **Complete API documentation for the whole Rust ecosystem**
* Automatically publishes API documentation for every version of every crate on Crates.io
* Documentation for old versions stays up, too. Easy to switch between versions.
* Links across crates just work

## Other kinds of dependencies

* Git dependencies
    * both `git+https` and `git+ssh` are allowed
    * can specify branch, tag, commit hash
    * when downloaded by Cargo exact commit hash used is written into `Cargo.lock`
* Path dependencies
    * both relative and absolute paths are allowed
    * common in workspaces

## C Libraries as dependencies

* Rust can call functions from C libraries using `unsafe` code
    * integrate with operating system APIs, frameworks, SDKs, etc.
    * talk to custom hardware
    * reuse existing code (SQLite, OpenSSL, libgit2, etc.)
* Building a crate that relies on C libraries often requires customization
    * done using `build.rs` file

## `build.rs` file

* compiled and executed before the rest of the package
* can manipulate files, execute external programs, etc.
    * download / install custom SDKs
    * call `cc`, `cmake`, etc. to build C++ dependencies
    * execute `bindgen` to generate Rust bindings to C libraries
* output can be used to set Cargo options dynamically
    ```rust ignore
    println!("cargo:rustc-link-lib=gizmo");
    println!("cargo:rustc-link-search=native={}/gizmo/", library_path);
    ```

## `-sys` crates

* Often Rust libraries that integrate with C are split into a pair of crates:
    * `library-name-sys`
        * thin wrapper around C functions
        * often all code is autogenerated by `bindgen`
    *  `library-name`
        * depends on `library-name-sys`
        * exposes convenient and idiomatic Rust API to users
* Examples:
    * `openssl` and `openssl-sys`
    * `zstd` and `zstd-sys`
    * `rusqlite` and `libsqlite3-sys`

## Cargo Workspaces

## Cargo Workspaces

Allow you to split your project into several packages

* Further encourages modularity
* Develop multiple applications and libraries in a single tree
* Synchronized dependency management, release process, etc.
* Way to parallelize compilation and speed up builds
* **Your internal projects should likely be workspaces** even if you don't use monorepos

## Anatomy of Rust Workspace

```text
my-app/
├── Cargo.toml   # a special workspace file
├── Cargo.lock   # notice that Cargo produces a common lockfile for all packages
├── packages/      # can use any directory structure
│   ├── main-app/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   ├── admin-app/
│   │   └── ...
│   ├── common-data-model/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── useful-macros
│   ├── service-a
│   ├── service-b
│   └── ...
└── tools/       # packages don't have to be in the same directory
    ├── release-bot/
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
    ├── data-migration-scripts/
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
    └── ...
```

## Workspace Cargo.toml

```toml
[workspace]
members = ["packages/*", "tools/*"]

[dependencies]
thiserror = "1.0.39"
...
```

using wildcards for members is very handy when you want to add new member packages, split packages, etc.

## Cargo.toml for a workspace member

```toml
[package]
name = "main-app"

[dependencies]
thiserror = { workspace = true }
service-a = { path = "../service-a" }
...
```

## Cargo commands for workspaces

* `cargo run --bin main-app`
* `cargo test -p service-a`

## Appendix: Rust projects build time

## Understanding Rust projects build time

* Cargo keeps track of changes you make and only rebuilds what is necessary
* When building a crate `rustc` can do most of work in parallel, but some steps still require synchronization
* Depending on type of a build times spent in different build phases may be vastly different.
    * debug vs release
    * various flags for `rustc` and LLVM
    * a build from scratch vs an incremental build

## Producing a build timings report

`rm -rf target/debug && cargo build --timings`

```text
.
└── target/
    ├── cargo-timings/
    │   ├── cargo-timings.html
    │   └── cargo-timings-<timestamp>.html
    ├── debug/
    └── ...
```

## Timings Report

![Cargo Build Report for Rust Analyzer](./images/rust-analyzer-cargo-build-timings.png)

## Reading the report

* Cargo can't start building a crate until all its dependencies have been built.
    * Cargo only waits for `rustc` to produce an LLVM IR, further compilation by LLVM can run in background (purple)
* A crate can't start building until its `build.rs` is built and finishes running (yellow)
* If multiple crates depend on a single crate they often can start building in parallel
* If a package is both a binary and a library then the binary is built after a library
    * Integration tests, examples, benchmarks, and documentation tests all produce binaries and thus take extra time to build.

## Actions you can take

## Keep your crates independent of each other

* Bad dependency graph:
    ```text
    D -> C -> B -> A -> App
    ```
* Good dependency graph (A, B, and C can be built in parallel):
    ```text
      /-> A  \
    D ->  B  -> App
      \-> C  /
    ```

## Turn off unused features

* Before:
    ```toml
    [dependencies]
    tokio = { version = "1", features = ["full"] } # build all of Tokio                .
    ```
* After:
    ```toml
    [dependencies]
    tokio = { version = "1", features = ["net", "io-util", "rt-multi-thread"] }
    ```

## Prefer pure-Rust dependencies

* crate cannot be build before `build.rs` is compiled and executed
    * crates using C-dependencies have to rely on `build.rs`
    * `build.rs` might trigger C/C++ compilation which in turn is often slow

* e.g.: `rustls` instead of `openssl`

## Use multi-module integration tests:

* Before (3 binaries)
```text
├── src/
│   └── ...
└── tests/
    ├── account-management.rs
    ├── billing.rs
    └── reporting.rs
```
* After (a single binary)
```text
├── src/
│   └── ...
└── tests/
    └── my-app-tests/
        ├── main.rs   # includes the rest as modules       .
        ├── account-management.rs
        ├── billing.rs
        └── reporting.rs
```
* examples, benchmarks too

## Other tips

* split your large package into a few smaller ones to improve build parallelization
* extract your binaries into separate packages
* remove unused dependencies

## Tools

* `cargo-chef` to speed up your docker builds
* `sccache` for caching intermediary build artifacts across multiple projects and developers
