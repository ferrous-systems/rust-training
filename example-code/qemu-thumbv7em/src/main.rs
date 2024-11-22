#![no_std]
#![no_main]

extern crate defmt_semihosting;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    defmt::error!("This is an error log");
    defmt::warn!("This is a warn log");
    defmt::info!("This is an info log");
    defmt::debug!("This is a debug log");
    defmt::trace!("This is a trace log");
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("Hello, world!");
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    }
}
