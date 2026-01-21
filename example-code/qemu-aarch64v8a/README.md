# Ferrocene for 64-bit Arm bare-metal Demo

This repository contains a small example application that can be built using the
[Ferrocene] toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene 24.08 is supported on *x86-64 Linux (glibc)*
(`x86_64-unknown-linux-gnu`), *x86-64 Windows* and *AArch64 macOS* as the host
platform. We are using *Armv8-A bare-metal* (`aarch64-unknown-none`) as a
cross-compilation target.

You must first install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).

You should also run `criticalup link create` to set up `+ferrocene` as a valid
option for `cargo`. You may also want a `rust-toolchain.toml` file to set
`ferrocene` as the default toolchain for this directory. You can copy
`rust-toolchain.toml.ferrocene` as a starting point.

## Demo contents

This demo provides a few simple applications, designed to run inside a QEMU
virtual machine that is emulating an Aarch64 Arm Cortex-A system.

There are five binaries in `./src/bin`:

* `defmt` prints some demt logs at different levels
* `global_uart` sets up a UART as a global variable and prints to it
* `panic` shows the panic handling
* `uart` prints to the first UART
* `with_heap` sets up a heap allocator and uses the `format!` macro to generate
  heap-allocated strings, which it then prints.

All binaries use defmt to print logging information.

## Building and Running with `cargo`

Ferrocene compiles standard Rust source code, and so this project has also
been set up as a valid Rust project. The
[`.cargo/config.toml`](./.cargo/config.toml) file configures the default
target as `aarch64-unknown-none`. The build script sets up the linker
arguments to ensure that the `aarch64-rt` crate's `./image.ld` is used as the
linker script, along with our local [`memory.ld`](./memory.ld) definition of
which memory region to use.

Before the build, `cargo` will compile and execute `build.rs`, which will copy
the linker script to the `cargo` temporary output directory where the linker
will look for it.

The compiled outputs will go into `./target/aarch64-none-eabi/<profile>`, where
`<profile>` is `debug` or `release`.

You will need to install [qemu-run](https://crates.io/crates/qemu-run), which
handles starting `qemu-system-aarch64` and decoding the defmt logs sent over
semihosting.

Most of the examples will run as-is, however if want to access the virtual UART
over telnet, you need to add the `-- --uart-telnet` option to the `cargo run`
invocation.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or
<http://opensource.org/licenses/MIT>) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
