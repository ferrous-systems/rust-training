# `std` Library Tour

---

It's time for a tour of some interesting parts in `std`.

We will focus on parts we have not otherwise covered.

## PhantomData

[`std::marker::PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)

Zero-sized types are used to mark things that "act like" they own a `T`.

These are useful for types which require markers, generics, or use unsafe code.

```rust []
use std::marker::PhantomData;

struct HttpRequest<ResponseValue> {
    // Eventually returns this type.
    response_value: PhantomData<ResponseValue>,
}

fn main() {}
```

## Command

[`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html)

A process builder, providing fine-grained control over how a new process should be spawned.

Used for interacting with other executables.

```rust []
use std::process::Command;

fn main() {
    Command::new("ls")
            .args(&["-l", "-a"])
            .spawn()
            .expect("ls command failed to start");
}
```

## Filesystem Manipulation

[`std::fs`](https://doc.rust-lang.org/std/fs/) & [`std::path`](https://doc.rust-lang.org/std/path/)

Path handling and file manipulation.

```rust []
use std::fs::{File, canonicalize};
use std::io::Write;

fn main() {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    
    let path = canonicalize("foo.txt").unwrap();
        
    let components: Vec<_> = path.components().collect();
    println!("{:?}", components);
}
```
