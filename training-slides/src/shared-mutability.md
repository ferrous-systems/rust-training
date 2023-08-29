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

* Optimisations!
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
    let (p1, p2) = (&x, &x);

    let p1_exclusive: &mut i32 = unsafe { &mut *p1.get() };
    *p1_exclusive += 27;
    drop(p1_exclusive);

    let p2_shared: &i32 = unsafe { &*p2.get() };
    assert_eq!(*p2_shared, 42 + 27);
    let p1_shared: &i32 = unsafe { &*p1.get() };
    assert_eq!(*p1_shared, *p2_shared);
}
```

Note:

The `UnsafeCell::get(&self) -> *mut T` method is safe, but deferencing the pointer (or converting it to a `&mut` reference) is unsafe because a human must verify there is no aliasing.

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
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
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

## Without `Cell` 2

```rust []
fn main() {
    // We need to make the entire struct mutable!
    let mut post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
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
        content: String::from("Blah"),
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

## `RefCell`

A `RefCell` is also safe, but lets you *borrow* or *mutably borrow* the contents.

The borrow-checking is deferred to *run-time*

## Using `RefCell`

```rust []
use std::cell::RefCell;

fn main() {
    let x: RefCell<i32> = RefCell::new(42);
    let (p1, p2) = (&x, &x);

    let mut p1_exclusive = p1.borrow_mut();
    *p1_exclusive += 27;
    drop(p1_exclusive);

    let p2_shared = p2.borrow();
    assert_eq!(*p2_shared, 42 + 27);
    // This isn't allowed here:
    // let p2_mutable = p2.borrow_mut();
    let p1_shared = p1.borrow();
    assert_eq!(*p1_shared, *p2_shared);
}
```

## Using `RefCell` instead

Let's see our previous example with `RefCell`.

```rust []
fn main() {
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
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
