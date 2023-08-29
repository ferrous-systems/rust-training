# Error Handling

## There are no exceptions

Rust has two ways of indicating errors:

* Returning a value
* Panicking

## Returning Values

When the result of a function is *either* __Ok__, or some __Error__ value, we use `Result<T, E>`:

```rust []
enum Error {}

fn calculate_sum(numbers: &[i32]) -> Result<i32, Error> {
    todo!();
}
```

*Why might this function fail?*

Note:
What happens if all the numbers add up to more than fits in an `i32`?

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

## Some magic happens

If you use `?` to return the error early, some extra conversion happens:

```rust [1-13|1|7|2]
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

## ? actually called .into() for you

```rust [2-7]
fn main() -> Result<(), String> {
    let num = match some_function(true) {
        Ok(ok_value) => ok_value,
        Err(error_value) => {
            return Err(error_value.into());
        }
    };
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

## Using String Literals

Setting `E` to be `&'static str` lets you use `"String literals"`

* It's cheap
* It's expressive
* But you can't change the text to include some specific value
* And your program can't tell what *kind* of error it was

## Using Strings

Setting `E` to be `String` lets you make up text at run-time:

* It's expressive
* You can render some values into the `String`
* But it costs you a heap allocation to store the bytes for the `String`
* And your program still can't tell what *kind* of error it was

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

## Very Important Enum #1 - Option

```rust
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let x = [1, 2, 3, 4];
    match x.get(5) {
        Some(value) => {
            println!("I got {value} from x.get(5)?");
        }
        None => {
            println!("I got None from x.get(5)");
        }
    }
}
```

Note:

It's so important, it is special-cased within the compiler so you can say `None` instead of `Option::None`, as you would with any other enum.

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

Note:

Also so important, it is special-cased within the compiler so you can say `Ok(...)` instead of `Result::Ok`, as you would with any other enum (except `Option`).

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

## Using enums to encode program state

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

## Adding data payloads to encoded enums

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
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
```

## Something universal

Exhaustively listing all the ways your dependencies can fail is hard.

One solution:

```rust [1-5|1|2|3] should_panic
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _f = std::fs::File::open("hello.txt")?; // IO Error
    let _s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```

## Anyhow

The [`anyhow`](https://crates.io/crates/anyhow) crate gives you a nicer type:

```rust [1-5|1] ignore
fn main() -> Result<(), anyhow::Error> {
    let _f = std::fs::File::open("hello.txt")?; // IO Error
    let _s = std::str::from_utf8(&[0xFF, 0x65])?; // Unicode conversion error
    Ok(())
}
```

Note:

* Use `anyhow` if you do not care what error type your function returns, just that it captures something.
* Use `thiserror` if you must design your own error types but want easy `Error` trait impl.
