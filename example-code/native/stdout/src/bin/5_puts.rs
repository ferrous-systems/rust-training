//! Hello world, using libc

extern "C" {
    fn puts(string: *const std::ffi::c_char) -> i32;
}

fn main() {
    let message = c"Hello, world!";
    let result = unsafe { puts(message.as_ptr()) };
    if result < 0 {
        panic!("puts returned {result}");
    }
}
