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

## Construction

-   there is no partial initialization

```rust [1-4|6]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
```

## Construction

-   but you can copy from an existing variable of the same type

```rust [7]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
let q = Point { x: 4, ..p };
```

## Field Access

```rust [1-4|6|7-8]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };
println!("{}", p.x);
println!("{}", p.y);
```

## Tuples

-   Holds values of different types together.
-   Like an anonymous `struct`, with fields numbered 0, 1, etc.

```rust [1|2-3]
let p = (1, 2);
println!("{}", p.0);
println!("{}", p.1);
```

## `()`

-   the *empty tuple*
-   represents the absence of data
-   we often use this similarly to how you’d use `void` in C

```rust
fn prints_but_returns_nothing(data: &str) -> () {
    println!("passed string: {}", data);
}
```

## Tuple Structs

-   Like a `struct`, with fields numbered 0, 1, etc.

```rust [1|3|4-5]
struct Point(i32,i32);

let p = Point(1, 2);
println!("{}", p.0);
println!("{}", p.1);
```

## Enums

-   An `enum` represents different variations of the same subject.
-   The different choices in an enum are called *variants*

<!--
-   stress that enums are an "either or" type: you can only have one
    variant at a time (you’re not accumulating data as with structs)
-   stress that you can only have the variants, not the enum itself
    (i.e. `Movement::Left`. but not `Movement`)
-->

## enum: Definition and Construction

```rust [1-6|8]
enum Shape {
    Square,
    Circle,
    Rectangle,
    Triangle,
}

let shape = Shape::Rectangle;
```

## Enums with Values

```rust [1-6|2-4|5|8|9]
enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down { speed: i32, excitement: u8 },
}

let movement = Movement::Left(12);
let movement = Movement::Down { speed: 12, excitement: 5 };
```

## Enums with Values

-   An enum is the same size, no matter which variant is picked
-   It will be the size of the largest variant

## Doing a `match` on an `enum`

-   When an `enum` has variants, you use `match` to extract the data
-   New variables are created from the *pattern* (e.g. `radius`)

```rust [1-4|7-14|8|11]
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn check_shape(shape: &Shape) {
    match shape {
        Shape::Circle(radius) => {
            println!("It's a circle, with radius {}", radius);
        },
        _ => {
            println!("Try a circle instead");
        }
    }
}
```

## Doing a `match` on an `enum`

-   There are two variables called `radius`
-   The later one hides the earlier one

```rust [7|9]
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn check_shape(shape: &Shape) {
    let radius = 10;
    match shape {
        Shape::Circle(radius) => {
            println!("It's a circle, with radius {}", radius);
        },
        _ => {
            println!("Try a circle instead");
        }
    }
}
```

## Match guards

Match guards allow further refining of a `match`

```rust [8]
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn check_shape(shape: &Shape) {
    match shape {
        Shape::Circle(radius) if *radius > 10 => {
            println!("It's a BIG circle, with radius {}", radius);
        },
        _ => {
            println!("Try a big circle instead");
        }
    }
}
```

## Combining patterns

-   You can use the `|` operator to join patterns together

```rust [1-16|9]
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
    Square(i32),
}

fn test_shape(shape: &Shape) {
    match shape {
        Shape::Circle(size) | Shape::Square(size) => {
            println!("Shape has single size field {}", size);
        },
        _ => {
            println!("Not a circle, nor a square");
        },
    }
}
```

## Shorthand: `if let` conditionals

-   You can use `if let` if only one case is of interest.
-   Still *pattern matching*

```rust []
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn test_shape(shape: &Shape) {
    if let Shape::Circle(radius) = shape {
        println!("Shape is a Circle with radius {}", radius);
    }
}
```

## Shorthand: `let else` conditionals

-   If you expect it to match, but want to handle the error...
-   The `else` block must *diverge*

```rust []
enum Shape {
    Circle(i32),
    Rectangle(i32, i32),
}

fn test_shape(shape: &Shape) {
    let Shape::Circle(radius) = shape else {
        println!("I only like circles");
        return;
    };
    println!("Shape is a Circle with radius {}", radius);
}
```

## Shorthand: `while let` conditionals

-   Keep looping whilst the pattern still matches

```rust []
fn main() {
    let mut numbers = vec![1, 2, 3];
    while let Some(num) = numbers.pop() {
        println!("popped number {}", num);
    }
}
```

## Very Important Enum #1 - Option

```rust
enum Option<T> {
    Some(T),
    None
}

let mut iter = "Hello".chars().into_iter();
while let Some(ch) = iter.next() {
    println!("ch: {}", ch);
}
```

## Very Important Enum #2 - Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}

match std::fs::File::open("hello.txt") {
    Ok(_file_handle) => {
        println!("I opened the file OK");
    }
    Err(error_value) => {
        println!("Failed to open file due to error: {:?}", error_value);
    }
}
```

## [Option](https://doc.rust-lang.org/std/option/enum.Option.html) and [Result](https://doc.rust-lang.org/std/result/enum.Result.html) have lots of useful methods

```rust
fn main() {
    let file_length = std::fs::File::open("hello.txt")
        .and_then(|file| file.metadata())
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    println!("File length is {}", file_length);
}
```

The `|x| ...` syntax indicates a *closure*
