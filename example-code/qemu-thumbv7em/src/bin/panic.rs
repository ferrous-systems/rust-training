//! An example program for QEMU's Armv7E-M Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use defmt_semihosting as _;
use qemu_thumbv7em as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("This is the panic example.");
    panic!("Oh no");
}

// End of file
