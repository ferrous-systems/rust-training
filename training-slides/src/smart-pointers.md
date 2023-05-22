# Smart Pointers in Rust

---

Rust offers several special pointers to handle different scenarios.

They all have something in common: They are managed by ownership.

## `std::rc::Rc<T>`

Runtime reference counted within a thread.

```rust []
use std::rc::Rc;

struct Point {
   x: i32,
   y: i32,
}

fn main() {
    let rced_point = Rc::new(Point { x: 1, y: 1});
    let first_handle = rced_point.clone();
    let second_handle = rced_point.clone();
}
```

## Semantics

-   `Rc` is a handle on the contained data
-   The handle can be cloned
-   If the last handle drops, drop the data as well
-   `Rc<T>` implements `Deref<Target=T>`

## `std::rc::Weak<T>`

Weak pointer to data.

```rust []
use std::rc::Rc;

struct Point {
   x: i32,
   y: i32,
}

fn main() {
    let rced_point = Rc::new(Point { x: 1, y: 1});
    let first_handle = rced_point.clone();
    let weak = Rc::downgrade(&first_handle);
}
```

## Semantics

-   Similar to `Rc`, however the existence of the data is not guaranteed
-   Single Threaded: The data is guaranteed to be available over the time of an operation
-   Is *not* automatically dereferenced
-   `Rc` cycles are memory leaks, weak pointers prevent that

## Use

-   Frequently used in data structures that require complex cross references
-   Higher runtime costs for more flexibility

## `std::sync::Arc<T>`

A more expensive `Rc` which works across thread boundaries since an atomic counter is used for incrementing.

## Remark

Do not use `Arc` on a hunch. `rustc` rejects code using `Rc` over thread boundaries.

## `std::borrow::Cow`

-   Copy-on-write
-   Abstracts over Borrowing and Ownership
-   Clones Data only when necessary
-   https://doc.rust-lang.org/std/borrow/enum.Cow.html#examples
