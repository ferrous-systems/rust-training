//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use core::fmt::Write;
use qemu_aarch32v8r::{uart, PERIPHERAL_CLOCK};

/// The entry-point to the Rust application.
#[aarch32_rt::entry]
fn main() -> ! {
    defmt::println!("This is the no_heap example.");
    let mut uart0 = unsafe { uart::CmsdkUart::new_with_raw_addr(uart::UART0_ADDR) };
    uart0.init(115200, PERIPHERAL_CLOCK).unwrap();
    uart0.check().expect("Your UART is missing");
    let _ = writeln!(uart0, "This text appears on the UART");
    semihosting::process::exit(0);
}

// End of file
