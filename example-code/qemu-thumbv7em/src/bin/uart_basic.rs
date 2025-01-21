//! Print to the UART on an MPS2-AN505

#![no_std]
#![no_main]

extern crate defmt_semihosting;

use core::fmt::Write;

use qemu_thumbv7em::uart;

/// Our system clock speed
const SYSTEM_CLOCK: u32 = 25_000_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_basic - printing to all five UARTs");

    let mut uarts = [
        unsafe { uart::CmsdkUart::new(uart::UART0_ADDR) },
        unsafe { uart::CmsdkUart::new(uart::UART1_ADDR) },
        unsafe { uart::CmsdkUart::new(uart::UART2_ADDR) },
        unsafe { uart::CmsdkUart::new(uart::UART3_ADDR) },
        unsafe { uart::CmsdkUart::new(uart::UART4_ADDR) },
    ];

    for (idx, uart) in uarts.iter_mut().enumerate() {
        uart.check().unwrap();
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        _ = write!(uart, "Hello, UART{}!\r\n", idx);
    }

    panic!("Got to end of fn main()!");
}

// End of file
