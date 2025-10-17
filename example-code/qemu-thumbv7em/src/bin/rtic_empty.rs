#![no_std]
#![no_main]
use defmt_semihosting as _;
use qemu_thumbv7em as _;
use rtic_monotonics::systick_monotonic;

#[rtic::app(device = qemu_thumbv7em, dispatchers = [AudioI2S])]
mod app {
    use qemu_thumbv7em::SYSTEM_CLOCK;
    use rtic_monotonics::fugit::ExtU32;
    use rtic_monotonics::Monotonic as _;

    use super::*;

    systick_monotonic!(Mono, 1000);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        Mono::start(cx.core.SYST, SYSTEM_CLOCK);
        defmt::println!("RTIC sample app!");
        hello::spawn().unwrap();
        (Shared {}, Local {})
    }

    #[task]
    async fn hello(_cx: hello::Context) -> ! {
        loop {
            defmt::info!("Hello from RTIC");
            Mono::delay(1000.millis()).await;
        }
    }
}
