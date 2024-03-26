# Printing to stdout

Printing to the *Standard Out* stream is so simple, it's the fundamental element of the most trivial computer program - the *Hello, world* program.

This package contains several versions of *Hello, world*, each using less standard library functionality than the last.

## 1 - Println

Code: [`1_println.rs`](./src/bin/1_println.rs)

This is the classic version, using *libstd*'s `println!` macro. This macro converts its arguments to a formatter object, and emits code which will:

1. Grab a lock on standard out
2. Write each piece of the formatted output to standard out, as a UTF-8 encoded byte slice
3. Write a newline character
4. Release the lock

## 2 - Writeln

Code: [`2_writeln.rs`](./src/bin/2_writeln.rs)

As before, but we grab a handled to standard out and write to it using the `writeln!` macro. This requires us to include the `std::io::Write` trait.

## 3 - Writefmt

Code: [`3_writefmt.rs`](./src/bin/3_writefmt.rs)

As before, but now we create the formatter object manually, and pass it to the `write_fmt` method on the standard output handle.

## 4 - Writestr

Code: [`4_writestr.rs`](./src/bin/4_writestr.rs)

As before, but now we dispense with the formatting machinery entirely, and send raw bytes to `write_str`.

## 5 - C `puts`

Code: [`5_puts.rs`](./src/bin/5_puts.rs)

This time we ignore the standard library and import the C Library Function `puts`, as defined in the C header `stdio.h`. This function takes a null-terminated string, and writes it to standard output, followed by a newline character.

## 6 - POSIX `write`

Code: [`6_write.rs`](./src/bin/6_write.rs)

Now we use the POSIX function `write`, as defined in the C header `unistd.h`. This takes a pointer and a length, so the string doesn't have to be null-terminated.

## 7 - POSIX `write`, but `no-std`

Code: [`7_write_nostd.rs`](./src/bin/7_write_nostd.rs)

This application is like number 6, but now we turn off the standard library entirely. This means we have to:

1. Use `!#[no_main]` to tell `rustc` that we don't have a `fn main` that it can recognise, and that's OK.
2. Mark our `fn main()` as having C linkage, and an unmangled symbol
3. Implement a panic handler
4. Implement the `eh_personality` lang item (which requires us to use nightly Rust)
   * We could avoid this by using a target *designed* for `no_std` use, like `aarch64-unknown-none` but that makes CI testing more difficult

## 8 - Raw syscalls

Code: [`8_syscalls.rs`](./src/bin/8_syscalls.rs)

Now we skip the C library entirely, and emit a *system call* (or *syscall*) using a software interrupt. We have to use raw assembly to pack our arguments into the correct registers, before triggering the software interrupt with either a `svc` or `int` instruction (depending on the platform).
