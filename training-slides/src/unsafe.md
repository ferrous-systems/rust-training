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

Not unsafe are:

- conversion to raw pointers
- memory leaks

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
use std::fmt::Debug;

fn main() {
    let pointer_to_int = &mut 1;
    let raw = pointer_to_int as *mut i32;
    unsafe { deref_pointer(raw) };
}

unsafe fn deref_pointer<T: Debug>(p: *mut T) {
    println!("{:?}", *p)
}
```

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
