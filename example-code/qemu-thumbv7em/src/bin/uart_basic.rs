//! Print to the UARTs of the MPS2-AN386 UARTs
//!
//! The UART output will be routed to log files inside the logs folder.

#![no_std]
#![no_main]

use core::fmt::Write;
use defmt_semihosting as _;
use embedded_hal::delay::DelayNs;

use qemu_thumbv7em::{timer, uart, SYSTEM_CLOCK};

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
    let mut delay_timer =
        timer::DelayTimer::new(timer::Timer::new(peripherals.timer0), SYSTEM_CLOCK);

    for (idx, uart) in uarts.iter_mut().enumerate() {
        uart.check().unwrap();
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        _ = write!(uart, "Hello, UART{}!\r\n", idx);
    }

    // Some time for the telnet server to receive the data.
    delay_timer.delay_ms(100);

    semihosting::process::exit(0);
}

// End of file
