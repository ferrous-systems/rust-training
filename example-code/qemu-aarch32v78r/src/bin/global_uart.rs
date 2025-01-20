//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

use core::fmt::Write;
use qemu_aarch32v78r::uart::{CmsdkUart, MutexUart, UART0_ADDR};

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-AN536
const PERIPHERAL_CLOCK: u32 = 25_000_000;

static UART: MutexUart = MutexUart::empty();

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
    UART.init(
        unsafe { CmsdkUart::new(UART0_ADDR) },
        115200,
        PERIPHERAL_CLOCK,
    )
    .unwrap();

    print_stuff()?;
    panic!("I am a panic");
}

fn print_stuff() -> Result<(), core::fmt::Error> {
    writeln!(&UART, "Hello, this is Rust!")?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            write!(&UART, "{z:>8.2} ")?;
        }
        writeln!(&UART)?;
    }
    Ok(())
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    const SYS_REPORTEXC: u32 = 0x18;
    let _ = writeln!(&UART, "PANIC: {:?}", info);
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
