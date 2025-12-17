#![no_std]

// pull in defmt logger
use defmt_semihosting as _;

pub mod uart;

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
