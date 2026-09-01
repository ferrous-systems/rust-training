# Ferrocene for 64-bit Arm bare-metal Demo

This repository contains a small example application that can be built using the
[Ferrocene] toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene is supported on [a number of host platforms]. We are using *Armv8-A
bare-metal* (`aarch64-unknown-none`) as a cross-compilation target.

[a number of host platforms]: https://public-docs.ferrocene.dev/main/user-manual/targets/index.html

You must first install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).

You should also run `criticalup link create` to set up `+ferrocene` as a valid
option for `cargo`. You may also want a `rust-toolchain.toml` file to set
`ferrocene` as the default toolchain for this directory. You can copy
`rust-toolchain.toml.ferrocene` as a starting point.

This demo will also build with standard Rust. You can use `rustup` to obtain the
`aarch64-unknown-none` target.

## Demo contents

This demo provides a few simple applications, designed to run inside a QEMU
virtual machine that is emulating an AArch64 Arm Cortex-A system.

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
  target as `aarch64-unknown-none`.
- The [build script](./build.rs) sets up the linker arguments to ensure that the
  `aarch64-rt` crate's `./image.ld` is used as the linker script, along with our
  local [`memory.ld`](./memory.ld) definition of which memory region to use. It
  also copies the linker scripts to the target folder so the linker can find
  them.
- The compiled outputs will go into `./target/aarch64-unknown-none/<profile>`,
  where `<profile>` is `debug` or `release`.

You will need to install [qemu-run](https://crates.io/crates/qemu-run), which
handles starting `qemu-system-aarch64` and decoding the defmt logs sent over
semihosting.

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `qemu-run --machine virt --cpu cortex-a57 --aarch64 --log-format oneline target/aarch64-unknown-none/debug/defmt`
Hello, world!
[ERROR] This is an error log (defmt src/bin/defmt.rs:15)
[WARN ] This is a warn log (defmt src/bin/defmt.rs:16)
[INFO ] This is an info log (defmt src/bin/defmt.rs:17)
[ERROR] PANIC at src/bin/defmt.rs:23: Testing panic!() in fn main (qemu_aarch64v8a src/lib.rs:88)
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
