# Control Flow

## Control Flow primitives


-   `if`

-   `match` and `enums`

-   `for`, `while` and `loop` loops

-   `return` and `?`

## Control Flow with `if`


```rust,editable
    fn main() {
        if 1 == 2 { //  
            println!("unlikely");
        } else {
            println!("expected");
        }
    }
```
-   Paranthesis around the conditional are not necessary

-   Blocks need brackets, no shorthand

## Control Flow with `match`

```rust,editable
    fn main() {
        let a = 4;
        match a % 3 {
            0 => { println!("divisible by 3") }, // 
            _ => { println!("not divisible by 3") }, // 
        }
    }
```
-   match arm

-   default arm

## Control Flow with `match` and `enums`

```rust,editable
    enum Direction { //
        North(i32), //
        East(i32),
        South(i32),
        West(i32),
    }

    fn going_west(dir: &Direction) -> bool {
        match dir { //
            Direction::West(_) => true, //
            _ => false
        }
    }
```
-   `enum` can take multiple forms

-   The forms are called "variants" and can carry data

-   Enums are inspected by matching …

-   … on the structure

## 2 important enums


```rust,editable
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```
-   `Option` describes the possible absence of a value

-   `Result` describes that an operation might return an error instead

## Using `Option` and `Result`


```rust,editable
    fn main() {
        let will_overflow: Option<u8> = 10_u8.checked_add(250);
        match will_overflow {
            Some(sum) => println!("interesting: {}", sum),
            None => eprintln!("addition overflow!"),
        }
    }
```
## Using `Option` and `Result`


```rust,editable
    use std::fs::File;
    use std::io;

    fn main() {
        let file_open: Result<File, io::Error> = File::open("Does not exist");

        match file_open {
            Ok(f)  => println!("Success!"),
            Err(e) => println!("Open failed: {:?}", e),
        }
    }
```
## Match guards


```rust,editable
    fn main() {
        let result: Option<u8> = 5_u8.checked_add(5);

        match result {
            Some(result) if result % 2 == 0 => println!("5+5 is even!"),
            _ => println!("5+5 ... isn't even?"),
        }
    }
```
-   Match guards allow further refining of a `match`

## Combining matches


You can use the `|` operator to match several values in one arm.

```rust,editable
    enum Direction {
        North(u32),
        East(u32),
        South(u32),
        West(u32),
    }

    fn going_south_or_west(dir: &Direction) -> bool {
        match dir {
            Direction::West(_) | Direction::South(_) => true,
            _ => false,
        }
    }
```
## Shorthand: `if let` conditionals


```rust,editable
    fn main() {
        let maybe_arg = std::env::args().nth(2);
        // can't know at compile time how many args are passed to our program
        if let Some(arg) = maybe_arg {
            println!("Got second command line argument: {}", arg);
        }
    }
```
-   `if let` are idiomatic if only one case is of interest

## `loop`


```rust,editable
    fn main() {
        let mut i = 0;

        loop {
            i += 1;

            if i > 100 { break; }
        }
    }
```
`loop` is used for (potentially) infinite loops

## `for`


```rust,editable
    fn main() {
        let numbers = vec![1, 2, 3];
        // `for item in iterable` creates an iterator by calling `iterable.into_iter()`
        // and keeps calling `next() -> Option<Item>` on it until it receives `None`
        for num in numbers {
            println!("{}", num);
        }
    }
```
`for` is used for iteration

## `while`


```rust,editable
    fn main() {
        let mut i = 0;

        while !(i > 100) {
            i += 1;
        }

        let mut iter = vec![1,2,3].into_iter();

        while let Some(i) = iter.next() {
            println!("number: {}", i);
        }
    }
```
`while` is used for conditional loops

## `break`, `continue`


```rust,editable
    'outer: for i in 0..10 {
        loop {
            if i < 5 {
                continue 'outer;
            } else {
                break 'outer;
            }
        }
    }
```
terminate current iteration or entire loop, using optional labels if not
referring to innermost loop

## `return`


```rust,editable
    fn get_number() -> u32 {
        return 5;

        8
    }
```
-   `return` can be used for early returns

-   The result of the last expression of a function is always returned

## `?` (early return operator)


```rust,editable
    use std::io;
    use std::io::Read;

    fn read_file(path: &std::path::Path) -> Result<String, io::Error> {
         let mut f = std::fs::File::open(path)?;

         let mut buffer = String::new();
         f.read_to_string(&mut buffer)?;

         Ok(buffer)
    }
```
-   `?` is "on error, early return"
