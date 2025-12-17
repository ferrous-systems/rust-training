#![no_std]
#![no_main]

use aarch64_rt::entry;

use qemu_aarch64v8a as _;

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
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
