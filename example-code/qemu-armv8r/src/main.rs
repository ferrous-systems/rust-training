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

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-
const PERIPHERAL_CLOCK: u32 = 25_000_000;

/// A driver for CMSDK Uart
struct Uart<const ADDR: usize>();

impl<const ADDR: usize> Uart<ADDR> {
    const STATUS_TX_FULL: u32 = 1 << 0;

    /// Turn on TX and RX
    fn enable(&mut self, baudrate: u32, system_clock: u32) {
        let divider = system_clock / baudrate;
        self.set_bauddiv(divider);
        self.set_control(0b0000_0011);
    }

    /// Write a byte (blocking if there's no space)
    fn write(&mut self, byte: u8) {
        // Check the Buffer Full bit
        while (self.get_status() & Self::STATUS_TX_FULL) != 0 {}
        self.set_data(byte as u32);
    }

    /// Write the data register
    fn set_data(&mut self, data: u32) {
        let ptr = ADDR as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }

    /// Read the status register
    fn get_status(&self) -> u32 {
        let ptr = (ADDR + 4) as *mut u32;
        unsafe { ptr.read_volatile() }
    }

    /// Set the control register
    fn set_control(&mut self, data: u32) {
        let ptr = (ADDR + 8) as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }

    /// Set the baud rate divider register
    fn set_bauddiv(&mut self, data: u32) {
        let ptr = (ADDR + 16) as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }
}

impl<const N: usize> core::fmt::Write for Uart<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write(b);
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
    let mut uart0: Uart<0xe7c00000> = Uart();
    uart0.enable(115200, PERIPHERAL_CLOCK);
    writeln!(uart0, "Hello, this is Rust!")?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = x * y;
            write!(uart0, "{z:4}")?;
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
    let mut uart0: Uart<0xe7c00000> = Uart();
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
