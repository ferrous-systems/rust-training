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
virtual machine that is emulating an Aarch64 Arm Cortex-A system. Both demos:

1. Print "Hello, world!" to the first QEMU UART, which is typically
   connected to the console when you run QEMU.
2. Print some floating point numbers in a grid (the 1 though 10 times tables).
3. Causes a `panic!` which causes the custom panic handler to execute.
4. The the panic handler prints the panic message using semihosting.
5. The panic handler exits QEMU using a semihosting operation that QEMU
   understands to mean "exit QEMU".

There are three binaries in `./src/bin`:

* `no_heap` runs with no heap
* `with_heap` sets up a heap allocator and uses the `format!` macro to generate
  heap-allocated strings, which it then prints.
* `global_uart` which sets up the UART as a global static object, with a Mutex

All binaries should produce the same output.

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

```console
$ criticalup run cargo build --release
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel target/aarch64-unknown-none/release/global_uart`
Hello, this is Rust @ EL1
    1.00     2.00     3.00     4.00     5.00     6.00     7.00     8.00     9.00    10.00
    2.00     4.00     6.00     8.00    10.00    12.00    14.00    16.00    18.00    20.00
    3.00     6.00     9.00    12.00    15.00    18.00    21.00    24.00    27.00    30.00
    4.00     8.00    12.00    16.00    20.00    24.00    28.00    32.00    36.00    40.00
    5.00    10.00    15.00    20.00    25.00    30.00    35.00    40.00    45.00    50.00
    6.00    12.00    18.00    24.00    30.00    36.00    42.00    48.00    54.00    60.00
    7.00    14.00    21.00    28.00    35.00    42.00    49.00    56.00    63.00    70.00
    8.00    16.00    24.00    32.00    40.00    48.00    56.00    64.00    72.00    80.00
    9.00    18.00    27.00    36.00    45.00    54.00    63.00    72.00    81.00    90.00
   10.00    20.00    30.00    40.00    50.00    60.00    70.00    80.00    90.00   100.00
PANIC: PanicInfo { message: I am a panic, location: Location { file: "src/bin/global_uart.rs", line: 76, column: 5 }, can_unwind: true, force_no_backtrace: false }
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
