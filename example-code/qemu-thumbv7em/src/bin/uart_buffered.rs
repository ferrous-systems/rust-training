//! Print to the UART on an MPS2-AN505, using interrupts and a buffer.
//!
//! The UART output will be routed to log file logs/uart0.log

#![no_std]
#![no_main]

extern crate defmt_semihosting;

use core::fmt::Write as _;

use qemu_thumbv7em::{interrupt, interrupts::Interrupts, uart, uart::BufferedUart, SYSTEM_CLOCK};

/// Our UART buffer size
///
/// The [`heapless::spsc::Queue`] docs say that to get better performance we
/// should use a value that is a power of 2.
const QLEN: usize = 256;

/// A global UART we can write to
static UART0: BufferedUart<QLEN> = BufferedUart::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_irq - printing to global UART0");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    UART0
        .init(
            uart::CmsdkUart::new(peripherals.uart0),
            115200,
            SYSTEM_CLOCK,
        )
        .unwrap();

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Tx);
        cortex_m::interrupt::enable();
    }

    _ = write!(&UART0, "Hello, this is on a static UART0!\r\n");

    // these should all be queued (don't send more than `QLEN` bytes!)
    critical_section::with(|_| {
        _ = write!(&UART0, "Hello, this another string on a static UART0!\r\n");
    });
    // now they should transmit

    // Wait for the UART bytes to be send
    UART0.flush();

    semihosting::process::exit(0);
}

/// Called when UART0 has a TX interrupt
#[interrupt]
fn Uart0Tx() {
    UART0.tx_isr();
}

// End of file
