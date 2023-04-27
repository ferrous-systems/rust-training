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

## Side note

It's common to hide construction behind a call to `Point::new()` instead
of using a raw struct literal.

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

* Holds values of different types together.
* Like an anonymous `struct`, with fields numbered 0, 1, etc.

```rust [1|2-3]
let p = (1, 2);
println!("{}", p.0);
println!("{}", p.1);
```

## Tuple Structs

* Like a `struct`, with fields numbered 0, 1, etc.

```rust [1|3|4-5]
struct Point(i32,i32);

let p = Point(1, 2);
println!("{}", p.0);
println!("{}", p.1);
```
## Enums

* An `enum` represents different variations of the same subject.
* The different choices in an enum are called *variants*

<!--
-   stress that enums are an "either or" type: you can only have one
    variant at a time (you’re not accumulating data as with structs)
-   stress that you can only have the variants, not the enum itself
    (i.e. `Movement::Left`. but not `Movement`)
-->

## enum: Definition and Construction

```rust [1-6|8]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

let direction = Direction::Left;
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

## Enums with Structured Variants

-   An enum is the same size, no matter which variant is picked
-   It will be the size of the largest variant

## `null`

* Does not exist.
* (unless we're talking about unsafe pointers...)

## `()`

-   the *empty tuple*
-   represents the absence of data
-   we often use this similarly to how you’d use `void` in C

```rust
fn prints_but_returns_nothing(data: &str) -> () {
    println!("passed string: {}", data);
}
```
