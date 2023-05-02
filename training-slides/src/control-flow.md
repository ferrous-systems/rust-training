# Control Flow

## Control Flow primitives

-   `if` expressions
-   `loop` and `while` loops
-   `match` expressions
-   `if let`, `let else` and `while let`
-   `for` loops
-   `return` and `?`

## Using `if` as a statement

-   Tests if a boolean expression is `true` 
-   Parentheses around the conditional are not necessary
-   Blocks need brackets, no shorthand

```rust []
fn main() {
    if 1 == 2 {
        println!("integers are broken");
    } else if 'a' == 'b' {
        println!("characters are broken");
    } else {
        println!("that's what I thought");
    }
}
```

## Using `if` as an expression

-   Every block is an expression
-   Note the final `;` to terminate the `let` statement.

```rust []
fn main() {
    let x = if 1 == 2 {
        100
    } else if 'a' == 'b' {
        200
    } else {
        300
    };
}
```

## Using `if` as the final expression

Now the `if` expression is the result of the function:

```rust []
fn some_function() -> i32 {
    if 1 == 2 {
        100
    } else if 'a' == 'b' {
        200
    } else {
        300
    }
}
```

## Looping with `loop`

`loop` is used for (potentially) infinite loops

```rust []
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 100 { break; }
    }
}
```

## Looping with `loop`

`loop` blocks are also expressions...

```rust []
fn main() {
    let mut i = 0;
    let loop_result = loop {
        i += 1;
        if i > 10 { break 6; }
        println!("i = {}", i);
    };
    println!("loop_result = {}", loop_result);
}
```

## `while`

* `while` is used for conditional loops.
* Loops while the boolean expression is `true`

```rust []
fn main() {
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("i = {}", i);
    }
}
```

## Control Flow with `match`

-   The `match` keyword does *pattern matching*
-   You can use it a bit like an `if/else if/else` expression
-   The first arm to match, wins
-   `_` means *match anything*

```rust []
    fn main() {
        let a = 4;
        match a % 3 {
            0 => { println!("divisible by 3") },
            _ => { println!("not divisible by 3") },
        }
    }
```

## Doing a `match` on an `enum`

* When an `enum` has variants, you use `match` to extract the data
* New variables are created from the *pattern* (e.g. `radius`)

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

* There are two variables called `radius`
* The later one hides the earlier one

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

* You can use the `|` operator to join patterns together

```rust [1-16|9]
enum Shape {
    Circle(i32),
    Square(i32),
    Triangle
}

fn test_shape(shape: &Shape) {
    match shape {
        Shape::Circle(size) | Shape::Square(size) => {
            println!("Shape has size {}", size);
        },
        _ => {
            println!("Must be a triangle");
        },
    }
}
```

## Shorthand: `if let` conditionals

* You can use `if let` if only one case is of interest.
* Still *pattern matching*

```rust []
enum Shape {
    Circle(i32),
    Triangle
}

fn test_shape(shape: &Shape) {
    if let Shape::Circle(size) = shape {
        println!("Circle has size {}", size);
    }
}
```

## Shorthand: `let else` conditionals

* If you expect it to match, but want to handle the error...
* The `else` block must *diverge*

```rust []
enum Shape {
    Circle(i32),
    Triangle
}

fn test_shape(shape: &Shape) {
    let Shape::Circle(size) = shape else {
        println!("I only like circles");
        return;
    };
}
```

## Shorthand: `while let` conditionals

* Keep looping whilst the pattern still matches

```rust []
fn main() {
    let mut numbers = vec![1, 2, 3];
    while let Some(num) = numbers.pop() {
        println!("popped number {}", num);
    }
}
```

## `for` loops

* `for` is used for iteration
* Here `0..10` creates a `Range`, which you can iterate

```rust []
fn main() {
    for num in 0..10 {
        println!("{}", num);
    }
}
```

## `for` loops

Lots of things are *iterable*

```rust []
fn main() {
    for ch in "Hello".chars() {
        println!("{}", ch);
    }
}
```

## `for` under the hood

What Rust actually does is more like...

```rust []
fn main() {
    let mut iter = "Hello".chars().into_iter();
    loop {
        let Some(ch) = iter.next() else {
            break;
        };
        println!("{}", ch);
    }
}
```

## Break labels

If you have nested loops, you can label them to indicate which one you want to break out of.

```rust []
fn main() {
    'rows: for x in 0..5 {
        'cols: for y in 0..5 {
            println!("x = {}, y = {}", x, y);
            if x + y >= 6 {
                break 'rows;
            }
        }
    }
}
```

## Continue

Means go around the loop again, rather than break out of the loop

```rust []
fn main() {
    'rows: for x in 0..5 {
        'cols: for y in 0..5 {
            println!("x = {}, y = {}", x, y);
            if x + y >= 4 {
                continue 'rows;
            }
        }
    }
}
```

## `return`

-   `return` can be used for early returns
-   The result of the last expression of a function is always returned

```rust []
fn get_number(x: bool) -> i32 {
    if x {
        return 42;
    }
    -1
}
```

## `?` (early return operator)

The `?` operator means "on error, early return, with automatic conversion"

```rust []
use std::io::{self, prelude::*};

fn read_file(path: &std::path::Path) -> Result<String, io::Error> {
    let mut f = std::fs::File::open(path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
```
