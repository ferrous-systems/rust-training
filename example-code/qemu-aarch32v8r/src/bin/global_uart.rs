//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use core::fmt::Write;
use qemu_aarch32v8r::{
    uart::{CmsdkUart, MutexUart, UART0_ADDR},
    PERIPHERAL_CLOCK,
};

/// Our global UART object
static UART: MutexUart = MutexUart::empty();

/// The entry-point to the Rust application.
#[aarch32_rt::entry]
fn main() -> ! {
    defmt::println!("This is the global_uart example.");
    UART.init(
        unsafe { CmsdkUart::new_with_raw_addr(UART0_ADDR) },
        115200,
        PERIPHERAL_CLOCK,
    )
    .unwrap();
    print_stuff().unwrap();
    semihosting::process::exit(0);
}

/// Prints some text to the global UART
fn print_stuff() -> Result<(), core::fmt::Error> {
    writeln!(&UART, "This text appears on the UART")?;
    Ok(())
}

// End of file
