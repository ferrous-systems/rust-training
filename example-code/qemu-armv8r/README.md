# Ferrocene 24.05 on QEMU Arm Cortex-R52 Demo

This repository contains a small example application that can be built using the
[Ferrocene] 24.05 toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Demo contents

This demo is a simple application designed to run inside a QEMU 9 virtual machine.

1. It prints "Hello, world!" to the CMSDK UART0, which is typically
   connected to the console when you run QEMU.
2. It then causes a `panic!` which causes the custom panic handler to execute.
3. The the panic handler also prints to the same UART.
4. The panic handler exits QEMU using a semihosting operation that QEMU
   understands to mean "exit QEMU".

Once you have built the demo, and built QEMU 9 from source (it's not out yet), the QEMU command line is something like:

```console
$ qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/production/kernel.elf
Hello, this is Rust!
   1   2   3   4   5   6   7   8   9  10
   2   4   6   8  10  12  14  16  18  20
   3   6   9  12  15  18  21  24  27  30
   4   8  12  16  20  24  28  32  36  40
   5  10  15  20  25  30  35  40  45  50
   6  12  18  24  30  36  42  48  54  60
   7  14  21  28  35  42  49  56  63  70
   8  16  24  32  40  48  56  64  72  80
   9  18  27  36  45  54  63  72  81  90
  10  20  30  40  50  60  70  80  90 100
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 114, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

See [`qemu.sh`](./qemu.sh).

## Building with `cargo` and Rust

Ferrocene compiles standard Rust source code, and so this project has also been
set up as a valid Rust project. The [`.cargo/config.toml`](./.cargo/config.toml)
file configures the default target as `armv8r-none-eabihf`. It also sets up the
linker arguments to ensure that [`./linker.ld`](./linker.ld) is used as the
linker script. Note that this target is Tier 3 and so will only compile with
Nightly Rust.

Before the build, `cargo` will compile and execute `build.rs`, which will:

1. Generate the name of AS (the assembler) and AR (the archiver) by taking the
   current linker name and changing `gcc` for `as` and `ar` respectively.
2. Copy the linker script to the `cargo` temporary output directory where the
   linker will look for it.
3. Assemble `src/boot.S` as `<output>/boot.o`
4. Add the assembled `<output>/` file to `<output>/libboot.a`
5. Tell `cargo` to link against `libboot.a`

The compiled outputs will go into `./target/armv8r-none-eabihf/<profile>`, where
`<profile>` is `debug` or `release`. The binary is called `basic-rust`, because
that's the name given in the `Cargo.toml` file.

```console
$ cargo +nightly build --release
    Finished release [optimized] target(s) in 0.00s
$ arm-none-eabi-size target/armv8r-none-eabihf/release/basic-rust
   text    data     bss     dec     hex filename
  16680       0       0   16680    4128 target/armv8r-none-eabihf/release/basic-rust
$ cargo +nightly run --release
   Compiling basic-rust v0.1.0 (/Users/jonathan/work/basic-rust)
    Finished release [optimized] target(s) in 0.16s
     Running `qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/armv8r-none-eabihf/release/basic-rust`
Hello, this is Rust!
   1   2   3   4   5   6   7   8   9  10
   2   4   6   8  10  12  14  16  18  20
   3   6   9  12  15  18  21  24  27  30
   4   8  12  16  20  24  28  32  36  40
   5  10  15  20  25  30  35  40  45  50
   6  12  18  24  30  36  42  48  54  60
   7  14  21  28  35  42  49  56  63  70
   8  16  24  32  40  48  56  64  72  80
   9  18  27  36  45  54  63  72  81  90
  10  20  30  40  50  60  70  80  90 100
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 114, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

## Building with Ferrocene

Because the `cargo` binary shipped with Ferrocene is not qualified, you may
prefer to use your own build system, or call `rustc` directly.

This demo includes a [`build.sh`](./build.sh) shell script to build our binary
by calling `rustc` directly. This script will:

1. Assemble `src/boot.S` as `<output>/boot.o`
2. Add the assembled `<output>/` file to `<output>/libboot.a`
3. Call `rustc` to compile `src/main.rs` into `<output>/basic-rust`
4. Generate `asm` and `map` files from the `<output>/basic-rust`

If you don't have the supported linker `arm-none-eabi-gcc` and wish to
substitute it with an unsupported linker, you can set the C toolchain prefix
with:

```console
PREFIX=aarch64-elf ./build.sh"
```

The outputs will go into `./target/production` and the binary is called
`basic-rust`. You can choose any suitable directory, but avoid clashing with
anything you do using `cargo`.

```console
$ ./build.sh
Running as...
Running ar..
Running rustc...
Generating asm...
Generating map...
$ arm-none-eabi-size target/armv8r-none-eabihf/release/basic-rust
   text    data     bss     dec     hex filename
  16680       0       0   16680    4128 target/armv8r-none-eabihf/release/basic-rust
$ qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel target/production/kernel.elf
Hello, this is Rust!
   1   2   3   4   5   6   7   8   9  10
   2   4   6   8  10  12  14  16  18  20
   3   6   9  12  15  18  21  24  27  30
   4   8  12  16  20  24  28  32  36  40
   5  10  15  20  25  30  35  40  45  50
   6  12  18  24  30  36  42  48  54  60
   7  14  21  28  35  42  49  56  63  70
   8  16  24  32  40  48  56  64  72  80
   9  18  27  36  45  54  63  72  81  90
  10  20  30  40  50  60  70  80  90 100
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 114, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

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
