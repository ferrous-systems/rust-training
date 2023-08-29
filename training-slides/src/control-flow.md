# Control Flow

## Control Flow primitives

-   `if` expressions
-   `loop` and `while` loops
-   `match` expressions
-   `for` loops
-   `break` and `continue`
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

-   `while` is used for conditional loops.
-   Loops while the boolean expression is `true`

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
            0 => { println!("divisible by 3") }
            _ => { println!("not divisible by 3") }
        }
    }
```

## Foreshadowing! Pattern Matching with bindings 👻

`match` is extremely powerful, we'll come back to it.

```rust ignore
enum Pets {
    Dog(String),
    Cat(String),
}

fn test_pet(pet: Pets) {
    match pet {
        Pets::Cat(cat_name) => println!("Cat name is {}", cat_name),
        Pets::Dog(dog_name) => println!("Dog name is {}", dog_name), 
        //          👆 New binding `dog_name` now exists for this match arm!
    }
}

```

- If a `match` arm's succesfully matches (destructures) a pattern, it introduces a new binding in that scope

## Foreshadowing! Pattern Matching with logic 👻

We'll see a more ergonomic form to handle a single case of interest and discard the rest

```rust ignore
let x = match pet {
    Pet::Dog(dog_name) => format!("My true love is {}", dog_name),
    _ => format!("I only care about dogs, sorry"),
}
```

## `for` loops

-   `for` is used for iteration
-   Here `0..10` creates a `Range`, which you can iterate

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

-    What Rust actually does is more like...
-    (More on this in the section on *Iterators*)

```rust []
fn main() {
    let mut iter = "Hello".chars().into_iter();
    loop {
        match iter.next() {
            Some(ch) => println!("{}", ch),
            None => break,
        }
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
