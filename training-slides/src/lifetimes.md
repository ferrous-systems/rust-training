# Lifetimes

---

```rust ignore
struct Point {
    x: i32,
    y: i32,
}

// error[E0106]: missing lifetime specifier
fn return_point() -> &Point {
    let p = Point { x: 1, y: 2 };
    &p
}

fn main() {
    return_point();
}
```

---

```rust
struct Point {
    x: i32,
    y: i32
}

fn return_point() -> Box<Point> {
    let p = Point { x: 1, y: 2 };
    Box::new(p)
}
```

---

Rust's lifetimes are notorious for being hard to understand.

---

That is not necessary.

## Lifetimes

-   Lifetimes describe the time that values remain in memory
-   They describe - they cannot force or change anything
-   Lifetimes are types!

## You have used them already

```rust
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn x(&self) -> &i32 {
        &self.x
    }

    fn y<'point>(&'point self) -> &'point i32 {
        &self.y
    }
}
```

## Motivation

```rust ignore
struct Container<T> {
    inner: &T,
}

impl<T> Container<T> {
    fn borrow_inner(&self) -> &T {
        self.inner
    }
}

fn inner_drops_before_container() {
    let s = "hello".to_string();
    let c = Container { inner: &s };
    drop(s);
}

fn container_drops_with_active_borrow() {
    let s = "hello".to_string();
    let c = Container { inner: &s };
    let borrowed_s = c.borrow_inner();
    drop(c);
}
```

---

This code would - if it compiled - violate memory safety.

---

The correct struct definition is:

```rust
struct Container<'container, T> {
    inner: &'container T,
}
```

---

Container is now:

* Generic over the type parameter T
* As well as a lifetime specified by `'container`
* The borrowed values must live *at least equally long*

---

Takeaway:

Lifetimes describe minimal conditions

## Multiple lifetimes in one signature

```rust
// slice::split_at

fn split_at<T>(slice: &[T], mid: usize) -> (&[T], &[T]) {
    todo!()
}

fn split_at_explicit<'a, T>(slice: &'a [T], mid: usize) -> (&'a [T], &'a [T]) {
    todo!()
}
```

## Sublifetimes

```rust
use std::str::Split;

struct Tokenizer<'input> {
    input: Split<'input, char>,
}

impl<'input> Tokenizer<'input> {
    fn next_token(&mut self) -> Option<&'input str> {
        self.input.next()
    }
}

struct Parser<'tokenizer, 'input: 'tokenizer> {
    tokenizer: &'tokenizer mut Tokenizer<'input>,
}

impl<'tokenizer, 'input: 'tokenizer> Parser<'tokenizer, 'input> {
    fn next_item(&mut self) -> Option<&'input str> {
        self.tokenizer.next_token()
    }
}

fn main() {
    let mut tok = Tokenizer { input: "( foo bar )".split(' ') };
    let mut parser = Parser { tokenizer: &mut tok };

    println!("{:?}", parser.next_item());
    let content = parser.next_item();
    let content2 = parser.next_item();
    println!("{:?}", parser.next_item());
    drop(parser);

    println!("{:?}", content);
    println!("{:?}", content2);

}
```

---

Lingo: `Input outlives Tokenizer`

---

Lifetimes cannot do _more_ than describe "this must live longer (or at least equally long) as the other".

---

Common pitfall: you cannot "shorten a lifetime", as it just describes what's already there.

## `'static`

* as part of a reference, `'static` means "lives forever"
* in trait bounds, the type does not contain any non-static references. That's not necessarily forever!  

---

Examples of `'static` data are:

* Data contained in the binary, for example static strings
* Heap-allocated values (for example the contents of a `Box`)
    - As long as they are not bound on values that live shorter!
* Globals

---

`'static` is not an escape hatch. In concurrent, especially evented, programs, `'static` is very common.

This is due to most data having to live outside of the stack.

---

Lifetimes describe all types, not only references, therefore they are also bounds in generic code.

```rust
fn inspect<'a, T: std::fmt::Debug + 'a>(t: T) {
    println!("{:?}", t);
}
```

## Lifetime Elision

For simple cases, lifetimes are automatically inserted into signatures.

```rust
fn foo(bar: &str) -> &str {
    todo!();
}

fn foo_explicit<'a>(bar: &'a str) -> &'a str {
    todo!();
}
```

## Lifetimes and Bindings

```rust
let mut sink = std::io::BufWriter::new(std::io::stdout().lock());
```

```rust
let stdout = std::io::stdout();
let mut sink = std::io::BufWriter::new(stdout.lock());
```

## Lifetimes and Boxes

For boxes, the default lifetime bound of the contained value is `'static`. Sometimes, this is too long and can be overwritten:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let i = make_iter(&v);
}

fn make_iter<'a>(v: &'a Vec<u8>) -> Box<impl Iterator<Item = &u8> + 'a> {
    Box::new(v.iter())
}
```
