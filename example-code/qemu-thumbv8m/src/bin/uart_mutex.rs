//! Print to the UART on an MPS2-AN505, using a static driver

#![no_std]
#![no_main]

extern crate defmt_semihosting;

use core::fmt::Write as _;

use qemu_thumbv8m::uart::MutexUart;

/// Our system clock speed
const SYSTEM_CLOCK: u32 = 25_000_000;

/// A global UART we can write to
static UART0: MutexUart = MutexUart::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_mutex - printing to global UART0");

    UART0
        .init(
            unsafe { qemu_thumbv8m::uart::CmsdkUart::new(0x5020_0000) },
            115200,
            SYSTEM_CLOCK,
        )
        .unwrap();

    _ = write!(&UART0, "Hello, this is on a static UART0!\r\n");

    panic!("Got to end of fn main()!");
}

// End of file
