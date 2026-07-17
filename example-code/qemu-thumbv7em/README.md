# Examples for QEMU emulating an Armv7E-M Machine

This repository contains a small example application that can be built using the
[Ferrocene] toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene is supported on [a number of host platforms]. We are using *Armv7E-M
bare-metal* (`thumbv7em-none-eabihf`) as a cross-compilation target.

[a number of host platforms]: https://public-docs.ferrocene.dev/main/user-manual/targets/index.html

You must first install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).

You should also run `criticalup link create` to set up `+ferrocene` as a valid
option for `cargo`. You may also want a `rust-toolchain.toml` file to set
`ferrocene` as the default toolchain for this directory. You can copy
`rust-toolchain.toml.ferrocene` as a starting point.

This demo will also build with standard Rust. You can use `rustup` to obtain the
`thumbv7em-none-eabihf` target.

## Demo contents

This demo provides a few simple applications, designed to run inside a QEMU
virtual machine that is emulating an Arm Cortex-M system.

There are seven binaries in `./src/bin`:

- `defmt` prints some demt logs at different levels
- `panic` shows the panic handling
- `rtic_empty` is a simple RTIC skeleton app
- `timer` sets up the SysTick timer
- `uart_mutex` sets up a UART as a global variable and prints to it
- `uart_echo` sets up a UART and echos any input received
- `uart_buffered` sets up an interrupt-drive UART using an in-memory buffer
- `with_heap` sets up a heap allocator and uses the `format!` macro to generate
  heap-allocated strings, which it then prints.

All binaries use defmt to print logging information.

## Target Hardware

The real-world Arm MPS2, MPS2+ and MPS3 boards have an FPGA on board. The CPU core and the peripherals that CPU has are therefore a function of which FPGA image you have loaded. Arm provide a bunch of FPGA images, named after the Arm Application Note they are described in.

Zephyr has good docs for the MPS2 at <https://docs.zephyrproject.org/latest/boards/arm/mps2/doc/mps2_an386.html>, including a photo of the real board. The Arm mbed website also has documentation for the MPS2, at <https://os.mbed.com/platforms/ARM-MPS2/>.

The MPS-AN386 is described in Arm [Application Note AN386]. This image is based on the Cortex-M System Design Kit. The hardware features:

* Cortex-M4 core
* Memory-mapped VGA frame-buffer
* 5x PL022 SPI interfaces
* 16MB PSRAM
* 4MB ZBTSRAM
* 16K Block RAM (QEMU doesn't emulate this)
* 4MB SRAM
* Standard CMSDK peripherals (5x UARTs, 4x Timers)

[Application Note AN386]: https://developer.arm.com/documentation/dai0386/latest/

## Building and Running with `cargo`

Ferrocene compiles standard Rust source code, and so this project has also
been set up as a valid Rust project.

- The [`.cargo/config.toml`](./.cargo/config.toml) file configures the default
  target as `thumbv7em-none-eabihf`.
- The [build script](./build.rs) sets up the linker arguments to ensure that the
  `cortex-m-rt` crate's `./link.x` is used as the linker script, along with our
  local [`memory.x`](./memory.x) definition of which memory region to use. It
  also copies the linker scripts to the target folder so the linker can find
  them.
- The compiled outputs will go into `./target/thumbv7em-none-eabihf/<profile>`,
  where `<profile>` is `debug` or `release`.

You will need to install [qemu-run](https://crates.io/crates/qemu-run), which
handles starting `qemu-system-arm` and decoding the defmt logs sent over
semihosting.

```console
$ cargo run
   Compiling qemu-thumbv7em v0.1.0 (/Users/jonathan/Documents/ferrous-systems/rust-training/example-code/qemu-thumbv7em)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.24s
     Running `qemu-run --machine mps2-an386 --cpu cortex-m4 --log-format oneline target/thumbv7em-none-eabihf/debug/defmt`
Hello, world!
[ERROR] This is an error log (defmt src/bin/defmt.rs:15)
[WARN ] This is a warn log (defmt src/bin/defmt.rs:16)
[INFO ] This is an info log (defmt src/bin/defmt.rs:17)
[ERROR] PANIC at src/bin/defmt.rs:23: Testing panic!() in fn main (qemu_thumbv7em src/lib.rs:74)
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
