# Compound Types

## Structs

A `struct` groups and names data of different types.

## Definition

```rust []
struct Point {
    x: i32,
    y: i32,
}
```

Note:

The fields may not be laid out in memory in the order they are written (unless
you ask the compiler to [ensure that they are](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)).

## Construction

- there is no partial initialization

```rust [1-4|6-8]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
}
```

## Construction

- but you can copy from an existing variable of the same type

```rust [8]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let q = Point { x: 4, ..p };
}
```

## Field Access

```rust [1-4|7|8-9]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p.x);
    println!("{}", p.y);
}
```

## Tuples

- Holds values of different types together.
- Like an anonymous `struct`, with fields numbered 0, 1, etc.

```rust [2|3-4]
fn main() {
    let p = (1, 2);
    println!("{}", p.0);
    println!("{}", p.1);
}
```

## `()`

- the *empty tuple*
- represents the absence of data
- we often use this similarly to how you’d use `void` in C

```rust
fn prints_but_returns_nothing(data: &str) -> () {
    println!("passed string: {}", data);
}
```

## Tuple Structs

- Like a `struct`, with fields numbered 0, 1, etc.

```rust [1|4|5-6]
struct Point(i32,i32);

fn main() {
    let p = Point(1, 2);
    println!("{}", p.0);
    println!("{}", p.1);
}
```

## Enums

- An `enum` represents different variations of the same subject.
- The different choices in an enum are called *variants*

<!--
-   stress that enums are an "either or" type: you can only have one
    variant at a time (you’re not accumulating data as with structs)
-   stress that you can only have the variants, not the enum itself
    (i.e. `Movement::Left`. but not `Movement`)
-->

## enum: Definition and Construction

```rust [1-6|9]
enum Shape {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

fn main() {
    let shape = Shape::Rectangle;
}
```

## Enums with Values

```rust [1-5|2|3|4|8|9|10]
pub enum Shapes {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn main() {
    let dot = Shapes::Dot;
    let square = Shapes::Square(10);
    let rectangle = Shapes::Rectangle { width: 10, length: 20 };
}
```

## Enums with Values

- An enum value is the same size, no matter which variant is picked
- It will be the size of the largest variant (plus a tag)

Note:

* From a computer science perspective, `enum`s are tagged unions.
* The tag in an enum specifies which variant is currently valid, and is stored as the
  smallest integer the compiler can get away with - it depends how many variants you
  have. Of course, if none of the variants have any data, the enum is *just* the tag.
* If you have a C background, you can think of this as being a `struct` containing an `int`
  and a `union`.

## Doing a `match` on an `enum`

- When an `enum` has variants, you use `match` to extract the data
- New variables are created from the *pattern* (e.g. `radius`)

```rust [1-5|7-16|10|13]
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn check_shape(shape: Shape) {
    match shape {
        Shape::Square(width) => {
            println!("It's a square, with the width {}", width);
        }
        _ => {
            println!("Try a square instead");
        }
    }
}
```

## Doing a `match` on an `enum`

- There are two variables called `width`
- The binding of `width` in the pattern on line 10 hides the `width` variable on line 8

```rust [8|10]
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn check_shape(shape: Shape) {
    let width = 10;
    match shape {
        Shape::Square(width) => {
            println!("It's a square, with width {}", width);
        }
        _ => {
            println!("Try a square instead");
        }
    }
}
```

Note:

* Rust allows the variable shadowing shown above in general

## Match guards

Match guards allow further refining of a `match`

```rust [9]
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn check_shape(shape: Shape) {
    match shape {
        Shape::Square(width) if width > 10 => {
            println!("It's a BIG square, with width {}", width);
        }
        _ => {
            println!("Try a big square instead");
        }
    }
}
```

## Combining patterns

- You can use the `|` operator to join patterns together

```rust [1-16|9]
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn test_shape(shape: Shape) {
    match shape {
        Shape::Rectangle { width, .. } | Shape::Square(width) => {
            println!("Shape has a width of {}", width);
        }
        _ => {
            println!("Not a rectangle, nor a square");
        }
    }
}
```

## Shorthand: `if let` conditionals

- You can use `if let` if only one case is of interest.
- Still *pattern matching*

```rust []
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn test_shape(shape: Shape) {
    if let Shape::Square(width) = shape {
        println!("Shape is a Square with width {}", width);
    }
}
```

## `if let` chains in newer Rust versions

Newer Rust versions (edition 2024) allow `if let` chaining, for example:

```rust
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn test_shape(shape: Shape) {
    // Hardcoded here, but could be determined by other logic.
    let ignore_rectangle = true;
    if !ignore_rectangle && let Shape::Rectangle(length, height) = shape {
        println!("Shape is a Rectangle with {length} x {height}");
    }
}
```

## Shorthand: `let else` conditionals

- If you expect it to match, but want to handle the error...
- The `else` block must *diverge*

```rust []
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn test_shape(shape: Shape) {
    let Shape::Square(width) = shape else {
        println!("I only like squares");
        return;
    };
    println!("Shape is a square with width {}", width);
}
```

## Shorthand: `while let` conditionals

- Keep looping whilst the pattern still matches

```rust should_panic []
pub enum Shape {
    Dot,
    Square(u32),
    Rectangle { width: u32, length: u32 }
}

fn main() {
    while let Shape::Square(width) = make_shape() {
        println!("got square, width {}", width);
    }
}

fn make_shape() -> Shape {
    todo!()
}
```

## Foreshadowing! 👻

Two very important enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

We'll come back to them after we learn about error handling.
