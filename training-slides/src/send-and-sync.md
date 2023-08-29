# Send & Sync

---

There are two special traits in Rust for concurrency semantics.

- `Send` marks a structure safe to *send* between threads.
- `Sync` marks a structure safe to *share* between threads.
  - (`&T` is `Send`)

---

These traits are what Rust uses to prevent data races.

They are *automatically derived* for all types if appropriate.

## Automatically Derived

```rust
use std::thread;

#[derive(Debug)]
struct Thing;

// Can send between threads!
fn main() {
    let thing = Thing;
    
    thread::spawn(move || {
        println!("{:?}", thing);
    }).join().unwrap();
}
```

---

There are some notable types which are not `Send` or `Sync`.

Such as `Rc`, raw pointers, and `UnsafeCell`.

## Example: `Rc`

```rust ignore
use std::rc::Rc;
use std::thread;

// Does not work!
fn main() {
    let value = Rc::new(true);
    thread::spawn(move || {
        println!("{:?}", value);
    }).join().unwrap();
}
```

## Example: `Rc` 2

```text
error[E0277]: the trait bound `std::rc::Rc<bool>: std::marker::Send` is not satisfied
 --> <anon>:7:5
  |
7 |     thread::spawn(move || {
  |     ^^^^^^^^^^^^^ the trait `std::marker::Send` is not implemented for `std::rc::Rc<bool>`
```

## Implementing

It's possible to add the implementation of `Send` and `Sync` to a type.

```rust
struct Thing(*mut String);

unsafe impl Send for Thing {}
unsafe impl Sync for Thing {}
```

In these cases, the task of thread safety is left to the implementor.

## Relationships

If a type implements both `Sync` and `Copy` then it can also implement `Send`.

## Relationships

A type `&T` can implement `Send` if the type `T` also implements `Sync`.

```rust ignore
unsafe impl<'a, T: Sync + ?Sized> Send for &'a T {}
```

## Relationships 2

A type `&mut T` can implement `Send` if the type `T` also implements `Send`.

```rust ignore
unsafe impl<'a, T: Send + ?Sized> Send for &'a mut T {}
```

## Consequences

What are the consequences of having `Send` and `Sync`?

## Consequences 2

Carrying this information at the type system level allows driving data race bugs down to a *compile time* level.

Preventing this error class from reaching production systems.

`Send` and `Sync` are independent of the choice of concurrency (async, threaded, etc.).
