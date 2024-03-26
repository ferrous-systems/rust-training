//! Hello world, using writeln!

use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    writeln!(stdout, "Hello, world!").unwrap();
}
