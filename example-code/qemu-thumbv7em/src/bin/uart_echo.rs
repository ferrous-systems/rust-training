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

/// How much we process every go around the loop
const MAX_READ_LEN: usize = 16;

/// A global UART we can write to
static UART0: BufferedUart<QLEN> = BufferedUart::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("-- QEMU UART Echo example --");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    UART0
        .init(
            uart::Uart::new(peripherals.uart0),
            115200,
            SYSTEM_CLOCK,
        )
        .unwrap();

    unsafe {
        // mark receive as higher prio than transmit
        cp.NVIC.set_priority(Interrupts::Uart0Rx, 0);
        cp.NVIC.set_priority(Interrupts::Uart0Tx, 255);
        // enable those interrupts
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Tx);
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Rx);
        cortex_m::interrupt::enable();
    }

    let mut rx_buffer: [u8; MAX_READ_LEN] = [0; MAX_READ_LEN];

    loop {
        // We do *not* want an interrupt to occur in-between checking our RX
        // buffer and going to sleep, because we might end up sleeping with RX
        // data in the buffer in an interrupt arrives between the check and
        // the sleep!
        //
        // So, let's check the RX buffer with interrupts disabled.
        let read_bytes = critical_section::with(|_| {
            let read_bytes = UART0.read(&mut rx_buffer);
            if read_bytes == 0 {
                // WFI will wake on interrupt, even though interrupts are disabled.
                cortex_m::asm::wfi();
            }
            read_bytes
        });
        // Now we either have data, which we can process, or we woke up, in
        // which case we do nothing and deal with the data next-time around
        // the loop.
        if read_bytes > 0 {
            let valid_data = &rx_buffer[0..read_bytes];
            defmt::info!(
                "Application read {} bytes ({=[u8]:02x}). Echoing back.",
                read_bytes,
                valid_data
            );
            (&UART0).write_all(valid_data).unwrap();
        } else {
            defmt::trace!("CPU woke up - checking for data...");
        }
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
