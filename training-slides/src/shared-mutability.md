# Shared Mutability (Cell, RefCell)

## Rust has a simple rule

|           | Immutable | Mutable  |
| --------- | --------- | -------- |
| Exclusive | `&mut T`  | `&mut T` |
| Shared    | `&T`      | 🔥🔥🔥      |

---

These rules can be ... *bent*

(but not broken)


## Why the rules exist...

* Optimizations!
* It is *undefined behaviour* (UB) to have multiple `&mut` references to the *same* object at the *same* time
* You *must* avoid UB

Note:

If you have UB in your program (anywhere), it is entirely valid for the compiler to delete your entire program and replace it with an empty program.

## Bending the rules

There is only *one* way to modify data through a `&T` reference:

`UnsafeCell`

## `UnsafeCell`

```rust []
use std::cell::UnsafeCell;

fn main() {
    let x: UnsafeCell<i32> = UnsafeCell::new(42);

    let exc_ref: &mut i32 = unsafe { &mut *x.get() };
    *exc_ref += 27;
    drop(exc_ref);

    let shared_1: &i32 = unsafe { &*x.get() };
    assert_eq!(*shared_1, 42 + 27);
    let shared_2: &i32 = unsafe { &*x.get() };
    assert_eq!(*shared_1, *shared_2);
}
```

Note:

The `UnsafeCell::get(&self) -> *mut T` method is safe, but dereferencing the pointer (or converting it to a `&mut` reference) is unsafe because a human must verify there is no aliasing.

## Can we be safer?

A human must do a lot of manual checks here.

Can we make it nicer to use?

## `Cell`

A `Cell` is safe to use.
But you can only *copy* in and *copy* out.

## A motivating example

We have some blog posts which have immutable content, and an incrementing view count.

Ideally, we would have a `fn view(&self) -> &str` to return the content, and increment the view count.

## Without `Cell` s

```rust []
#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: u64,
}

impl Post {
    // `mut` is a problem here!
    fn view(&mut self) -> &str {
        self.viewed_times += 1;
        &self.content
    }
}
```

## Without `Cell`

This isn't ideal! `view` takes a `&mut self`, meaning this won't work:

```rust []
fn main() {
    let post = Post { content: "Blah".into(), ..Post::default() };
    // This line is a compile error!
    // println!("{}", post.view());
}

// From before

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: u64,
}

impl Post {
    // `&mut self` is the problem here!
    fn view(&mut self) -> &str {
        self.viewed_times += 1;
        &self.content
    }
}
```

## Without `Cell`

```rust []
fn main() {
    // We need to make the entire struct mutable!
    let mut post = Post { content: "Blah".into(), ..Post::default() };
    println!("{}", post.view());
    // Now this is allowed too...
    post.content.push_str(" - extra content");
}

// From before

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: u64,
}

impl Post {
    fn view(&mut self) -> &str {
        self.viewed_times += 1;
        &self.content
    }
}
```

## Using `Cell` instead

Let's see our previous example with `Cell`.

```rust []
fn main() {
    let post = Post {
        content: "Blah".into(),
        ..Post::default()
    };
    println!("{}", post.view());
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: std::cell::Cell<u64>,
}

impl Post {
    fn view(&self) -> &str {
        // Note how we are making a copy, then replacing the original.
        let current_views = self.viewed_times.get();
        self.viewed_times.set(current_views + 1);
        &self.content
    }
}
```

Note:

As an in-depth example of the borrow checker's limitations, consider the [Splitting Borrows](https://doc.rust-lang.org/nomicon/borrow-splitting.html) idiom, which allows one to borrow different fields of the same struct with different mutability semantics:

```rust
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

let mut x = Foo {a: 0, b: 0, c: 0};
let a = &mut x.a;
let b = &mut x.b;
let c = &x.c;
*b += 1;
let c2 = &x.c;
*a += 10;
println!("{} {} {} {}", a, b, c, c2);
```

The code works, but, once you have mutably borrowed a field you cannot mutably borrow the whole value (e.g. by calling a method on it) at the same time - otherwise you could get two mutable references to the same field at the same time.

Here's an example where tuple fields are special-cased for the borrow checker:

```rust ignore
let mut z = (1, 2);
let r = &z.1;
z.0 += 1;
println!("{:?}, {}", z, r);
```

but fails on an equivalent array

```rust ignore
let mut z = [1, 2];
let r = &z[1];
z[0] += 1;
println!("{:?}, {}", z, r);
```

## `RefCell`

A `RefCell` is also safe, but lets you *borrow* or *mutably borrow* the contents.

The borrow checking is deferred to *run-time*

## Using `RefCell`

```rust []
use std::cell::RefCell;
 
fn main() {
    let x: RefCell<i32> = RefCell::new(42);
 
    let mut exc_ref = x.borrow_mut();
    *exc_ref += 27;
    drop(exc_ref);
 
    let shared_1 = x.borrow();
    // This isn't allowed here:
    // let exc_ref = x.borrow_mut();
    assert_eq!(*shared_1, 42 + 27);
    let shared_2 = x.borrow();
    assert_eq!(*shared_1, *shared_2);
}
```

## Using `RefCell` instead

Let's see our previous example with `RefCell`.

```rust []
fn main() {
    let post = Post { content: "Blah".into(), ..Post::default() };
    println!("{}", post.view());
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: std::cell::RefCell<u64>,
}

impl Post {
    fn view(&self) -> &str {
        let mut view_count_ref = self.viewed_times.borrow_mut();
        *view_count_ref += 1;
        &self.content
    }
}
```

## `RefCell` Tradeoffs

Moving the *borrow checking* to run-time:

* Might make your program actually compile 😀
* Might cause your program to panic 😢

> interior mutability is something of a last resort

-- [The Rust Documentation](https://doc.rust-lang.org/std/cell/index.html)

## Using with `Rc`

To get *shared ownership* and *mutability* you need two things:

* `Rc<RefCell<T>>`
* (Multi-threaded programs might use `Arc<Mutex<T>>`)

## `OnceCell` for special cases

A `OnceCell` lets you initialise a value using `&self`, but not subsequently modify it.

```rust
fn main() {
    let post: Post = Post { content: "Blah".into(), ..Post::default() };
    println!("{:?}", post.first_viewed());
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    first_viewed_at: std::cell::OnceCell<std::time::Instant>,
}

impl Post {
    fn first_viewed(&self) -> std::time::Instant {
        self.first_viewed_at.get_or_init(std::time::Instant::now).clone()
    }
}
```
