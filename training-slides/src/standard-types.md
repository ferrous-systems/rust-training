# Standard Types

---

There several pervasive types in Rust.

They leverage the powerful type system to accomplish fundamental tasks.

## Overview

-   `Option<T>` - Removes the need for a `null` primitive.
-   `Result<T,E>` - Removes the need for exceptions.
-   `Vec<T>` - Growable arrays.
-   `HashMap<K,V>` - Key value storage.

## `Option<T>`

```rust []
enum Option<T> {
    Some(T),
    None,
}
```

Options are wrapper types, and need to be unwrapped to be used.

## `Option<T>`

Any function which does not always return a value returns an `Option<T>`.

```rust []
fn main() {
    let values = vec![1, 2, 3];
    println!("{:?}", values.get(0)); // Some(1)
    println!("{:?}", values.get(4)); // None
}
```

## `Option<T>`: Benefit

The programmer *always* knows where a `None` may appear, and is able to decide how the situation should be handled.

This characteristic helps *remove mystery* from the coding process, and aids in confidence.

## `Option<T>`: Unwrapping

`unwrap()` will panic the application if the value is `None`.

This is only recommended in testing and prototyping.

```rust [] should_panic
fn main() {
    let nothing: Option<usize> = None;
    nothing.unwrap();
}
```

## `Option<T>`: Safety

`match` is one of several ways to safety work with `Option`s.

```rust []
fn main() {
    let maybe_a_value = Some(1);
    match maybe_a_value {
        Some(v) => println!("{}", v),
        None    => println!("None"),
    }
}
```
No matter what the value of `maybe_a_value`, the program will never crash.

## `Option<T>`: Questions

Does this type completely remove the need for a `null` primitive?

What are the benefits?

## `Result<T,E>`

```rust []
enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

Results are wrapper types which either contain the successful value, or the error value.

## `Result<T,E>`: Using

Results can be handled via `unwrap()` just like `Option` types, and can be handled in the same ways.

```rust []
fn main() {
    if let Err(e) = std::fs::File::open("nein") {
        println!("{:?}", e);
    }
}
```
Handling complex error scenarios will be addressed in a later chapter.

## `Result<T,E>`: Questions

Does this type completely remove the need for exceptions?

What are the benefits?

## `Vec<T>`

Owned, mutable, growable arrays. Located on the heap.

```rust []
struct Vec<T> {
    items: *mut T,
    length: usize,
    capacity: usize,
}
```

## `Vec<T>`: Creation

Create with `Vec::new()` or the `vec![]` macro.

```rust []
fn main() {
    let explicit_type = Vec::<usize>::new();
    let mut implicit_type = Vec::new();
    implicit_type.push(1);
    let macro_created = vec![1, 2, 3];
}
```

## `Vec<T>`: As a Slice

`Vec<T>` implements `Deref<Target=[T]>`, so it can be easily used as a slice.

```rust []
fn main() {
    let items = vec![1, 2, 3];
    let ref_to_items: &[usize] = &items;
}
```

## `HashMap<K,V>`

HashMaps are key value stores. Keys must implement `Hash`.

```rust []
use std::collections::HashMap;

fn main() {
    let mut kv_store = HashMap::new();
    kv_store.insert("key", true);
    println!("{:?}", kv_store.get("key"));
}
```

## `HashMap<K,V>`: `entry()`

Manipulate a key's cooresponding entry in place.

```rust []
use std::collections::HashMap;

fn main() {
    let mut kv_store = HashMap::new();
    let mut value = kv_store.entry("key").or_insert(true);
    *value = false;
}
```

