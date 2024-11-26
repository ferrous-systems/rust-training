#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use nrf52840_dk_bsp::{hal, Board};

#[entry]
fn main() -> ! {
    let mut nrf52 = Board::take().unwrap();
    let mut timer = hal::Timer::new(nrf52.TIMER0);
    loop {
        writeln!(nrf52.cdc, "On!").unwrap();
        nrf52.leds.led_2.enable();
        timer.delay(1_000_000);
        writeln!(nrf52.cdc, "Off!").unwrap();
        nrf52.leds.led_2.disable();
        timer.delay(1_000_000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::udf();
    }
}
