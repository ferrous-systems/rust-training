//! An example async UART program for QEMU's Armv7E-M Virtual Machine
//!
//! Written by Robin Müller at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2026

#![no_std]
#![no_main]

use defmt_semihosting as _;
use qemu_thumbv7em::{uart, SYSTEM_CLOCK};
use rtic_monotonics::{fugit::ExtU32, systick_monotonic, Monotonic as _};

#[rtic::app(device = qemu_thumbv7em, dispatchers = [AudioI2S])]
mod app {
    use super::*;

    systick_monotonic!(Mono, 1000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        async_tx: uart::asynch::AsyncTx,
        async_tx_irq_ctx: uart::asynch::InterruptContext,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        Mono::start(cx.core.SYST, SYSTEM_CLOCK);
        defmt::info!("RTIC async UART example starting!");

        let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
        let mut uart = uart::CmsdkUart::new(peripherals.uart0);
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        uart.check().unwrap();
        let (basic_tx, _basic_rx) = uart.split();
        let (async_tx, async_tx_irq_ctx) = uart::asynch::AsyncTx::new(basic_tx).unwrap();
        tx_task::spawn().unwrap();
        defmt_task::spawn().unwrap();
        (
            Shared {},
            Local {
                async_tx,
                async_tx_irq_ctx,
            },
        )
    }

    /// Our idle loop - does nothing
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    /// Prints to UART in a loop
    #[task(local = [async_tx], priority = 1)]
    async fn tx_task(cx: tx_task::Context) -> ! {
        static STRINGS: [&str; 3] = ["Hello!\n", "Nice to see you.\n", "Have a nice day!\n"];

        loop {
            for (index, s) in STRINGS.iter().enumerate() {
                defmt::info!("Writing string {} asynchronously", index);
                // This is a non-blocking transfer! While it is on-going, the executor can run other
                // tasks. The hardware will transfer the passed buffer in the background using the
                // [tx_interrupt] task to make progress.
                cx.local.async_tx.write(s.as_bytes()).await;
                Mono::delay(500.millis()).await;
            }
        }
    }

    /// Prints to defmt in a loop
    #[task(priority = 1)]
    async fn defmt_task(_cx: defmt_task::Context) -> ! {
        loop {
            defmt::info!("I am defmt_task()");
            Mono::delay(100.millis()).await;
        }
    }

    /// This interrupt indicates that the async UART transmission can progress.
    #[task(binds = Uart0Tx, local = [async_tx_irq_ctx])]
    fn tx_interrupt(cx: tx_interrupt::Context) {
        // Safety: We're in the UART TX interrupt handler
        unsafe {
            cx.local.async_tx_irq_ctx.handle_irq();
        }
    }
}
