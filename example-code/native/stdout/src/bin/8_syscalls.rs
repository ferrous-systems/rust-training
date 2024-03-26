//! Hello world, as a no-std binary
//!
//! Requires `RUSTFLAGS="-C panic=abort -lc"` (and nightly Rust for Linux targets).
//!
//! Runs on Aarch64 macOS, Aarch64 Linux and x86-64 Linux.

#![no_std]
#![no_main]
#![cfg_attr(target_os = "linux", feature(lang_items))]
#![allow(internal_features)]

use core::ffi::c_int;

/// 0 for "Standard In", 1 for "Standard Out", 2 for "Standard Error"
const STDOUT: i32 = 1;

#[no_mangle]
extern "C" fn main() {
    write(STDOUT, "Hello, world!\n".as_bytes()).unwrap();
}

pub fn write(fd: c_int, buf: &[u8]) -> Result<usize, isize> {
    let buf_pointer = buf.as_ptr();
    let buf_size = buf.len();
    let retval: isize;

    #[cfg(all(target_arch = "x86_64", any(target_os = "linux", target_os = "none")))]
    unsafe {
        core::arch::asm!(
            "SYSCALL",
            in("rax") 1,
            inout("rdi") fd as usize => retval,
            in("rsi") buf_pointer,
            in("rdx") buf_size,
            options(nostack),
        );
    }

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    unsafe {
        core::arch::asm!(
            "SVC 0x80",
            in("x16") 4,
            inout("x0") fd as usize => retval,
            in("x1") buf_pointer,
            in("x2") buf_size,
            options(nostack),
        );
    }

    // tested working
    #[cfg(all(target_arch = "aarch64", any(target_os = "linux", target_os = "none")))]
    unsafe {
        core::arch::asm!(
            "SVC 0x80",
            in("x8") 64,
            inout("x0") fd as usize => retval,
            in("x1") buf_pointer,
            in("x2") buf_size,
            options(nostack),
        );
    }

    if retval >= 0 {
        Ok(retval as usize)
    } else {
        Err(retval)
    }
}

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
#[lang = "eh_personality"]
fn rustc_eh_personality() {}
