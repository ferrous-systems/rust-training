# Examples for QEMU emulating an Armv7E-M Machine

These examples are designed to run in QEMU emulating the MPS2-AN386 machine.

## Examples

There are seven binaries in `./src/bin`:

* `defmt` prints some demt logs at different levels
* `panic` shows the panic handling
* `rtic_empty` is a simple RTIC skeleton app
* `timer` sets up the SysTick timer
* `uart_mutex` sets up a UART as a global variable and prints to it
* `uart_echo` sets up a UART and echos any input received
* `uart_buffered` sets up an interrupt-drive UART using an in-memory buffer
* `with_heap` sets up a heap allocator and uses the `format!` macro to generate
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

## Ferrocene

This project has been setup to build with the standard Rust Project toolchain. You can also build it with Ferrocene.

To use Ferrocene with this project:

1. Install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).
2. Run `criticalup link create` to set up `+ferrocene` as a valid option for `cargo`.
3. Copy [`rust-toolchain.hide.toml`](./rust-toolchain.hide.toml) to `rust-toolchain.toml` to set the default toolchain to be `+ferrocene`.

Alternatively, you can skip steps 2 and 3, and execute `criticalup run cargo run`. However, if you have an editor open using Rust Analyzer it will continue to build the code in the background with the standard Rust Project toolchain, and this may cause conflicts.

## Running

You will need to install [qemu-run](https://crates.io/crates/qemu-run), which
handles starting `qemu-system-aarch64` and decoding the defmt logs sent over
semihosting.

Most of the examples will run as-is, however if want to access the virtual UART
over telnet, you need to add the `-- --uart-telnet` option to the `cargo run`
invocation.

```console
$ cargo run --bin uart_echo -- --uart-telnet
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.03s
     Running `qemu-run --machine mps2-an386 --cpu cortex-m4 --log-format=oneline target/thumbv7em-none-eabihf/debug/uart_echo --uart-telnet`
[INFO ] Told QEMU to start telnet server on localhost:4321. Connect to interact with UART0. (<crate> qemu-run/src/main.rs:157) (HOST)
qemu-system-arm: -chardev socket,id=sock0,server=on,telnet=on,port=4321,host=localhost: info: QEMU waiting for connection on: disconnected:telnet:::1:4321,server=on
[INFO ] Running uart_echo - echoing via to global buffered UART0 (uart_echo src/bin/uart_echo.rs:32)
[INFO ] Application read 1 bytes ([54]). Echoing back. (uart_echo src/bin/uart_echo.rs:76)
[INFO ] Application read 1 bytes ([48]). Echoing back. (uart_echo src/bin/uart_echo.rs:76)
[INFO ] Application read 1 bytes ([69]). Echoing back. (uart_echo src/bin/uart_echo.rs:76)
[INFO ] Application read 1 bytes ([73]). Echoing back. (uart_echo src/bin/uart_echo.rs:76)
[INFO ] Application read 1 bytes ([20]). Echoing back. (uart_echo src/bin/uart_echo.rs:76)
```

You can access the telnet server with `telnet localhost:4321` or similar.

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
