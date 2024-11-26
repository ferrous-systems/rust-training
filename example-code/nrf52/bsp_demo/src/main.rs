//! A simple LED blinking demo for nRF52840-DK
//!
//! Demonstrates using a Board Support Package, defmt, and the probe-rs extension for VS Code

#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use nrf52840_dk_bsp::{hal, Board};

extern crate defmt_rtt;

#[entry]
fn main() -> ! {
    // We do some hardware set-up first
    defmt::info!("Starting up...");
    let mut board = Board::take().unwrap();
    let mut timer = hal::Timer::new(board.TIMER0);

    // Now go into an infinite loop
    loop {
        // We're logging over both defmt, and the J-Link's UART to USB Serial
        // Port interface.
        defmt::debug!("On!");
        writeln!(board.cdc, "On!").unwrap();

        // Control the LED
        board.leds.led_2.enable();

        // Wait for 1 second (this is a 1 MHz timer)
        timer.delay(1_000_000);

        // More logging
        defmt::debug!("Off!");
        writeln!(board.cdc, "Off!").unwrap();

        // Control the LED
        board.leds.led_2.disable();

        // Wait for 1 second again
        timer.delay(1_000_000);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("Panic! {:?}", defmt::Debug2Format(&info));
    cortex_m::asm::udf();
}
