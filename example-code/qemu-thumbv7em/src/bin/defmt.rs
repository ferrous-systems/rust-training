#![no_std]
#![no_main]

use defmt_semihosting as _;
use qemu_thumbv7em as _;

#[cortex_m_rt::entry]
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
