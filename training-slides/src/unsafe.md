# Unsafe Rust

---

Rust's type system provides many guarantees, but sometimes, they make specific solutions hard or impossible.

For that reason, Rust has the concept of "unsafe code".

---

Unsafe code is allowed to:

- freely access memory
- dereference raw pointers
- call external functions
- declare values `Send` and `Sync`
- write to unsynced global variables

---

By definition, these are not unsafe:

- conversion to raw pointers
- memory leaks

## Making pointers

```rust
#![allow(unused_variables)]
fn main() {
    let mut x = 1;
    // The old way
    let p1 = &x as *const i32;
    let p2 = &mut x as *mut i32;
    // Added in 1.51, was unsafe until 1.82
    let p1 = core::ptr::addr_of!(x);
    let p2 = core::ptr::addr_of_mut!(x);
    // As of Rust 1.82, use this instead:
    let p1 = &raw const x;
    let p2 = &raw mut x;    
}
```

---

Unsafe code should never:

- be used to manage memory managed by a different allocator (e.g. construct a `std:::vec::Vec` from a C++ vector and drop it)
- cheat on the borrow checker, for example by changing lifetimes or mutability of a type. The most common source of "but I was so sure that works" bugs.

## Rust's little secret

When implementing data structures, unsafe isn't unusual.

Safe Rust is the worst language to implement linked lists. There's a full [text on this](https://rust-unofficial.github.io/too-many-lists/)

---

Unsafe code must *always* be marked `unsafe`.

```rust []
fn main() {
    let mut x = 1;
    let p = &raw mut x;
    unsafe {
        my_write(p, 100);
    }
    println!("x is {} (or {})", x, unsafe { p.read() });
}

pub unsafe fn my_write<T>(p: *mut T, new_value: T) {
    p.write(new_value)
}
```

Note:

Modern Rust generally tries to have only a small number of `unsafe` operations
per `unsafe` block. And any unsafe function *should* still use `unsafe` blocks for
the unsafe code within, even though the function itself is unsafe to call.

Try running `clippy` on this example and play with `clippy::multiple_unsafe_ops_per_block` and `clippy::undocumented_unsafe_blocks`. Then try "Edition 2024".

## Traps of `unsafe`

- Not all examples are that simple. `unsafe` *must* guarantee the invariants that Rust expects.
- This *especially* applies to ownership and mutable borrowing
- `unsafe` can lead to a value having 2 owners -&gt; double free
- `unsafe` can make immutable data temporarily mutable, which will lead to broken promises and tears.

---

Rust allows you to shoot yourself in the foot, it just requires you to take your gun out of the holster and remove the safety first.

## Practical example

As Rust forbids aliasing, it is impossible in safe Rust to split a slice into 2 non-overlapping parts.

```rust []
#[inline]
fn split_at_mut<T>(value: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = value.len();
    let ptr = value.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid),
         std::slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}
```

## Highlight unsafe code in VSCode

- Will highlight which function calls are `unsafe` inside an `unsafe` block
- Helpful for longer `unsafe` blocks

```json
{
    "editor.semanticTokenColorCustomizations": {
        "rules": {
            "*.unsafe:rust": "#ff00ff"
        }
    }
}
```
