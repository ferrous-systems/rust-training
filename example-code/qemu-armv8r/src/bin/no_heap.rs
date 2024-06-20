//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

use core::fmt::Write;
use qemu_demo::cmsdk_uart;

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-
const PERIPHERAL_CLOCK: u32 = 25_000_000;

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
    let mut uart0 = unsafe { cmsdk_uart::Uart::new_uart0() };
    uart0.enable(115200, PERIPHERAL_CLOCK);
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
    const SYS_REPORTEXC: u32 = 0x18;
    // We assume it is already enabled
    let mut uart0 = unsafe { cmsdk_uart::Uart::new_uart0() };
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

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start
.code 32
.align 0

_start:
    // Set stack pointer
    ldr r3, =stack_top
    mov sp, r3
    // Allow VFP coprocessor access
    mrc p15, 0, r0, c1, c0, 2
    orr r0, r0, #0xF00000
    mcr p15, 0, r0, c1, c0, 2
    // Enable VFP
    mov r0, #0x40000000
    vmsr fpexc, r0
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .

"#
);

// End of file
