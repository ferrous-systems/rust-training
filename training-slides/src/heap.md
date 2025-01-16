# Heap Allocation (Box, Rc and Cow)

## Where do Rust variables live?

```rust []
struct Square {
    width: f32
}

fn main() {
    let x: u64 = 0;
    let y = Square { width: 1.0 };
    let mut z: String = "Hello".to_string();
    z.push_str(", world!");
}
```

Note:

* The variable `x` is an 8-byte (64-bit) value, and lives on the stack.
* The variable `y` is a 4-byte value, and also lives on the stack.
* The variable `z` is a 3x4-byte value on 32-bit platforms, and a 3x8-byte value on 64-bit platforms. The `String` itself is a struct, and the bytes contained within the struct live on the heap.

## Let's see some addresses...

```rust []
struct Square {
    width: f32
}

fn main() {
    let x: u64 = 0;
    let y = Square { width: 1.0 };
    let mut z: String = "Hello".to_string();
    z.push_str(", world!");
    println!("x @ {:p}", &x);
    println!("y @ {:p}", &y);
    println!("z @ {:p}", &z);
    println!("z @ {:p}", z.as_str());
}
```

Note:

You expect to see something like:

```text
x @ 0x7ffc2272c618
y @ 0x7ffc2272c624
z @ 0x7ffc2272c628
z @ 0x555829f269d0
```

The first `z @` line is the `struct String { ... }` itself. The second `z @` line are the bytes the `String` contains. They have a different addresses because they are in the heap and not on the stack.

If you run it multiple times, you will get different results. This is due to the Operating System randomizing the virtual addresses used for the stack and the heap, to make security vulnerabilities harder to exploit.

On macOS, you can run `vmmap <pid>` to print the addresses for each region. On Linux you can use `pmap <pid>`, or you could add something like:

```rust
if let Ok(maps) = std::fs::read_to_string(&format!("/proc/{}/maps", std::process::id())) {
    println!("{}", maps);
}
```

## How does Rust handle the heap?

On three levels:

* Talking to your Operating System (or its C Library)
* A low-level API, called the *Global Allocator*
* A high-level API, with `Box`, `Rc`, `Vec`, etc

## What's in the Box?

* A `Box<T>` in Rust, is a handle to a unique, owned, heap-allocated value of type `T`
* The *value* is the size of a pointer
* The *contents* of the Box can be any T (including *unsized* things)

Note:

Pointers can be 'thin' (one word in length) or 'wide' (two words in length). In a wide pointer, the second word holds the length of the thing being pointed to, or a pointer to the vtable if it's a dyn-trait pointer. The same applies to Boxes.

## Why not raw pointers?

Because `Box<T>`:

* doesn't let you do pointer arithmetic on it
* will automatically free the memory when it goes out of scope
* implements `Deref<T>` and `DerefMut<T>`

## Making a Box

The `Deref` and `DerefMut` trait implementations let us use a Box quite naturally:

```rust []
fn main() {
    let x: Box<f64> = Box::new(1.0_f64);
    let y: f64 = x.sin() * 2.0;
    let z: &f64 = &x;
    println!("x={x}, y={y}, z={z}");
}
```

## When should I use a Box?

* Not very often - friendlier containers (like `Vec<T>`) exist
* If you have a large value that moves around a lot
  * Moving a `Box<T>` is cheap, because only the *pointer* moves, not the *contents*
* To hide the size or type of a returned value...

## Boxed Traits

```rust []
fn make_stuff(want_integer: bool) -> Box<dyn std::fmt::Debug> {
    if want_integer {
        Box::new(42_i32)
    } else {
        Box::new("Hello".to_string())
    }
}

fn main() {
    println!("make_stuff(true): {:?}", make_stuff(true));
    println!("make_stuff(false): {:?}", make_stuff(false));
}
```

Note:

An `i32` and a `String` are very different sizes, and a function must have a single fixed size for the return value. But it does - it returns a `Box` and the `Box` itself always has the same size. The thing that varies in size is the *value inside the box* and that lives somewhere else - on the heap in fact.

This trick is also useful for *closures*, where the type cannot even be said out loud because it's compiler-generated. But you can say a closure implements the `FnOnce` trait, for example.

