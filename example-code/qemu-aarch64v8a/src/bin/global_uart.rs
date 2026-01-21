//! An example program for QEMU's Armv8-A Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use core::{cell::RefCell, fmt::Write, ptr::NonNull};

use aarch64_rt::entry;
use arm_pl011_uart::{
    DataBits, LineConfig, PL011Registers, Parity, StopBits, Uart, UniqueMmioPointer,
};

use qemu_aarch64v8a as _;

const UART_ADDRESS: NonNull<PL011Registers> =
    NonNull::new(0x0900_0000 as *mut PL011Registers).unwrap();

struct GlobalUart {
    inner: critical_section::Mutex<RefCell<Option<Uart<'static>>>>,
}

impl GlobalUart {
    pub const fn new() -> GlobalUart {
        GlobalUart {
            inner: critical_section::Mutex::new(RefCell::new(None)),
        }
    }

    pub fn init(&self, uart: Uart<'static>) {
        critical_section::with(|cs| {
            let mut lock = self.inner.borrow_ref_mut(cs);
            lock.replace(uart);
        });
    }
}

impl core::fmt::Write for &GlobalUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        critical_section::with(|cs| {
            let mut lock = self.inner.borrow_ref_mut(cs);
            if let Some(uart) = lock.as_mut() {
                uart.write_str(s)
            } else {
                Ok(())
            }
        })
    }
}

static UART: GlobalUart = GlobalUart::new();

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    defmt::println!("This is the global_uart example.");

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
    UART.init(uart0);
    print_stuff().unwrap();
    semihosting::process::exit(0);
}

/// Prints some text to the global UART
fn print_stuff() -> Result<(), core::fmt::Error> {
    writeln!(&UART, "This text appears on the UART")?;
    Ok(())
}

// End of file
