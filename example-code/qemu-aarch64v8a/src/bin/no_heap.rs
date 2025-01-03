//! An example program for QEMU's Aarch64 Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025
//!
//! To

#![no_std]
#![no_main]

use core::fmt::Write;
use qemu_aarch64v8a::{exception_level, virt_uart};

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `lib.rs`.
#[no_mangle]
pub extern "C" fn kmain() {
    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    let mut uart0 = unsafe { virt_uart::Uart::new_uart0() };
    writeln!(uart0, "Hello, this is Rust @ {:?}", exception_level())?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            write!(uart0, "{z:>8.2} ")?;
        }
        writeln!(uart0)?;
    }
    panic!("I am a panic");
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    const SYS_REPORTEXC: u64 = 0x18;
    let mut c = unsafe { virt_uart::Uart::new_uart0() };
    let _ = writeln!(c, "PANIC: {:?}", info);
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "hlt 0xf000",
                in("x0") SYS_REPORTEXC
            )
        }
    }
}

// End of file
