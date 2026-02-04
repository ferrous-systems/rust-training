//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use core::{fmt::Write, ptr::NonNull};

use aarch64_rt::entry;
use arm_pl011_uart::{
    DataBits, LineConfig, PL011Registers, Parity, StopBits, Uart, UniqueMmioPointer,
};

use qemu_aarch64v8a as _;

const UART_ADDRESS: NonNull<PL011Registers> =
    NonNull::new(0x0900_0000 as *mut PL011Registers).unwrap();

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    defmt::println!("This is the no_heap example.");

    // SAFETY: `UART_ADDRESS` is the base address of a PL011 UART register block. It remains valid for
    // the lifetime of the application and nothing else references this address range.
    let uart_pointer = unsafe { UniqueMmioPointer::new(UART_ADDRESS) };

    // Create driver instance
    let mut uart0 = Uart::new(uart_pointer);

    // Configure and enable UART
    let line_config = LineConfig {
        data_bits: DataBits::Bits8,
        parity: Parity::None,
        stop_bits: StopBits::One,
    };
    uart0.enable(line_config, 115_200, 16_000_000).unwrap();
    let _ = writeln!(uart0, "This text appears on the UART");
    semihosting::process::exit(0);
}

// End of file
