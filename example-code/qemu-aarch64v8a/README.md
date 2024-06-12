# Ferrocene for 64-bit Arm bare-metal Demo

This repository contains a small example application that can be built using the
[Ferrocene] toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene 24.05 is supported on *x86-64 Linux (glibc)*
(`x86_64-unknown-linux-gnu`) as the host platform, and *Armv8-A bare-metal*
(`aarch64-unknown-none`) as a cross-compilation target.

You must first install Ferrocene by executing `criticalup install` inside this
folder. This will require a valid CriticalUp token - please see the [CriticalUp
documentation](https://criticalup.ferrocene.dev).

## Demo contents

This demo is a simple application designed to run inside a QEMU virtual machine.

1. It prints "Hello, world!" to the first QEMU UART, which is typically
   connected to the console when you run QEMU.
2. It then causes a `panic!` which causes the custom panic handler to execute.
3. The the panic handler also prints to the same UART.
4. The panic handler exits QEMU using a semihosting operation that QEMU
   understands to mean "exit QEMU".

## Building and Running with `cargo`

Ferrocene compiles standard Rust source code, and so this project has also been
set up as a valid Rust project. The [`.cargo/config.toml`](./.cargo/config.toml)
file configures the default target as `aarch64-unknown-none` and sets up the
linker arguments to ensure that [`./linker.ld`](./linker.ld) is used as the
linker script.

Before the build, `cargo` will compile and execute `build.rs`, which will copy
the linker script to the `cargo` temporary output directory where the linker
will look for it.

The compiled outputs will go into `./target/aarch64-none-eabi/<profile>`, where
`<profile>` is `debug` or `release`. The binary is called `basic-rust`, because
that's the name given in the `Cargo.toml` file.

```console
$ criticalup run cargo build --release
    Finished release [optimized] target(s) in 0.00s
$ criticalup run cargo run --release
   Compiling basic-rust v0.1.0 (/Users/jonathan/work/basic-rust)
    Finished release [optimized] target(s) in 0.16s
     Running `qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel target/aarch64-unknown-none/release/basic-rust`
Hello, this is Rust!
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
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 40, col: 5 }, can_unwind: true, force_no_backtrace: false }
```

## Building and Running without `cargo`

Because the `cargo` binary shipped with Ferrocene is not qualified, you may
prefer to use your own build system, or call `rustc` directly.

This demo includes a [`build.sh`](./build.sh) shell script to build our binary
by calling `rustc` directly. This script will:

1. Find the location of the tools it needs
2. Call `criticalup run rustc` to compile `src/main.rs` into `<output>/basic-rust`
3. Generate `asm` and `map` files from the `<output>/basic-rust` using LLVM
   tools shipped with Ferrocene

The outputs will go into `./target/production` and the binary is called
`basic-rust`. You can choose any suitable directory, but avoid clashing with
anything you do using `cargo`.

```console
$ ./build.sh
Running rustc...
Generating asm...
Generating map...
$ qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -semihosting \
    -nographic \
    -kernel ./target/production/basic-rust
Hello, this is Rust!
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
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic), location: Location { file: "src/main.rs", line: 40, col: 5 }, can_unwind: true, force_no_backtrace: false }
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
