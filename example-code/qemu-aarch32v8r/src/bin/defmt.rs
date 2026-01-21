#![no_std]
#![no_main]

use qemu_aarch32v8r as _;

/// The entry-point to the Rust application.
#[aarch32_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    defmt::error!("This is an error log");
    defmt::warn!("This is a warn log");
    defmt::info!("This is an info log");
    defmt::debug!("This is a debug log");
    defmt::trace!("This is a trace log");
    // unsafe {
    //     core::arch::asm!("udf 0");
    // }
    panic!("Testing panic!() in fn main");
}

// End of file
