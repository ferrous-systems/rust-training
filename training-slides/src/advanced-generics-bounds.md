# Advanced Generics: Bounds

---

Sometimes, we want to constrain a type to certain aspects, while still writing generic code.

To achieve this, we can constrain type parameters.

---

This can happen at any point where type parameters are used.

---

Example: `println!("{:?}")` requires Debug

```rust []
use std::fmt::Debug;

fn print_everything<T: Debug>(to_print: T) {
    println!("{:?}", to_print);
}

fn print_everything2<T>(to_print: T)
    where T: Debug
{
    println!("{:?}", to_print);
}
```

---

Example: A generic Struct that requires inner values to implement `Debug`

```rust []
use std::fmt::Debug;

struct MyStruct<T: Debug> {
    inner: T
}
```

---

Bounds can also be expressed for implementation targets:

```rust []
trait Distance<T> { /* ... */ }

trait Centered {
    fn center(&self) -> (i32, i32);
}

impl<X,T> Distance<X> for T
    where T: Centered,
          X: Centered {
}
```

---

Traits can also directly require prerequisites:

```rust []
use std::fmt::Debug;

trait Logger<X: Debug> {
    fn log(&self, x: X);
}
```

---

Rust does not allow negative Bounds (Trait A and *not* Trait B)

## Exception: `Sized`

If not specified otherwise, all type parameters carry the bound `Sized` (the type has a statically known memory size). This can be suppressed by using the bound `?Sized`.

```rust []
fn take_unsized<T: ?Sized>(t: &T) {
    //...
}
```

---

This has ergonomic reasons, as passing types by value is common and requires a known size.

---

Bounds can be used everywhere, which can be used to de-facto constrain types at the call site.

```rust []
use std::fmt::Debug;

struct Wrapper<T> {
    inner: T
}

impl<T> Wrapper<T> {
    fn new(inner: T) -> Wrapper<T> where T: Debug {
        Wrapper { inner: inner }
    }

    fn inspect(&self) where T: Debug {
        println!("{:?}", &self.inner);
    }
}
```

---

This can be very practical, as this allows expressing different bounds during construction and at call sites.

---

Bounds are very common in conversion functions.

```rust []
use std::path::Path;

fn open_file<P: AsRef<Path>>(pathlike: P) {
    let path = pathlike.as_ref();
}
```

## Generic implementations

Bounds can be used to constrain the target of an implementation.

```rust []
use std::fmt::Debug;

trait Log<T> {
    fn log(&self, t: T);
}

impl<T> Log<T> for T where T: Debug {
    fn log(&self, t: T) {
        println!("Logging: {:?}", t);
    }
}
```

## Trait Inheritance

Traits can also request the implementation of other traits and declare default implementations for methods relying on that information.

```rust []
struct Address;

trait Named {
    fn name(&self) -> &'static str;
}

trait Person : Named {
    fn home_address(&self) -> Option<Address>;
}
```
