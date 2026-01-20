#![no_std]

// pull in defmt logger
use defmt_semihosting as _;
// pull in critical-section
use aarch32_cpu as _;

pub mod uart;

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board.
///
/// Probably right for an MPS3-AN536
pub const PERIPHERAL_CLOCK: u32 = 25_000_000;

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    match (info.message().as_str(), info.location()) {
        (Some(m), Some(l)) => {
            defmt::error!("PANIC at {=str}:{=u32}: {}", l.file(), l.line(), m);
        }
        (Some(m), None) => {
            defmt::error!("PANIC: {}", m);
        }
        (None, Some(l)) => {
            defmt::error!("PANIC at {=str}:{=u32}", l.file(), l.line());
        }
        (None, None) => {
            defmt::error!("PANIC!");
        }
    }
    semihosting::process::exit(1);
}

// End of file
