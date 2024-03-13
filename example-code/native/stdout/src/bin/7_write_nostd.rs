//! Hello world, as a no-std binary
//!
//! Requires `RUSTFLAGS="-C panic=abort -lc"` (and nightly Rust for Linux targets).
//!
//! Runs on anything with a POSIX compatible `write` function.

#![no_std]
#![no_main]
#![cfg_attr(target_os = "linux", feature(lang_items))]
#![allow(internal_features)]

extern "C" {
    // As per unistd.h
    // ssize_t write(int fildes, const void *buf, size_t nbyte);
    fn write(fd: i32, buffer: *const core::ffi::c_void, length: usize) -> isize;
}

/// 0 for "Standard In", 1 for "Standard Out", 2 for "Standard Error"
const STDOUT: i32 = 1;

#[no_mangle]
extern "C" fn main() {
    let message = "Hello, world!\n";
    let result = unsafe { write(STDOUT, message.as_ptr().cast(), message.len()) };
    if result < 0 {
        panic!("Write returned {result}");
    }
}

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
#[lang = "eh_personality"]
fn rustc_eh_personality() {}
