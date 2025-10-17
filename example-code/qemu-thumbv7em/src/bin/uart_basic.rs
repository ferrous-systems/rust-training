//! Print to the UART on an MPS2-AN505

#![no_std]
#![no_main]

use defmt_semihosting as _;
use core::fmt::Write;

use qemu_thumbv7em::{uart, SYSTEM_CLOCK};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Running uart_basic - printing to all five UARTs");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let mut uarts = [
        uart::CmsdkUart::new(peripherals.uart0),
        uart::CmsdkUart::new(peripherals.uart1),
        uart::CmsdkUart::new(peripherals.uart2),
        uart::CmsdkUart::new(peripherals.uart3),
        uart::CmsdkUart::new(peripherals.uart4),
    ];

    for (idx, uart) in uarts.iter_mut().enumerate() {
        uart.check().unwrap();
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        _ = write!(uart, "Hello, UART{}!\r\n", idx);
    }

    // Some time for the telnet server to receive the data.
    cortex_m::asm::delay(500_000_000);

    semihosting::process::exit(0);
}

// End of file
