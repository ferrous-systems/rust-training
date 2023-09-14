#![no_std]
#![no_main]

use nrf52840_dk_bsp::Board;
use cortex_m_rt::entry;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut nrf52 = Board::take().unwrap();
    loop {
        writeln!(nrf52.cdc, "On!").unwrap();
        nrf52.leds.led_2.enable();
        writeln!(nrf52.cdc, "Off!").unwrap();
        nrf52.leds.led_2.disable();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::udf();
    }
}

