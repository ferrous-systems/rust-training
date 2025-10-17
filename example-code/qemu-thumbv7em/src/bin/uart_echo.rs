//! UART echo application using an CMSDK UART.
//!
//! You can connect to this app via telnet and you should see all sent characters echoed back.

#![no_std]
#![no_main]

use defmt_semihosting as _;
use embedded_io::Write as _;

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
    defmt::info!("-- QEMU UART Echo example --");

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
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Rx);
        cortex_m::interrupt::enable();
    }

    let mut rx_buffer: [u8; QLEN] = [0; QLEN];

    loop {
        let read_bytes = UART0.read(&mut rx_buffer);
        if read_bytes > 0 {
            (&UART0).write_all(&rx_buffer[0..read_bytes]).unwrap();
        }
        // Go to sleep until data is received.
        cortex_m::asm::wfi();
    }
}

/// Called when UART0 has a TX interrupt
#[interrupt]
fn Uart0Tx() {
    UART0.tx_isr();
}

/// Called when UART0 has a RX interrupt
#[interrupt]
fn Uart0Rx() {
    UART0.rx_isr();
}

// End of file
