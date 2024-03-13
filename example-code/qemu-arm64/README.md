# Ferrocene 23.06 on QEMU Demo

This repository contains a small example application that can be built using the
[Ferrocene] 23.06 toolchain from [Ferrous Systems].

[Ferrocene]: https://ferrocene.dev
[Ferrous Systems]: https://ferrous-systems.com

## Supported Platforms

Ferrocene 23.06 is supported on the following host platforms:

* Ubuntu 18.04 AMD64, with GCC 7.5 (`x86_64-unknown-linux-gnu`)

Using that host, it supports the following target platforms:

* Ubuntu 18.04 AMD64, with GCC 7.5 (`x86_64-unknown-linux-gnu`)
  * With `libstd` support for Linux
  * So that Ferrocene can compile and execute build scripts and proc-macros
* ARM ARMv8-A and ARMv9-A Aarch64 systems (`aarch64-unknown-none`)
  * `gcc-aarch64-linux-gnu` version 7.5 is the supported linker
  * There is no `libstd`, and therefore there is no Operating System support.
    Users are exected to use the Rust FFI mechanism to call their chosen
    Operating System's C APIs.

This demo therefore builds on Ubuntu 18.04 AMD64, and creates binaries suitable
for running on `qemu-system-aarch64`. This demo has not been certified as
meeting any particular standard or process and is offered merely as an example.

This set of hosts and targets is expected to change in future releases of
Ferrocene.

## Limitations of Ferrocene 23.06

Ferrocene 23.06 is the first release of the Ferrocene toolchain. As such it has
some limitations, which this demo accomodates:

* There is no installer - you have to manually unpack the supplied `.tar.xz`
  files. This demo unpacks them into `/opt/ferrocene-23.06`.
* Only GCC 7.5 is supported as the target linker
* Only `rustc` has been submitted for qualification, and not `cargo`. You should
  therefore arrange to call `rustc` directly in your production builds.

## Demo contents

This demo is a simple application designed to run inside a QEMU virtual machine.

1. It prints "Hello, world!" to the first QEMU UART, which is typically
   connected to the console when you run QEMU.
2. It then causes a `panic!` which causes the custom panic handler to execute.
3. The the panic handler also prints to the same UART.
4. The panic handler exits QEMU using a semihosting operation that QEMU
   understands to mean "exit QEMU".

Once you have built the demo, the QEMU command line is something like:

```console
$ qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel target/production/kernel.elf
Hello, world!
PANIC: PanicInfo {
    payload: Any { .. },
    message: Some(
        Let's try a panic?,
    ),
    location: Location {
        file: "src/main.rs",
        line: 28,
        col: 5,
    },
    can_unwind: true,
}
```

See [`qemu.sh`](./qemu.sh).

## Building with `cargo` and Rust

Ferrocene compiles standard Rust source code, and so this project has also been
set up as a valid Rust project. The [`.cargo/config.toml`](./.cargo/config.toml)
file configures the default target as `aarch64-unknown-none` and selects
`aarch64-linux-gnu-gcc` as the linker. It also sets up the linker arguments to
ensure that [`./linker.ld`](./linker.ld) is used as the linker script.

Before the build, `cargo` will compile and execute `build.rs`, which will:

1. Generate the name of AS (the assembler) and AR (the archiver) by taking the
   current linker name and changing `gcc` for `as` and `ar` respectively.
2. Copy the linker script to the `cargo` temporary output directory where the
   linker will look for it.
3. Assemble `src/boot.S` as `<output>/boot.o`
4. Add the assembled `<output>/` file to `<output>/libboot.a`
5. Tell `cargo` to link against `libboot.a`

If you don't have the supported linker `aarch64-linux-gnu-gcc` and wish to
substitute it with an unsupported linker, you can run something like:

```console
cargo build --release --config "target.aarch64-unknown-none.linker=\"aarch64-elf-gcc\""
```

The compiled outputs will go into `./target/aarch64-none-eabi/<profile>`, where
`<profile>` is `debug` or `release`. The binary is called `basic-rust`, because
that's the name given in the `Cargo.toml` file.

```console
$ cargo build --release
    Finished release [optimized] target(s) in 0.00s
$ aarch64-elf-size target/aarch64-unknown-none/release/basic-rust
   text    data     bss     dec     hex filename
  23146       0       0   23146    5a6a target/aarch64-unknown-none/release/basic-rust
$ cargo run --release
   Compiling basic-rust v0.1.0 (/Users/jonathan/Downloads/ferrocene-23.06/work/basic-rust)
    Finished release [optimized] target(s) in 0.16s
     Running `qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel target/aarch64-unknown-none/release/basic-rust`
Hello, world!
PANIC: PanicInfo {
    payload: Any { .. },
    message: Some(
        Let's try a panic?,
    ),
    location: Location {
        file: "src/main.rs",
        line: 28,
        col: 5,
    },
    can_unwind: true,
}
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

If you don't have the supported linker `aarch64-linux-gnu-gcc` and wish to
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
$ aarch64-linux-gnu-size ./target/production/basic-rust
   text    data     bss     dec     hex filename
  24127       0       0   24127    5e3f ./target/production/basic-rust
$ qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -semihosting \
    -nographic \
    -kernel ./target/production/basic-rust
Hello, world!
PANIC: PanicInfo { payload: Any { .. }, message: Some(I am a panic),
location: Location { file: "src/main.rs", line: 72, col: 5 }, can_unwind: true }
```

## Creating a Docker Container

If you don't have an Ubuntu 18.04 machine, but you do have Docker available, you
can use `docker/Dockerfile` to create environment.

You should change to the location where you downloaded the Ferrocene install
tarballs, and run:

```console
docker build . -t ferrocene-23.06 -f ./path/to/docker/Dockerfile
```

Adjust the final path to point to where the
[`./docker/Dockerfile`](./docker/Dockerfile) is located relative to your
download area.

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
