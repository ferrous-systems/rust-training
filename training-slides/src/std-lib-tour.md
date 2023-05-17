# `std` Library Tour

---

It's time for a tour of some interesting parts in `std`.

We will focus on parts we have not otherwise covered.

## Collections

[`std::collections`](https://doc.rust-lang.org/std/collections/index.html)

Contains a number of valuable data structures. In particular:

* [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) for storing sequences of values.
* [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) for storing key value pairs.

When seeking to optimize code other options may be appropriate.

## Entry

[`std::collections::hash_map::Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)

Calling `.entry()` on a `HashMap` accesses this API and allows for 'insert or update' access.

```rust []
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("foo").or_insert(1);
    map.entry("bar").or_insert_with(|| {
        let mut value = 1;
        value += 1;
        value
    });
    println!("{:?}", map);
}
```

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

## Synchronization Primitives

[`std::sync`](https://doc.rust-lang.org/std/sync/)

Provides types such a `Mutex`, `RwLock`, `CondVar`, `Arc` and `Barrier` s.

```rust []
use std::sync::Mutex;

fn main() {

    let mut mutex = Mutex::new(0);
    
    // Use a new scope to force a drop.
    {
        let mut val = mutex.get_mut().unwrap();
        *val += 1;
    }
    
    println!("{}", *mutex.lock().unwrap());
}
```

## Read and Write

[`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) & [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html)

Generic read and write functionality to files, sockets, buffers, and anything in between.

Also part of [`std::io::prelude`](https://doc.rust-lang.org/std/io/prelude/) (`use std::io::prelude::*`).

```rust []
use std::io::{Read, Write};

fn main() {
    // File, Socket, Vector, ...
    let mut buffer: Vec<u8> = vec![];
    
    buffer.write(b"some bytes").unwrap();
    
    let mut read_in = String::new();
    buffer.as_slice().read_to_string(&mut read_in).unwrap();
    
    println!("{}", read_in);
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

