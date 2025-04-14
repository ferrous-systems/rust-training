#![no_std]

use cortex_m_semihosting::debug as cmsd;

pub mod interrupts;
pub mod uart;

/// A panic handler which logs to defmt and then does a semihosting exit.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        defmt::error!(
            "Panic! {} ({=str}:{=u32})",
            defmt::Debug2Format(&info.message()),
            location.file(),
            location.line()
        );
    } else {
        defmt::error!("Panic! {}", defmt::Debug2Format(&info.message()));
    }
    loop {
        cmsd::exit(cmsd::EXIT_FAILURE);
    }
}

/// A Hard Fault handler which logs to defmt and then does a semihosting exit.
#[cortex_m_rt::exception(trampoline = true)]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    defmt::error!("HardFault: {}", defmt::Debug2Format(frame));
    loop {
        cmsd::exit(cmsd::EXIT_FAILURE);
    }
}

// End of file
