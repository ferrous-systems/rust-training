# Deref Coercions

## Motivation

Why does the following work?

```rust
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let boxed_p = Box::new(Point { x: 1, y: 2 });
    println!("{}", boxed_p.x);
}
```

Box doesn't have a field named "x"!

## Auto-Dereferencing

Rust automatically dereferences in certain cases. Like everything else, it must be explicitly requested:

- Through a call or field access using the `.` operator
- By explicitly dereferencing through `*`
- When borrowing through `&`
- This sometimes leads to the ugly `&*`-Pattern

---

This makes wrapper types very ergonomic and easy to use!

---

Dereferencing is described by the `Deref` and `DerefMut`-Traits.

```rust ignore
impl<T> std::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!()
    }
}
```

This call is introduced when dereferencing is requested.

## Important deref behaviours

- `String --> &str`
- `Vec<T> --> &[T]`

Functions that don't modify the lengths of a String or a Vector should accept a slice instead. The memory layout is chosen so that this is *cost free*.

---

```rust
fn print_me(message: &str) { println!("{}", message); }

fn main() {
    print_me("Foo");
    let a_string = String::from("Bar");
    print_me(&a_string);
    print_me(a_string.as_str())
}
```
