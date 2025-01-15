//! Print to the UART on an MPS2-AN505, using interrupts and a buffer.

#![no_std]
#![no_main]

extern crate defmt_semihosting;

use core::fmt::Write as _;

use qemu_thumbv8m::{interrupts::Interrupts, uart::BufferedUart};

// Yes, these two must be imported with the same name
// this is a macro
use cortex_m_rt::interrupt;
// this is an enum that the macro uses
use qemu_thumbv8m::interrupts::Interrupts as interrupt;

/// Our system clock speed
const SYSTEM_CLOCK: u32 = 25_000_000;

/// A global UART we can write to
static UART0: BufferedUart<256> = BufferedUart::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    let cm = cortex_m::Peripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(cm.SYST, SYSTEM_CLOCK);

    defmt::info!("Running uart_irq - printing to global UART0");

    UART0
        .init(
            unsafe { qemu_thumbv8m::uart::CmsdkUart::new(0x5020_0000) },
            115200,
            SYSTEM_CLOCK,
        )
        .unwrap();

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupts::Uart0Tx);
        cortex_m::interrupt::enable();
    }

    _ = write!(&UART0, "Hello, this is on a static UART0!\r\n");

    // these should all be queued (don't send more than QLEN!)
    critical_section::with(|_| {
        _ = write!(&UART0, "Hello, this another string on a static UART0!\r\n");
    });
    // now they should transmit

    // Wait for the UART bytes to be send
    delay.delay_ms(2000);

    panic!("Got to end of fn main()!");
}

/// Called when UART0 has a TX interrupt
#[interrupt]
unsafe fn Uart0Tx() {
    unsafe {
        UART0.tx_isr();
    }
}

// End of file
