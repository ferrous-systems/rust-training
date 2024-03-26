//! Hello world, using write_all

use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    stdout.write_all("Hello, world!\n".as_bytes()).unwrap();
}
