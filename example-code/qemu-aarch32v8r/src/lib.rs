#![no_std]

// pull in the start-up code
use cortex_r_rt as _;

// pull in defmt logger
use defmt_semihosting as _;

pub mod uart;

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {:?}", defmt::Debug2Format(info));
    semihosting::process::exit(1);
}

// End of file
