# Imports and Modules

## Namespaces

* A namespace is simply a way to distinguish two things that have the same name.
* It provides a *scope* to the identifiers within it.

## Rust supports namespacing in two ways:

1. Crates for re-usable software libraries
2. Modules for breaking up your crates

## Crates

* A crate is the unit of Rust software suitable for shipping.
* Yes, it's a deliberate pun.
* The Rust Standard Library is a crate.

## Kinds of Crate

* Library crate (has a `src/lib.rs`)
* Binary crates (has e.g. a `src/bin.rs`)

Note:

A package can have multiple binary crates, e.g. `src/bin/foo.rs` and `src/bin/bar.rs` and you use `cargo run --bin foo` to pick between them.

You can also put the binary crates in `src/examples`, where they should act as examples of how to use your library.

A package can even contain both a library and some binaries. But never more than one library, and it can never contain nothing.

## Libraries

A library cannot be 'executed'. It can only be included into another crate.

## Binaries

A binary crate can be executed on the command-line.

```console
$ cargo new --bin hello
     Created binary (application) `hello` package
$ cd hello 
$ cargo run
   Compiling hello v0.1.0 (/private/tmp/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 2.46s
     Running `target/debug/hello`
Hello, world!
```

## There's no build file

* Have you noticed that `Cargo.toml` says nothing about which files to compile?
* Cargo starts with `lib.rs` for a library or the relevant `bin.rs` for a binary
* It then finds all the *modules*

## Modules

* A module is block of source code within a crate
* It qualfies the names of everything in it
* It has a parent module (or it is the crate root)
* It can have child modules
* The crate is therefore a *tree*

## Standard Library

We've been using modules from the Rust Standard Library...

```rust []
use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = fs::File::create("hello.txt")?;
    f.write(b"hello")?;
    Ok(())
}
```

Note:

* The [`std::fs` module](https://doc.rust-lang.org/std/fs/index.html)
* The [`std::io` module](https://doc.rust-lang.org/std/io/index.html)
* The [`std::io::prelude` module](https://doc.rust-lang.org/std/io/prelude/index.html)

Prelude modules, like `std::io::prelude`, usually contain important traits and you usually want to import all of it with a `*` wildcard.

## What kind of import?

Choosing whether to import the parent module, or each of the types contained within, is something of an art form.

```rust []
use std::fs;
use std::collections::VecDeque;
use std::io::prelude::*;
```

## Standard Library

There's also a more compact syntax for imports.

```rust []
use std::{fs, io::prelude::*};

fn main() -> std::io::Result<()> {
    let mut f = fs::File::create("hello.txt")?;
    f.write(b"hello")?;
    Ok(())
}
```

## Comments

* The Rust Standard Library docs come from `cargo doc` on the libstd source code
* You should add `/// Hello` comments to every public item
* Ideally you would add `/// Hello` comments to *every* item

```rust []
/// Parses the input string as a 32-bit unsigned integer.
/// Will panic if the input is not a valid integer.
pub fn parse(input: &str) -> u32 {
    todo!()
}
```

## Doc Comments

You can even embed code-snippets in your comments.

```rust []
/// Parses the input string as a 32-bit unsigned integer.
/// Will panic if the input is not a valid integer.
///
/// ```
/// let x = parse("123");
/// assert_eq(x, 123);
/// ```
pub fn parse(input: &str) -> u32 {
    todo!()
}
```

Note:

For example, <https://doc.rust-lang.org/std/fs/struct.File.html#examples>.
