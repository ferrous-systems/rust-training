//! An embassy example program for QEMU's Armv7E-M Virtual Machine
//!
//! Written by Robin Mueller at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2026

#![no_std]
#![no_main]

use defmt_semihosting as _;
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs as _;

use embassy_executor::{Spawner, main};
use qemu_thumbv7em::{SYSTEM_CLOCK, interrupt, interrupts::Interrupts};

#[main]
async fn main(_spawner: Spawner) -> ! {
    defmt::println!("Embassy example application");
    unsafe {
        cortex_m::interrupt::enable();
    }

    let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
    embassy_time::driver_cmsdk::init_cortex_m(
        SYSTEM_CLOCK,
        Interrupts::Timer0,
        peripherals.timer0.into(),
        Interrupts::Timer1,
        peripherals.timer1.into(),
    );

    loop {
        defmt::info!("Hello from Embassy");
        Delay.delay_ms(1000).await;
    }
}

#[interrupt]
fn Timer0() {
    // Safety: We only call this once here for timekeeping.
    unsafe { embassy_time::driver_cmsdk::on_interrupt_timekeeping() }
}

#[interrupt]
fn Timer1() {
    // Safety: We only call this once here for alarm handling.
    unsafe { embassy_time::driver_cmsdk::on_interrupt_alarm() }
}

// End of file
