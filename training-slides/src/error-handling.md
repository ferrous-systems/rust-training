# Error Handling

## There are no exceptions

Rust has two ways of indicating errors:

* Returning a value
* Panicking

## Returning Values

When a function can *either* be __Ok__ or can return an __Error__, we use `Result<T, E>`:

```rust []
enum Error {}

fn calculate_sum(numbers: &[i32]) -> Result<i32, Error> {
    todo!();
}
```

*Why might this function fail?*

## What kind of Error?

You can put anything in for the `E` in `Result<T, E>`:

* A `&'static str`: `"Number overflow"`
* A `String`: `format!("Number {} too large", sum)`
* An `enum`: `Error::NumberOverflow(sum)`
* etc.

## Some magic happens

If you use `?` to return the error early, some extra conversion happens:

```rust [1-13, 1, 7, 2] 
fn main() -> Result<(), String> {
    let num = some_function(true)?;
    println!("num = {}", num);
    Ok(())
}

fn some_function(works: bool) -> Result<u32, &'static str> {
    if works {
        Ok(42)
    } else {
        Err("I'm not working today")
    }
}
```

## ? also called '.into()' for you

```rust [2-7]
fn main() -> Result<(), String> {
    let ch = match some_function(true) {
        Ok(ok_value) => ok_value,
        Err(error_value) => {
            return Err(error_value.into());
        }
    };
    println!("ch = {}", ch);
    Ok(())
}

fn some_function(works: bool) -> Result<u32, &'static str> { Ok(4) }
```

## Using String Literals

Setting `E` to be `&'static str` lets you use `"String literals"`

* It's cheap
* It's expressive
* But you can't change the string to include some specific value
* And your program can't tell what *kind* of error it was

## Using Strings

Setting `E` to be `String` lets you use heap-allocated strings:

* It's expressive
* You can include some specific values in the String
* But it costs you a heap allocation to store the bytes for the String
* And your program still can't tell what *kind* of error it was

## Using enums

An `enum` is ideal to express *one* of a number of differerent *kinds* of thing:

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

## Using enums

An `enum` can also hold data for each variant:

```rust
/// Represents the ways this module can fail
enum Error {
    /// An error came from the underlying transport
    Io(std::io::Error),
    /// During an arithmetic operation a result was produced that could not be stored
    NumericOverflow,
    /// etc
    DiskFull,
    /// etc
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

```rust [] ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

## Something universal

Exhaustively listing all the ways your dependencies can fail is hard.

One solution:

```rust [1-5|1|2|3] should_panic
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("hello.txt")?; // IO Error
    let s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```

## Anyhow

The [`anyhow`](https://crates.io/crates/anyhow) crate gives you a nicer type:

```rust [1-5|1] ignore
fn main() -> Result<(), anyhow::Error> {
    let f = std::fs::File::open("hello.txt")?; // IO Error
    let s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```
