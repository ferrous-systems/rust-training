# WebAssembly

## What?

WebAssembly(WASM) enables running Rust (among others) in a sandbox environment, including the browser.

WebAssembly is supported as a _compile target_.

## High performance

WASM is built with speed in mind and executes almost as fast as native code.

## The WASM sandbox

In its initial state, WASM does only provide memory and execution, no functionality.

This can be added through the host system in various ways.

## Hello World

```wat []
(module
    ;; Import the required fd_write WASI function which will write the given io vectors to stdout
    ;; The function signature for fd_write is:
    ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
    (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    ;; Write 'hello world\n' to memory at an offset of 8 bytes
    ;; Note the trailing newline which is required for the text to appear
    (data (i32.const 8) "hello world\n")

    (func $main (export "_start")
        ;; Creating a new io vector within linear memory
        (i32.store (i32.const 0) (i32.const 8))  ;; iov.iov_base - This is a pointer to the start of the 'hello world\n' string
        (i32.store (i32.const 4) (i32.const 12))  ;; iov.iov_len - The length of the 'hello world\n' string

        (call $fd_write
            (i32.const 1) ;; file_descriptor - 1 for stdout
            (i32.const 0) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
            (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
            (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
        )
        drop ;; Discard the number of bytes written from the top of the stack
    )
)
```

## WASM targets in Rust

Rust ships 3 WASM targets:

* wasm32-unknown-emscripten (legacy)
  * ships with an implementation of libc for WASM
* wasm32-unknown-unknown (stable)
  * direct compilation to WASM, with no additional tooling
* wasm32-wasi (in development)
  * WASM with support for _interface types_, a structured way of adding capabilities

## Installation: `rustup` Target

`rustup` allows installing multiple compilation targets.

```console
rustup target install wasm32-unknown-unknown
rustup target install wasm32-wasi
```

## Installing a host runtime

```console
curl --proto '=https' --tlsv1.2 -sSf https://wasmtime.dev/install.sh | bash
```

* Currently need building from git: https://github.com/bytecodealliance/wasmtime

## Usage: Hello World!

```console
$ cargo new hello
    Created binary (application) `hello` package
$ cargo build --target wasm32-wasi
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
$ wasmtime target/wasm32-wasi/debug/hello.wasm
Hello, world!
```

## A Rust & WASM Tutorial

<https://ferrous-systems.github.io/wasm-training-2022/>
