//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

#![no_std]
#![no_main]

use core::{cell::RefCell, fmt::Write};

use aarch64_rt::entry;
use critical_section::Mutex;
use qemu_aarch64v8a::{exception_level, virt_uart::Uart};

static UART: GlobalUart = GlobalUart::new();

struct GlobalUart {
    inner: Mutex<RefCell<Option<Uart<0x0900_0000>>>>,
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
    fn store(&self, uart: Uart<0x0900_0000>) -> Option<Uart<0x0900_0000>> {
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

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    if let Err(e) = rust_main() {
        panic!("main returned {:?}", e);
    }
    semihosting::process::exit(0);
}

/// The main function of our Rust application.
///
/// Called by [`main`].
fn rust_main() -> Result<(), core::fmt::Error> {
    let uart0 = unsafe { Uart::new_uart0() };
    UART.store(uart0);

    print_stuff()?;
    panic!("I am a panic");
}

/// Print some things to the global UART
fn print_stuff() -> Result<(), core::fmt::Error> {
    writeln!(&UART, "Hello, this is Rust @ {:?}", exception_level())?;
    for x in 1..=10 {
        for y in 1..=10 {
            let z = f64::from(x) * f64::from(y);
            write!(&UART, "{z:>8.2} ")?;
        }
        writeln!(&UART)?;
    }
    Ok(())
}

// End of file
