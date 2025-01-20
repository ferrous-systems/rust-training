//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

extern crate alloc;

use core::{fmt::Write, ptr::addr_of_mut};
use embedded_alloc::Heap;
use qemu_aarch32v78r::uart;

#[global_allocator]
static HEAP: Heap = Heap::empty();

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-AN536
const PERIPHERAL_CLOCK: u32 = 25_000_000;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `lib.rs`.
#[no_mangle]
pub extern "C" fn kmain() {
    // Initialize the allocator BEFORE you use it
    {
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
        unsafe {
            let heap_start = addr_of_mut!(HEAP_MEM);
            HEAP.init(heap_start as usize, HEAP_SIZE);
        }
    }

    if let Err(e) = main() {
        panic!("main returned {:?}", e);
    }
}

/// The main function of our Rust application.
///
/// Called by [`kmain`].
fn main() -> Result<(), core::fmt::Error> {
    let mut uart0 = unsafe { uart::CmsdkUart::new(uart::UART0_ADDR) };
    uart0.init(115200, PERIPHERAL_CLOCK).unwrap();

    writeln!(uart0, "Hello, this is Rust!")?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            let msg = alloc::format!("{z:>8.2} ");
            write!(uart0, "{}", msg)?;
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
    const SYS_REPORTEXC: u32 = 0x18;
    // We assume it is already enabled
    let mut uart0 = unsafe { uart::CmsdkUart::new(uart::UART0_ADDR) };
    let _ = writeln!(uart0, "PANIC: {:?}", info);
    loop {
        // Exit, using semihosting
        unsafe {
            core::arch::asm!(
                "svc 0x123456",
                in("r0") SYS_REPORTEXC,
                in("r1") 0x20026
            )
        }
    }
}

// End of file
