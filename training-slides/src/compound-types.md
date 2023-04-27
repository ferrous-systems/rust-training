# Compound Types

## Structs

`struct`s group and name data of different types.

## Definition
```rust,editable
    struct Point {
        x: i32,
        y: i32,
    }
```

## Construction

-   there is no partial initialization

<!-- -->
```rust,editable
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 1, y: 1 };
    }
```

## Side note

It’s common to hide construction behind a call to `Point::new()` instead
of using a raw struct literal.

## Field Access

-   note: no `->` operator for structs behind pointers, always `.`

<!-- -->

```rust,editable
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

```rust,editable
    fn main() {
        let p = (1, 2);
        println!("{}", p.0);
        println!("{}", p.1);
    }
```
## Tuple Structs

```rust,editable
    struct Point(i32,i32);

    fn main() {
        let p = Point(1, 2);
        println!("{}", p.0);
        println!("{}", p.1);
    }
```
## Enums

`enum`s represent different variation of the same subject.

-   stress that enums are an "either or" type: you can only have one
    variant at a time (you’re not accumulating data as with structs)

-   stress that you can only have the variants, not the enum itself
    (i.e. `Movement::Left`. but not `Movement`)

## Definition and Construction
```rust,editable
    enum Direction {
        Right,
        Left,
        Up,
        Down,
    }

    fn main() {
        let direction = Direction::Left;
    }
```
The different choices of Enums are called "variants."

## Enums with Values

```rust,editable
    enum Movement {
        Right(i32),
        Left(i32),
        Up(i32),
        Down(i32),
    }

    fn main() {
        let movement = Movement::Left(12);
    }
```
## Enums with Structured Variants

-   each enum variant will be its **worst-case** size! (e.g. the size of
    its biggest member)

TODO: Fix this
## **possible interactive detour:** 

Q: what’s the size of `Actions` on
bytes? - correct A: 12, because we have a tagged union:

```c
    struct {
        tag: u32 // discriminant is always u32 => 4 bytes
        data: union {
            stick_around: (), // 0 bytes
            move_to: tsruct{ x: i21, y: i32} // 4 bytes
        }
    }

    enum Actions {
        StickAround,
        MoveTo { x: i32, y: i32},
    }

    fn main() {
        let action = Actions::MoveTo { x: 0, y: 0 };
    }
```

## `null`

Does not exist.

## `()`

-   we often use this similarly to how you’d use `void` in C

The empty tuple `()` represents the absence of data.

```rust,editable
    fn prints_but_returns_nothing(data: &str) -> () {
        println!("passed string: {}", data);
    }
```
