//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use qemu_aarch32v8r as _;

/// The entry-point to the Rust application.
#[aarch32_rt::entry]
fn main() -> ! {
    defmt::println!("This is the panic example.");
    panic!("Oh no");
}

// End of file
