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
* Binary Crates and Library Crates

## There's no build file

* Have you noticed that `Cargo.toml` says nothing about which files to compile?
* Cargo starts with `lib.rs` for a library or the relevant `main.rs` for a binary
* It then finds all the *modules*

## Modules

* A module is block of source code within a crate
* It qualifies the names of everything in it
* It has a parent module (or it is the crate root)
* It can have child modules
* The crate is therefore a *tree*

## Standard Library

We've been using modules from the Rust Standard Library...

```rust
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

## In-line modules

You can declare a module in-line:

```rust
mod animals {
    pub struct Cat { name: String }

    impl Cat {
        pub fn new(name: &str) -> Cat {
            Cat { name: name.to_owned() }
        }
    }
}

fn main() {
    let c = animals::Cat::new("Mittens");
    // let c = animals::Cat { name: "Mittens".to_string() };
}
```

## Modules in a file

You can also put modules in their own file on disk.

This will load from either `./animals/mod.rs` or `./animals.rs`:

```rust ignore
mod animals;

fn main() {
    let c = animals::Cat::new("Mittens");
    // let c = animals::Cat { name: "Mittens".to_string() };
}
```

## Modules can be nested...

```console
~/probe-run $ tree src
src
├── backtrace
│   ├── mod.rs
│   ├── pp.rs
│   ├── symbolicate.rs
│   └── unwind.rs
├── canary.rs
├── cli.rs
├── cortexm.rs
├── dep
│   ├── cratesio.rs
│   ├── mod.rs
│   ├── rust_repo.rs
│   ├── rust_std
│   │   └── toolchain.rs
│   ├── rust_std.rs
│   └── rustc.rs
├── elf.rs
├── main.rs
├── probe.rs
├── registers.rs
├── stacked.rs
└── target_info.rs
```

Note:

The choice about `foo.rs` vs `foo/mod.rs` often depends on whether `mod foo`
itself has any child modules.

The example is from the Knurling tool [`probe-run`](https://github.com/knurling-rs/probe-run).

## What kind of import?

Choosing whether to import the parent module, or each of the types contained within, is something of an art form.

```rust
use std::fs;
use std::collections::VecDeque;
use std::io::prelude::*;
```

## Standard Library

There's also a more compact syntax for imports.

```rust
use std::{fs, io::prelude::*};

fn main() -> std::io::Result<()> {
    let mut f = fs::File::create("hello.txt")?;
    f.write(b"hello")?;
    Ok(())
}
```
