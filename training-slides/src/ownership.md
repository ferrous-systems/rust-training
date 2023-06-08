# Ownership and Borrowing

## Ownership

Ownership is the basis for the memory management of Rust.

## Rules

-   Every value has exactly one owner
-   Ownership can be passed on, both to functions and other types
-   The owner is responsible for removing the data from memory
-   The owner always has full control over the data and can mutate it

## These Rules are:

-   fundamental to Rust’s type system
-   enforced at compile time
-   important for optimisations

## Example

```rust [1-13|3|4|5|10-12|6]
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::create("hello.txt")?;
    write_and_close(f);
    // f cannot be used any more - you gave it away
    Ok(())
}

fn write_and_close(mut f: std::fs::File) {
    f.write_all(b"Hello, world!");
}
```

Note:

The statement `let f = ...;` introduces a *variable binding* called `f` and gives it a *value* which is of type `std::fs::File`. This distinction is important when it comes to transferring ownership.

## Does this compile?

```rust compile_fail []
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::create("hello.txt")?;
    write_and_close(f);
    write_and_close(f);
    Ok(())
}

fn write_and_close(mut f: std::fs::File) {
    f.write_all(b"Hello, world!");
}
```

## It does not...

```text
error[E0382]: use of moved value: `f`
  -> src/main.rs:6:21
   |
4  |     let f = std::fs::File::create("hello.txt")?;
   |         - move occurs because `f` has type `File`, which does not implement the `Copy` trait
5  |     write_and_close(f);
   |                     - value moved here
6  |     write_and_close(f);
   |                     ^ value used here after move
   |
```

## Background

* When calling `write_and_close` with `f`, the value *in* `f` is *transferred* into the arguments of `write_and_close`.
* At that moment, ownership passes to `write_and_close`. We say the function *consumed* the value.
* The *variable binding* `f` ceases to exist, and thus `main` is not allowed to access it any more.

## Mutability

* The *variable binding* can be *immutable* (the default) or *mutable*.
* If you own it, you can rebind it and change this.

```rust
fn main() {
    let x = 6;
    // x += 1; ❌
    let mut x = x;
    x += 1; // ✅
}
```

## Borrowing

* Transferring ownership back and forth would get tiresome.
* We can let other functions *borrow* the values we own.
* The outcome of a *borrow* is a *reference*
* There are two kinds of *reference* - *Shared/Immutable* and *Exclusive/Mutable*

## Shared References

* Also called an *immutable reference*.
* Use the `&` operator to borrow (i.e. to make a reference).
* It's like a C pointer but with special compile-time checks.

## Making a Reference

```rust [1-8|4|5|6]
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::create("hello.txt")?;
    let file_ref = &f;
    let file_ref2 = &f;
    Ok(())
}
```

## Taking a Reference

* We can also say a function takes a reference
* We use a type like `&SomeType`:

```rust
fn truncate_file(f: &std::fs::File) -> std::io::Result<()> {
    f.set_len(0)
}
```

## Full Example

```rust
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::create("hello.txt")?;
    truncate_file(&f)?;
    Ok(())
}

fn truncate_file(f: &std::fs::File) -> std::io::Result<()> {
    // We don't need -> syntax here!
    f.set_len(0)
}
```

## How does `set_len` work?

* It's a method on `struct File`...
* But method calls are just *syntactic sugar* for a function call

```rust []
use std::io::prelude::*;

fn truncate_file(f: &std::fs::File) -> std::io::Result<()> {
    // These are equivalent
    // f.set_len(0)
    std::fs::File::set_len(f, 0)
}
```

## What if I own the `File`?

* For method calls Rust does the borrow automatically if required.

```rust
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::create("hello.txt")?;
    f.set_len(0)?;
    // Same as:
    // std::fs::File::set_len(&f, 0)?;
    Ok(())
}
```

## Exclusive References

* Also called a *mutable reference*
* Use the `&mut` operator to borrow (i.e. to make a reference)
* Even stricter rules than the `&` references
* Only a *mutable binding* can make a *mutable reference*

## Exclusive Reference Rules

* There can be only one exclusive reference to an object at any given moment
* You also cannot have shared and exclusive references live at the same time
* Therefore, the compiler knows an `&mut` reference cannot alias any other data

# Rust forbids *shared mutability*

## Making an Exclusive Reference

```rust [1-7|4|5]
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = std::fs::File::create("hello.txt")?;
    let file_ref = &mut f;
    Ok(())
}
```

## Taking an Exclusive Reference

* We can also say a function takes an exclusive reference
* We use a type like `&mut SomeType`:

```rust
use std::io::prelude::*;

fn write_to_file(f: &mut std::fs::File) -> std::io::Result<()> {
    f.write_all(b"Hello, world!")
}
```

## Full Example

```rust []
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = std::fs::File::create("hello.txt")?;
    write_to_file(&mut f)?;
    Ok(())
}

fn write_to_file(f: &mut std::fs::File) -> std::io::Result<()> {
    f.write_all(b"Hello, world!")
}
```

## How does `write_all` work?

* It's a method on `struct File`...
* But method calls are just *syntactic sugar* for a function call

```rust []
use std::io::prelude::*;

fn write_to_file(f: &mut std::fs::File) -> std::io::Result<()> {
    // These are equivalent
    // f.write_all(b"Hello, world!")
    std::fs::File::write_all(f, b"Hello, world!")
}
```

## What if I own the `File`?

* For method calls Rust does the borrow automatically if required.
* Again, there is no need for C's `ptr->field` syntax

```rust []
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = std::fs::File::create("hello.txt")?;
    f.write_all(b"Hello, world!")?;
    // Same as:
    // std::fs::File::write_all(&mut f, b"Hello, world!")?;
    Ok(())
}
```

## Can methods take ownership?

* We saw methods that take `&self` and `&mut self`
* Is there a version that takes ownership?
* Yes!

```rust
struct File();
impl File {
    fn into_raw_fd(self) -> i32 {
        todo!();
    }
}
```

## A Summary

|                  | Owned  | Borrowed | Mutably Borrowed |
| ---------------- | ------ | -------- | ---------------- |
| Types (e.g. i32) | `i32`  | `&i32`   | `&mut i32`       |
| Methods          | `self` | `&self`  | `&mut self`      |

## Are there any alternatives to borrowing?

If you want to give a function their own object, and keeps yours separate, you have two choices:

* Clone
* Copy

## Clone

Some types have a `.clone()` method. It makes a new object, which looks just like the original object.

```rust []
fn main() {
    let data = vec![1, 2, 3];
    let mut data_clone = data.clone();
    data_clone.push(4);
    println!("data = {:?}", data);
    println!("data_clone = {:?}", data_clone);
}
```

## Making things Cloneable

* You can mark your `struct` or `enum` with `#[derive(Clone)]`
* (But only if every value in your `struct`/`enum` itself is `Clone`)

```rust []
#[derive(Clone)]
struct Square {
    width: i32
}

fn main() {
    let sq = Square { width: 10 };
    let sq2 = sq.clone();
}
```

## Copy

* Some types, like integers and floats, are `Copy`
* Compiler copies these objects automatically
* If cloning is very cheap, you could make your type `Copy`

```rust []
fn main() {
    let x = 6;
    do_stuff(x);
    do_stuff(x);
}

fn do_stuff(x: i32) {
    println!("Do I own x, with value {}?", x);
}
```
