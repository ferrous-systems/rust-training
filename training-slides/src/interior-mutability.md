# Interior Mutability

---

In Rust, values are immutable by default.

We can make them mutable with the `mut` keyword.

What if we want just partial mutability? Can we do this?

---

Of course we can!

Our prime accomplices are `Cell<T>` and `RefCell<T>`.

## A motivating example

We have some blog posts which have immutable content, and an incrementing view count.

Ideally, we would have a `fn view(&self) -> &'static str` to return the content, and increment the view count.

## Without `Cell` s

```rust []
#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: usize,
}

impl Post {
    // `mut` is a problem here!
    fn view(&mut self) {
        self.viewed_times += 1;
    }
}
```

## Without `Cell` s

This isn't ideal! `view` takes a `&mut self`, meaning this won't work:

```rust []
fn main() {
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    // This line is a compile error!
    // (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

// From before

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: usize,
}

impl Post {
    // `&mut self` is the problem here!
    fn view(&mut self) {
        self.viewed_times += 1;
    }
}
```

## Without `Cell` s

```rust []
fn main() {
    // We need to make the entire struct mutable!
    let mut post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

// From before

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: usize,
}

impl Post {
    fn view(&mut self) {
        self.viewed_times += 1;
    }
}
```

## Tossing our views into a `Cell`

* `Cell` lets us move and take **values** inside.
* `RefCell` works with **references** through 'dynamic borrowing'.

Let's see our previous example with `Cell`.

## Tossing our views into a `Cell`

```rust []
fn main() {
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: Cell<usize>,
}

impl Post {
    fn view(&self) {
        // Note how we are making a copy, then replacing the original.
        let current_views = self.viewed_times.get();
        self.viewed_times.set(current_views + 1);
    }
}

use std::cell::Cell;
```

## Again with `RefCell`

```rust []
fn main() {
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: RefCell<usize>,
}

impl Post {
    fn view(&self) {
        // Note how we're mutating a value.
        *self.viewed_times.borrow_mut() += 1;
    }
}

use std::cell::RefCell;
```

---

> ...interior mutability is something of a last resort.

<https://doc.rust-lang.org/std/cell/index.html#when-to-choose-interior-mutability>