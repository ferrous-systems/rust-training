# Examples for QEMU emulating an Armv7E-M Machine

These examples are designed to run in QEMU emulating the MPS2-AN386 machine.

## Examples

### `defmt`

Demomstrates how to use `defmt` inside QEMU. It uses `defmt-semihosting` to output the `defmt` frames using Cortex-M semi-hosting interrupts. The script `qemu_run.sh` will start QEMU and pipe the semihosting output into `defmt-print` so that the `defmt` logs are decoded.

### `uart_basic`

Demonstrates use of stack-allocated CMSDK UART object from Rust. The UART writes are blocking.

### `uart_mutex`

Demonstrates use of globally-allocated CMSDK UART object from Rust. The UART writes are blocking and execute in a critical-section with interrupts disabled.

### `uart_buffered`

Demonstrates use of globally-allocated CMSDK UART object from Rust alongside a circular buffer. The UART writes are copied into the ring-buffer (blocking is space is exhausted) and the ring-buffer is emptied byte-by-byte under interrupt.

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

Ferrocene 24.05 is supported on *x86-64 Linux (glibc)*
(`x86_64-unknown-linux-gnu`) as the host platform, and *Armv8-A bare-metal*
(`aarch64-unknown-none`) as a cross-compilation target. To use Ferrocene with this project:

1. Install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).
2. Run `criticalup link create` to set up `+ferrocene` as a valid option for `cargo`.
3. Copy [`rust-toolchain.hide.toml`](./rust-toolchain.hide.toml) to `rust-toolchain.toml` to set the default toolchain to be `+ferrocene`.

Alternatively, you can skip steps 2 and 3, and execute `criticalup run cargo run`. However, if you have an editor open using Rust Analyzer it will continue to build the code in the background with the standard Rust Project toolchain, and this may cause conflicts.

## Running

QEMU has been configured to redirect the UART data to a telnet server on `localhost:4321` so you can see the UART output separate from any `defmt` output. If QEMU pauses on start-up then it is waiting for you to run `telnet localhost 4321` to open a connection and start receiving the UART data.

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
