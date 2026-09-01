# Ferrocene for 32-bit Arm Cortex-R bare-metal Demo

This repository contains a small example application that can be built using the
[Ferrocene] toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene is supported on [a number of host platforms]. We are using *Armv8-R
AArch32 bare-metal* (`armv8r-none-eabihf`) as a cross-compilation target.

[a number of host platforms]: https://public-docs.ferrocene.dev/main/user-manual/targets/index.html

You must first install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).

You should also run `criticalup link create` to set up `+ferrocene` as a valid
option for `cargo`. You may also want a `rust-toolchain.toml` file to set
`ferrocene` as the default toolchain for this directory. You can copy
`rust-toolchain.toml.ferrocene` as a starting point.

This demo will also build with standard Rust. You can use `rustup` to obtain the
`armv8r-none-eabihf` target.

## Demo contents

This demo provides a few simple applications, designed to run inside a QEMU
virtual machine that is emulating an AArch32 Arm Cortex-R system.

There are five binaries in `./src/bin`:

- `defmt` prints some demt logs at different levels
- `global_uart` sets up a UART as a global variable and prints to it
- `panic` shows the panic handling
- `uart` prints to the first UART
- `with_heap` sets up a heap allocator and uses the `format!` macro to generate
  heap-allocated strings, which it then prints.

All binaries use defmt to print logging information.

## Building and Running with `cargo`

Ferrocene compiles standard Rust source code, and so this project has also
been set up as a valid Rust project.

- The [`.cargo/config.toml`](./.cargo/config.toml) file configures the default
  target as `armv8r-none-eabihf`.
- The [build script](./build.rs) sets up the linker arguments to ensure that the
  `aarch32-rt` crate's `./link.x` is used as the linker script, along with our
  local [`memory.x`](./memory.x) definition of which memory region to use. It
  also copies the linker scripts to the target folder so the linker can find
  them.
- The compiled outputs will go into `./target/armv8r-none-eabihf/<profile>`,
  where `<profile>` is `debug` or `release`.

You will need to install [qemu-run](https://crates.io/crates/qemu-run), which
handles starting `qemu-system-arm` and decoding the defmt logs sent over
semihosting.

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `qemu-run --machine mps3-an536 --cpu cortex-r52 --log-format oneline target/armv8r-none-eabihf/debug/defmt`
Hello, world!
[ERROR] This is an error log (defmt src/bin/defmt.rs:10)
[WARN ] This is a warn log (defmt src/bin/defmt.rs:11)
[INFO ] This is an info log (defmt src/bin/defmt.rs:12)
[ERROR] PANIC at src/bin/defmt.rs:18: Testing panic!() in fn main (qemu_aarch32v8r src/lib.rs:23)
```

Most of the examples will run as-is, however if want to access the virtual UART
over telnet, you need to add the `-- --uart-telnet` option to the `cargo run`
invocation.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
