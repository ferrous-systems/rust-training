#![no_std]
#![no_main]
use defmt_semihosting as _;
use qemu_thumbv7em as _;
use qemu_thumbv7em::{uart, SYSTEM_CLOCK};
use rtic_monotonics::fugit::ExtU32;
use rtic_monotonics::systick_monotonic;
use rtic_monotonics::Monotonic as _;

#[rtic::app(device = qemu_thumbv7em, dispatchers = [AudioI2S])]
mod app {
    use super::*;

    systick_monotonic!(Mono, 1000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        tx: uart::asynch::TxAsynch,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        Mono::start(cx.core.SYST, SYSTEM_CLOCK);
        defmt::println!("RTIC sample app!");

        let peripherals = qemu_thumbv7em::Peripherals::take().unwrap();
        let mut uart = uart::Uart::new(peripherals.uart0);
        uart.init(115200, SYSTEM_CLOCK).unwrap();
        uart.check().unwrap();
        let (tx, _rx) = uart.split();
        let tx_async = uart::asynch::TxAsynch::new(tx, 0).unwrap();
        tx_task::spawn().unwrap();
        (Shared {}, Local { tx: tx_async })
    }

    #[task(local = [tx])]
    async fn tx_task(cx: tx_task::Context) -> ! {
        static STRINGS: [&str; 3] = ["Hello!\n", "Nice to see you.\n", "Have a nice day!\n"];

        let mut index = 0;
        loop {
            defmt::info!("Writing string {} asynchronously", index);
            // This is a non-blocking transfer! While it is on-going, the executor can run other
            // tasks. The hardware will transfer the passed buffer in the background using the
            // [tx_interrupt] task to make progress.
            //
            // We could even create the futures ourselves to start multiple concurrent UART
            // transfers.
            cx.local.tx.write(STRINGS[index].as_bytes()).await;
            index = (index + 1) % STRINGS.len();
            Mono::delay(1000.millis()).await;
        }
    }

    /// This interrupt is important so the asynchronous UART transmission can progress.
    #[task(binds = Uart0Tx)]
    fn tx_interrupt(_cx: tx_interrupt::Context) {
        // It would be nice if we could hide this inside the interrupt handler, but the interrupt
        // handler lives inside a generic library which does not know the base addresses of the
        // UARTS..
        let mut tx = unsafe { uart::Tx::steal(uart::UART0_ADDR) };
        uart::asynch::on_interrupt_tx(&mut tx, 0);
    }
}
