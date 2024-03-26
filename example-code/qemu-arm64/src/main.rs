//! An example program for QEMU's Aarch64 Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2023
//!
//! To

#![no_std]
#![no_main]

use core::fmt::Write;

/// Represents an emulated QEMU UART.
struct Uart {
    base: *mut u32,
}

impl Uart {
    /// Create a handle to the first UART
    pub fn uart0() -> Uart {
        const UART0_ADDR: usize = 0x0000_0000_0900_0000;
        Uart {
            base: UART0_ADDR as *mut u32,
        }
    }

    /// Write one byte to the QEMU virtual UART.
    ///
    /// We don't have to check for FIFO space as the emulated FIFO never runs
    /// out of space.
    pub fn putchar(&mut self, byte: u8) {
        unsafe {
            self.base.write_volatile(u32::from(byte));
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.putchar(b);
        }
        Ok(())
    }
}

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
    let mut c = Uart::uart0();
    writeln!(c, "Hello, this is Rust!")?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = x * y;
            write!(c, "{z:4}")?;
        }
        writeln!(c)?;
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
    let mut c = Uart::uart0();
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
