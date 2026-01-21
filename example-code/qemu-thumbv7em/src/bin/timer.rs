//! An example program for QEMU's Armv7E-M Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

use defmt_semihosting as _;
use embedded_hal::delay::DelayNs as _;

use qemu_thumbv7em::SYSTEM_CLOCK;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Timer example application");

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    let timer = qemu_thumbv7em::timer::Timer::new(peripherals.timer0);
    let mut delay_timer = qemu_thumbv7em::timer::DelayTimer::new(timer, SYSTEM_CLOCK);

    loop {
        defmt::info!("hello from the timer app");
        delay_timer.delay_ms(1000);
    }
}

// End of file
