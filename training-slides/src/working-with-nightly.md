# Working With Nightly

## Why?

* There are many features which are not yet stable
  * language
  * library
  * cargo, rustdoc, etc
* Dependencies may require nightly
* You can't wait for the train
* Compile times and error messages are sometimes better (sometimes not)

## Using Nightly

Use `rustup` to override the version used in a specific directory.

```sh
cd /nightly_project
rustup override set nightly-2024-02-01
```

## Pinning a version

You can also store the information in your repo:

```console
$ cat rust-toolchain.toml
[toolchain]
channel = "nightly-2024-02-01"
```

## Langauge features

[Language features](https://doc.rust-lang.org/unstable-book/language-features.html) are parts of Rust we haven't quite agreed on yet, but there's an implementation there to be tested. Each one has a tracking issue.

Some examples:

* [`riscv_target_feature`](https://github.com/rust-lang/rust/issues/44839) - adds `target_feature` on RISC-V
* [`naked_functions`](https://github.com/rust-lang/rust/issues/32408) - functions with no prologue or epilogue
* [`never_type`](https://github.com/rust-lang/rust/issues/35121) - supporting `!` as a type

## RPIT, RPITIT, AFIT, and more

* Return Position Impl Trait
* Return Position Impl Trait in Trait
* Async Function in Trait
* [A handy guide](https://santiagopastorino.com/2022/10/20/what-rpits-rpitits-and-afits-and-their-relationship/)

Note:

* RPIT would be something like `fn fetch() -> impl Debug`.
* RPITIT is a trait method that has impl trait in the return position.
* AFIT is a trait method like `async fn do_stuff()`

## Enabling Language Features

To enable, add the feature attribute to your top-level module:

```rust ignore
#![feature(riscv_target_feature)]
```

## Compiler features

Unstable compiler flags start with `-Z`.

See them all with:

```sh
rustc +nightly -Z help
```

## Library features

Some parts of the Standard Library are 'unstable' and only available on nightly.

Nothing special required to opt-in, just nightly Rust.

You can see them in the docs, like [`slice::new_zeroed_slice()`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.new_zeroed_slice)

## Cargo features

You can specify unstable cargo features in your `.cargo/config.toml`:

```toml
[unstable]
mtime-on-use = true
```

## The Standard Library

* The Standard Library is written in Rust
* It must therefore be compiled
* But stable `rustc` cannot compile the Standard Library
* => `rustup` gives you a pre-compiled Standard Library for your target

Note:

Why does it require nightly? Because it's full of unstable library APIs, and makes use of unstable compiler features.

So how do they build libstd during a toolchain release? With a secret magic flag that makes stable Rust look like nightly Rust for the purposes of building the standard library. You should not use this flag yourself.

## Compiling the Standard Library

* If you have nightly rust, you can compile it from source yourself
* `rustup component add rust-src`
* `rustc -Z build-std=core,alloc ...`, or give [cargo](https://github.com/rust-lang/wg-cargo-std-aware) this config:

```toml
[unstable]
build-std = ["core", "alloc"]
```

## Availability

* Nightly doesn't always succesfully build
* rustup can go back in time and find a working build
* [rustup-component-history](https://rust-lang.github.io/rustup-components-history/) can help

## The books

* [rustc](https://doc.rust-lang.org/rustc/command-line-arguments.html#-z-set-unstable-options)
* [cargo](https://doc.rust-lang.org/cargo/reference/unstable.html)
* [rustdoc](https://doc.rust-lang.org/rustdoc/unstable-features.html)
* [The Unstable Book](https://doc.rust-lang.org/unstable-book/index.html)
