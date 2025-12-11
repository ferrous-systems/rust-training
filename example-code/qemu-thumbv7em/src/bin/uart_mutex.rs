//! Print to the UART on an MPS2-AN505, using a static driver

#![no_std]
#![no_main]

use core::fmt::Write as _;
use defmt_semihosting as _;

use qemu_thumbv7em::{uart, SYSTEM_CLOCK};

/// A global UART we can write to
static UART0: uart::MutexUart = uart::MutexUart::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_mutex - printing to global UART0");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let uart_handle = uart::Uart::new(peripherals.uart0);
    UART0.init(uart_handle, 115200, SYSTEM_CLOCK).unwrap();

    _ = write!(&UART0, "Hello, this is on a static UART0!\r\n");

    // Some time for the telnet server to receive the data.
    cortex_m::asm::delay(500_000_000);

    semihosting::process::exit(0);
}

// End of file
