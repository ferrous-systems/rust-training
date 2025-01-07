//! An example program for QEMU's Aarch64 Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

extern crate alloc;

use core::{fmt::Write, ptr::addr_of_mut};
use embedded_alloc::Heap;
use qemu_aarch64v8a::{exception_level, virt_uart};

#[global_allocator]
static HEAP: Heap = Heap::empty();

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
    let mut uart0 = unsafe { virt_uart::Uart::new_uart0() };
    writeln!(uart0, "Hello, this is Rust @ {:?}", exception_level())?;
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
