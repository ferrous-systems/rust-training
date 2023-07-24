# Good Design Practices

## Two types of Rust crates

* binary - a program you can run directly
* library - a collection of useful code that you can re-use in a binary

## Binary crate

```shell
cargo new my_app
```
```text
my_app/
├── src/
│   └── main.rs
└── Cargo.toml
```

## Library crate

```shell
cargo new --lib my_library
```
```text
my_library/
├── src/
│   └── lib.rs
└── Cargo.toml
```

## How to *run* the code in a library?

Use tests!

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

## Testing

* mark your function with `#[test]`
* use `assert!`, `assert_eq!`, `assert_ne!` for assertions
    * `assert_eq!`, `assert_ne!` will show you the difference between left and right arguments
    * all assertions take an optional custom error message argument
* first failed assertion in a test function will stop the current test, other tests will still run
* `cargo test` will run all tests

## Assertions for your own types:

```rust ignore
struct Point(i32, i32);

let p = Point (1, 2);
assert_eq!(p, Point(1, 2));
```

Errors:
* "binary operation `==` cannot be applied to type `Point`"
    * can't compare two Points
* "`Point` doesn't implement `Debug`"
    * can't print out a Point in error messages

## Derives - adding behavior to your types

```rust
#[derive(Debug, PartialEq)]
struct Point(i32, i32);

let p = Point (1, 2);
assert_eq!(p, Point(1, 2));
```

## `Debug`

Allows printing of values with *debug formatting*

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
struct TuplePoint(i32, i32);

let p = Point { y: 2, x: 1 };
let tp = TuplePoint (1, 2);
println!("{:?}", p);
// Point { x: 1, y: 2 }
println!("{:?}", tp);
// TuplePoint (1, 2)
```

## `PartialEq`

* Allows checking for equality (`==` and `!=`)
* For complex types does a field-by-field comparison
* For references it compares data that references observe
* Can compare arrays and slices if their elements are `PartialEq`, too

## `PartialEq` and `Eq`

`Eq` means strict mathematical equality:
1. `a == a` should always be true
2. `a == b` means `b == a`
3. `a == b` and `b == c` means `a == c`

IEEE 754 floating point numbers (`f32` and `f64`) break the first rule (`NaN == NaN` is always false). They are `PartialEq` and not `Eq`.

## `PartialOrd` and `Ord`

* Same as `PartialEq` and `Eq`, but they also allow other comparisons (`<`, `<=`, `>=`, `>`).
* Generally, everything is `Ord`, except `f32` and `f64`.
* Characters are compared by their code point numerical values
* Arrays and slices are compared element by element. Length acts as a tiebreaker.
    * `"aaa" < "b"`, but `"aaa" > "a"`
    * elements themselves have to be `PartialOrd` or `Ord`

## How derives work?

* `Debug`, `PartialEq`, `Eq`, etc. are simultaneously names of "Traits" and names of "derive macros".
* If a trait has a corresponding derive macro it can be "derived":
    * Rust will generate a default implementation.
* Not all traits have a corresponding derive macros
    * these traits have to be implemented manually.

## `Debug` and `Display`

* a pair of traits.
* `Debug` is for debug printing
    * can be derived
* `Display` is for user-facing printing
    * cannot be derived, and must be implemented manually

```rust ignore
println!("{:?}", value); // uses `Debug`
println!("{:#?}", value); // uses `Debug` and pretty-prints structures
println!("{}", value); // uses `Display`
```

## Traits dependencies

Traits can depend on each other.

* `Eq` and `PartialOrd` both require `PartialEq`.
* `Ord` requires both `Eq` and `PartialOrd`

```rust ignore
#[derive(Debug, Ord)] // will give an error

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)] // Ok
```

## Other useful traits:

* `Hash` - a type can be used as a key for `HashMap`
* `Default` - a type gets a `default()` method to produce a default value
    * `0` is used for numbers, `""` for strings
    * collections starts as empty
    * `Option` fields will be `None`
* `Clone` adds a `clone()` method to produce a deep copy of a value

`derive` lists can get be pretty long.

## Documentation

* `///` marks doc comments
* Markdown
* Rust fragments in doc comments produce documentation tests
    * Use it to test you examples.
* Example from a standard library:
    * [`Vec::len()` docs page](https://doc.rust-lang.org/1.69.0/std/vec/struct.Vec.html#method.len)
    * [`Vec::len()` docs source code](https://doc.rust-lang.org/1.69.0/src/alloc/vec/mod.rs.html#2050)

## Formatting and Linting

`rustfmt` is a default Rust formatter
```shell
cargo fmt
```

`Clippy` is a linter for Rust code
```shell
cargo clippy
```
