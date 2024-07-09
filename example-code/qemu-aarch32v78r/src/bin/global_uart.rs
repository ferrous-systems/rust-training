//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

use core::{cell::RefCell, fmt::Write};
use critical_section::Mutex;
use qemu_aarch32v78r::cmsdk_uart::Uart;

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-AN536
const PERIPHERAL_CLOCK: u32 = 25_000_000;

static UART: GlobalUart = GlobalUart::new();

struct GlobalUart {
    inner: Mutex<RefCell<Option<Uart<0xe7c0_0000>>>>,
}

impl GlobalUart {
    /// Create a new, empty, global UART wrapper
    const fn new() -> GlobalUart {
        GlobalUart {
            inner: Mutex::new(RefCell::new(None)),
        }
    }

    /// Store a new UART at run-time
    ///
    /// Gives you back the old one, if any.
    fn store(&self, uart: Uart<0xe7c0_0000>) -> Option<Uart<0xe7c0_0000>> {
        critical_section::with(|cs| {
            let mut uart_ref = self.inner.borrow_ref_mut(cs);
            uart_ref.replace(uart)
        })
    }
}

// Note that we're implementing for `&GlobalUart`, so we can write to a shared
// reference instead of requiring an exclusive-mutable reference.
impl core::fmt::Write for &GlobalUart {
    /// Write the string to the inner UART, with a lock held
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        critical_section::with(|cs| {
            let mut maybe_uart = self.inner.borrow_ref_mut(cs);
            let Some(uart) = maybe_uart.as_mut() else {
                return Err(core::fmt::Error);
            };
            uart.write_str(s)
        })
    }
}

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
    let mut uart0 = unsafe { Uart::new_uart0() };
    uart0.enable(115200, PERIPHERAL_CLOCK);
    UART.store(uart0);

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
