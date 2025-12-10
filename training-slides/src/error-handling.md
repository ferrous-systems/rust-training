# Error Handling

## There are no exceptions

Rust has two ways of indicating errors:

* Returning a value
* Panicking

## Returning a value

```rust ignore
fn parse_header(data: &str) -> bool {
    if !data.starts_with("HEADER: ") {
        return false;
    }

    true
}
```

It would be nice if we could return *data* as well as *ok, or error*...

## Foretold enums strike back! 🤯

Remember these? They are *very important* in Rust.

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

<p>&nbsp<!-- run-button placeholder --></p>

For now, think of `T` and `E` as placeholders for your own types. These are also called
generics and there will be a dedicated chapter for them.

## I can't find it

If you have an function where one outcome is "can't find it", we use `Option`:

```rust
fn parse_header(data: &str) -> Option<&str> {
    if !data.starts_with("HEADER: ") {
        return None;
    }
    Some(&data[8..])
}
```

Note:

It's so important, it is special-cased within the compiler so you can say `None` instead of `Option::None`, as you would with any other enum.

## That's gone a bit wrong

When the result of a function is *either* __Ok__, or some __Error__ value, we use `Result`:

```rust []
enum MyError {
    BadHeader
}

// Need to describe both the Ok type and the Err type here:
fn parse_header(data: &str) -> Result<&str, MyError> {
    if !data.starts_with("HEADER: ") {
        return Err(MyError::BadHeader);
    }
    Ok(&data[8..])
}
```

Note:

It's so important, it is special-cased within the compiler so you can say `Ok` and `Err` instead of `Result::Ok` and `Result::Err`, as you would with any other enum.

## Handling Results by hand

You can handle `Result` like any other `enum`:

```rust
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(e);
    }
    Ok(contents)
}
```

## Handling Results with ?

It is idiomatic Rust to use `?` to let the caller handle errors while continuing
for the regular happy path.

```rust
use std::io::prelude::*;

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

Note:

This was added in Rust 1.39.

The ? operator will evaluate to the `Ok` value if the `Result` is `Ok`, and it will cause an early return with the error value if it is `Err`. It will also call `.into()` to perform a type conversion if necessary (and if possible).

The `?` operator allows exception like behaviour: Errors can be bubbled up, similar to exceptions.

## What kind of Error?

You can put anything in for the `E` in `Result<T, E>`:

```rust
fn literals() -> Result<(), &'static str> {
    Err("oh no")
}

fn strings() -> Result<(), String> {
    Err(String::from("oh no"))
}

fn enums() -> Result<(), Error> {
    Err(Error::BadThing)
}

enum Error { BadThing, OtherThing }
```

## Using String Literals as the Err Type

Setting `E` to be `&'static str` lets you use `"String literals"`

* It's cheap
* It's expressive
* But you can't change the text to include some specific value
* And your program can't tell what *kind* of error it was

## Using Strings as the Err Type

Setting `E` to be `String` lets you make up text at run-time:

* It's expressive
* You can render some values into the `String`
* But it costs you a heap allocation to store the bytes for the `String`
* And your program still can't tell what *kind* of error it was

## Using enums as the Err Type

An `enum` is ideal to express *one* of a number of different *kinds* of thing:

```rust
/// Represents the ways this module can fail
enum Error {
    /// An error came from the underlying transport
    Io,
    /// During an arithmetic operation a result was produced that could not be stored
    NumericOverflow,
    /// etc
    DiskFull,
    /// etc
    NetworkTimeout,
}
```

## Enum errors with extra context

An `enum` can also hold data for each variant:

```rust
/// Represents the ways this module can fail
enum Error {
    /// An error came from the underlying transport
    Io(std::io::Error),
    /// During an arithmetic operation a result was produced that could not
    /// be stored
    NumericOverflow,
    /// Ran out of disk space
    DiskFull,
    /// Remote system did not respond in time
    NetworkTimeout(std::time::Duration),
}
```

## The std::error::Error trait

* The Standard Library has a `trait` that your `enum Error` should implement
* However, it's not easy to use
* Many people didn't bother
* See <https://doc.rust-lang.org/std/error/trait.Error.html>

## Helper Crates

So, people created helper crates like [`thiserror`](https://crates.io/crates/thiserror)

```rust ignore []
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
```

## Something universal

Exhaustively listing all the ways your dependencies can fail is hard.

One solution:

```rust should_panic
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _f = std::fs::File::open("hello.txt")?; // IO Error
    let _s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```

## Anyhow

The [`anyhow`](https://crates.io/crates/anyhow) crate gives you a nicer type:

```rust ignore
fn main() -> Result<(), anyhow::Error> {
    let _f = std::fs::File::open("hello.txt")?; // IO Error
    let _s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```

Note:

* Use `anyhow` if you do not care what error type your function returns, just that it captures something.
  This oftentimes applies to applications.
* Use `thiserror` if you must design your own error types but want easy `Error` trait impl. This
  oftentimes applies to libraries.

## `Result` type conversions

```rust [|4-6|8-10]
enum ErrorX { Oops };
enum ErrorY { Oops };

fn convert_ok(input: Result<u32, ErrorX>) -> Result<u64, ErrorX> {
    input().map(|v| v as u64)
}

fn convert_err(input: Result<u32, ErrorX>) -> Result<u32, ErrorY> {
    result_u32().map_err(|e| ErrorY::Oops)
}
```

Note:

The `|..| ...` syntax is a *closure* - an anonymous inline function, where the function parameters are between the `|` symbols, and the function body follows. The parameter types and the return type are usually inferred automatically (but you can add them if required).

## Convert `Result` <-> `Option`

```rust [|3-5|7-9]
enum Error { Oops };

fn option_to_result(input: Option<u32>) -> Result<u32, Error> {
    option.ok_or(Error::Oops)
}

fn result_to_option(input: Result<u32, Error>) -> Option<u32> {
    result.ok()
}
```

## Replace `None` or `Err(e)` with a value

```rust [|3-5|7-9]
enum Error { Oops };

fn none_becomes_be_zero(input: Option<u32>) -> u32 {
    input.unwrap_or(0)
}

fn error_becomes_zero(input: Result<u32, Error>) -> u32 {
    input.unwrap_or(0)
}
```

## There is more

* `Result` and `Option` have a lot more methods available!
* [`Result` documentation](https://doc.rust-lang.org/std/result/enum.Result.html)
* [`Option` documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
* These methods can reduce a lot of boilerplate code, especially when
  combined with the `From` and `Into` value conversion traits.

Note:

* Example for combining this with `From` / `Into`: Mapping a child error into a parent error can
  be simply achieved by using `child_err.map_err(|e| e.into())` as long a `From<ChildError>` is
  implemented for `ParentError`

## Panicking

The other way to handle errors is to generate a controlled, program-ending, failure.

* You can `panic!("x too large ({})", x);`
* You can call an API that panics on error (like indexing, e.g. `s[99]`)
* You can convert a `Result::Err` into a panic with `.unwrap()` or `.expect("Oh no")`
