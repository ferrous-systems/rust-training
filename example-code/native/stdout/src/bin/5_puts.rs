//! Hello world, using libc

extern "C" {
    fn puts(string: *const std::ffi::c_char) -> i32;
}

fn main() {
    let message = std::ffi::CStr::from_bytes_with_nul("Hello, world!\0".as_bytes()).unwrap();
    let result = unsafe { puts(message.as_ptr()) };
    if result < 0 {
        panic!("puts returned {result}");
    }
}
