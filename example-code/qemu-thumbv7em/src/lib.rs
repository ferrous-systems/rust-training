#![no_std]

use core::sync::atomic::AtomicBool;

// Yes, these two must be imported with the same name. It is also required for RTIC.
//
// this is a macro
pub use cortex_m_rt::interrupt;
// this is an enum that the macro uses
pub use interrupts::Interrupts as interrupt;

use defmt_semihosting as _;

pub mod interrupts;
pub mod timer;
pub mod uart;

/// Number available in the NVIC for configuring priority. Required for RTIC as well.
pub const NVIC_PRIO_BITS: u8 = 3;

/// Our system clock speed
pub const SYSTEM_CLOCK: u32 = 25_000_000;

static PERIPHS_TAKEN: AtomicBool = AtomicBool::new(false);

/// Singleton containing the CMSDK device peripherals.
///
/// RTIC expects the singleton with this name.
pub struct Peripherals {
    pub uart0: uart::MmioRegisters<'static>,
    pub uart1: uart::MmioRegisters<'static>,
    pub uart2: uart::MmioRegisters<'static>,
    pub uart3: uart::MmioRegisters<'static>,
    pub uart4: uart::MmioRegisters<'static>,
    pub timer0: timer::registers::MmioRegisters<'static>,
    pub timer1: timer::registers::MmioRegisters<'static>,
}

impl Peripherals {
    /// Take the peripherals singleton
    pub fn take() -> Option<Self> {
        if PERIPHS_TAKEN.swap(true, core::sync::atomic::Ordering::Relaxed) {
            return None;
        }
        // Safety: We just checked the atomic flag to ensure this only runs once.
        Some(unsafe { Self::steal() })
    }

    /// # Safety
    ///
    /// This steals the peripherals singleton and circumvents ownership rules for the device
    /// peripherals.
    pub unsafe fn steal() -> Self {
        Self {
            uart0: unsafe { uart::Registers::new_mmio_at(uart::UART0_ADDR) },
            uart1: unsafe { uart::Registers::new_mmio_at(uart::UART1_ADDR) },
            uart2: unsafe { uart::Registers::new_mmio_at(uart::UART2_ADDR) },
            uart3: unsafe { uart::Registers::new_mmio_at(uart::UART3_ADDR) },
            uart4: unsafe { uart::Registers::new_mmio_at(uart::UART4_ADDR) },
            timer0: unsafe { timer::registers::Registers::new_mmio_at(timer::TIMER_0_ADDR) },
            timer1: unsafe { timer::registers::Registers::new_mmio_at(timer::TIMER_1_ADDR) },
        }
    }
}

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
    semihosting::process::exit(1);
}

/// A Hard Fault handler which logs to defmt and then does a semihosting exit.
#[cortex_m_rt::exception(trampoline = true)]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    defmt::error!("HardFault: {}", defmt::Debug2Format(frame));
    semihosting::process::exit(1);
}

// End of file
