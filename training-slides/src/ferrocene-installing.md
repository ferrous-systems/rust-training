# Installing Ferrocene

## What's in the box?

* `rustc` - a compiler (★)
* `lld` - the LLVM linker (★)
* `cargo`/`rustdoc`/`rustfmt`/`clippy` - our usual friends
* `llvm-tools` - objcopy, size, etc
* `rust-analyzer` - for IDE integration
* `rust-src` - libstd source code
* `rust-std-xxx` - precompiled standard libraries (☆)
* `ferrocene-self-test` - checks your installation
* `ferrocene-docs-xxx` - documentation

★: qualified tool ☆: certification in progress

## Installation

* Ferrocene runs on x86-64 Linux, only (for now)
* Ferrocene 23.06 shipped as tarballs
* Ferrocene now has `criticalup`
* Either way, get `rustc` in your `PATH`

## Read the Manual

* `/opt/ferrocene-23.06/share/doc/ferrocene/html/user-manual`
* <https://public-docs.ferrocene.dev/main/user-manual/>

## Targets

* Supported targets installed as per the rest of the toolchain
* [Currently]:
  * `x86_64-unknown-linux-gnu`
  * `aarch64-unknown-none`
  * `thumbv7em-none-eabi`/`thumbv7em-none-eabihf` (☆)
  * `wasm32-unknown-unknown` (☆)

☆: experimental

[Currently]: https://public-docs.ferrocene.dev/main/user-manual/targets/index.html
