//! Hello world, using write_fmt

use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    stdout.write_fmt(format_args!("Hello, world!\n")).unwrap();
}
