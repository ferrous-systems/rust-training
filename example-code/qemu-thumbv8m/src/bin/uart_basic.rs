//! Print to the UART on an MPS2-AN505

#![no_std]
#![no_main]

extern crate defmt_semihosting;

use core::fmt::Write;

use qemu_thumbv8m::uart::CmsdkUart;

/// Our system clock speed
const SYSTEM_CLOCK: u32 = 25_000_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_basic - printing to all five UARTs");

    let mut uarts = [
        unsafe { CmsdkUart::new(0x5020_0000) },
        unsafe { CmsdkUart::new(0x5020_1000) },
        unsafe { CmsdkUart::new(0x5020_2000) },
        unsafe { CmsdkUart::new(0x5020_3000) },
        unsafe { CmsdkUart::new(0x5020_4000) },
    ];

    for (idx, uart) in uarts.iter_mut().enumerate() {
        uart.check().unwrap();
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        _ = write!(uart, "Hello, UART{}!\r\n", idx);
    }

    panic!("Got to end of fn main()!");
}

// End of file