## Smarter Boxes

What if I want my Box to have multiple owners? And for the memory to be freed when *both* of the owners have finished with it?

We have the *reference counted* `Rc<T>` type for that!

## Using `Rc<T>`

```rust []
use std::rc::Rc;

struct Point { x: i32, y: i32 }

fn main() {
    let first_handle = Rc::new(Point { x: 1, y: 1});
    let second_handle = first_handle.clone();
    let third_handle = second_handle.clone();
}
```

## Reference Counting

* The `Rc` type is a *handle* to reference-counted heap allocation
* When you do a `clone()` the count goes up by one
* When you drop it, the count goes down by one
* The memory isn't freed until the count hits zero
* There's a `Weak` version which will not keep the allocation alive - to break cycles

Note:

A cycle would be if you managed to construct two `Rc` wrapped structs and had
each one hold an `Rc` reference to the other. Now neither can ever be freed,
because each will always have at least one owner (the other).

## Thread-safety

* `Rc` cannot be sent into a thread (or through any API that requires the type to be `Send`).
    * If in doubt, try it! Rust will save you from yourself.
* The trade-off is that `Rc` is really fast!
* There is an *Atomic Reference Counted* type, `Arc` if you need it.

## Rc is not mutable

NB: `Rc` allows *sharing*, but not *mutability*...

```rust []
use std::rc::{Rc, Weak};

struct Dog { name: String, owner: Weak<Human> }
struct Human { name: String, pet_dogs: Vec<Dog> }

fn main() {
    let mut sam = Rc::new(
        Human { name: "Sam".to_string(), pet_dogs: Vec::new() }
    );
    let rover = Dog { name: "Rover".to_string(), owner: Rc::downgrade(&sam) };
    // This is not allowed, because `sam` is actually immutable
    // sam.pet_dogs.push(rover);
}
```

Note:

You get an error like:

<pre><code data-trim data-noescape><span class="er b">error[E0596]</span><b>: cannot borrow data in an `Rc` as mutable</b>
<span class="eb b">  --&gt; </span>src/main.rs:12:5
<span class="eb b">   |</span>
<span class="eb b">12 |</span>     sam.pet_dogs.push(rover);
<span class="eb b">   |</span>     <span class="er b">^^^^^^^^^^^^</span> <span class="er b">cannot borrow as mutable</span>
<span class="eb b">   |</span>
<span class="eb b">   = </span><b>help</b>: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc&lt;Human&gt;`
<b>For more information about this error, try `rustc --explain E0596`.</b>
</code></pre>

Why do you want this structure? Because given some `&Dog` you might very well want to know who owns it!

## Shared Mutability

We have more on this later...

```rust []
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Dog { name: String, owner: Weak<RefCell<Human>> }
struct Human { name: String, pet_dogs: Vec<Dog> }

fn main() {
    let mut sam = Rc::new(RefCell::new(
        Human { name: "Sam".to_string(), pet_dogs: Vec::new() }
    ));
    let rover = Dog { name: "Rover".to_string(), owner: Rc::downgrade(&sam) };
    // This is now allowed because `RefCell::borrow_mut` does a run-time borrow check
    sam.borrow_mut().pet_dogs.push(rover);
}
```

## Maybe Boxed, maybe not?

Why is this function less than ideal?

```rust should_panic []
/// Replaces all the ` ` characters with `_`
fn replace_spaces(input: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", replace_spaces("Hello, world!"));
    println!("{}", replace_spaces("Hello!"));
}
```

Note:

Did the second call replace anything? Did you have to allocate a `String` and copy all the data anyway, even though nothing changed?

## Copy-On-Write

Rust has the [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) type to handle this.

```rust should_panic []
/// Replaces all the ` ` characters with `_`
fn replace_spaces(input: &str) -> std::borrow::Cow<str> {
    todo!()
}

fn main() {
    println!("{}", replace_spaces("Hello, world!"));
    println!("{}", replace_spaces("Hello!"));
}
```

Note:

`Cow` works on any `T` where there is both a *Borrowed* version and an *Owned* version.

For example, `&[u8]` and `Vec<u8>`.
