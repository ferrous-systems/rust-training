//! An example program for QEMU's Aarch64 Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024
//!
//! To

#![no_std]
#![no_main]

use core::fmt::Write;

use aarch64_rt::entry;
use qemu_aarch64v8a::{exception_level, virt_uart};

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    if let Err(e) = rust_main() {
        panic!("main returned {:?}", e);
    }
    semihosting::process::exit(0);
}

/// The main function of our Rust application.
///
/// Called by [`main`].
fn rust_main() -> Result<(), core::fmt::Error> {
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

// End of file
