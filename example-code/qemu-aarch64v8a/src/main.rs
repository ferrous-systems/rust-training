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

mod virt_uart;

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in [`boot.S`](./boot.S) and thus exported
/// as a C-compatible symbol.
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
    writeln!(uart0, "Hello, this is Rust!")?;
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

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start

// Assumes we are in EL1

_start:
    // Set stack pointer
    ldr x30, =stack_top
    mov sp, x30
    // Set FPEN bits [21:20] to 0b11 to prevent trapping.
    mov x0, #3 << 20
    msr cpacr_el1, x0   
    // Clear interrupt bit
    msr daifclr, #0x4   
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .

"#
);

// End of file
