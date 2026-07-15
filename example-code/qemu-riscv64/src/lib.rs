//! Utility functions

#![no_std]

/// A console that uses SBI debug output
struct Console {}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

/// Write formatted data to the SBI debug output
pub fn print_args(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Console {};
    writer.write_fmt(args).unwrap();
}

/// Write an unformatted string to the SBI debug output
///
/// You should use this through the `print!` and `println!` macros.
#[doc(hidden)]
pub fn print(s: &str) {
    for c in s.bytes() {
        _ = sbi::debug_console::write_byte(c);
    }
}

/// Write to the SBI debug output
#[macro_export]
macro_rules! print {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::print_args(format_args!($fmt $(,$($arg)+)?))
    }
}

/// Write to the SBI debug output, with a newline
#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {{
        $crate::print!($fmt $(,$($arg)+)?);
        $crate::print("\n");
    }};
    () => {
        $crate::print("\n");
    }
}

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC! {:?}", info);
    loop {
        _ = sbi::system_reset::system_reset(
            sbi::system_reset::ResetType::Shutdown,
            sbi::system_reset::ResetReason::SystemFailure,
        );
    }
}
