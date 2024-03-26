//! Hello world, using a POSIX libc

extern "C" {
    fn write(fd: i32, buffer: *const u8, length: usize) -> i32;
}

/// 0 for "Standard In", 1 for "Standard Out", 2 for "Standard Error"
const STDOUT: i32 = 1;

fn main() {
    let message = "Hello, world!\n";
    let result = unsafe { write(STDOUT, message.as_ptr(), message.len()) };
    if result < 0 {
        panic!("Write returned {result}");
    }
}
