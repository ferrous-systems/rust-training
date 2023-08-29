# Working With Nightly

## Why?

- Dependencies may require nightly
- Compile times and error messages are sometimes better (sometimes not)
- There are several features which are not yet stable
- Compiler plugins

## Using Nightly

Use `rustup` to override the version used in a specific directory.

```sh
cd /nightly_project
rustup override set nightly
```

## Features

Features are gated behind "Feature Flags" which are enabled project wide.

Some examples:

- `asm` which provides inline assembly support
- `no_std` which disables implict `extern crate std`
- `inclusive_range`, similar to the stable `exclusive_range`

## Enabling Features

To enable a feature, add the following line into `src/main.rs` (for executables), or `src/lib.rs` (for libraries):

```rust ignore
#![feature(asm, no_std)]
```

## Compiler Plugins

Compiler Plugins add additional capabilities to Rust. For example:

- (Previously) custom derive
- Linters
- Libraries like [`regex_macros`](https://github.com/rust-lang/regex#usage-regex-compiler-plugin)

## Enabling Compiler Plugins

To enable a compiler plugin add the following line into `src/main.rs` (for executables), or`src/lib.rs` (for libraries):

```rust ignore
#![plugin(some_plugin)]
```

## Warning

It is unknown, when and if ever compiler-plugins will be stabilised.

## Stable development on nightly

It is recommendable to use a nighly compiler close to the release version used.
