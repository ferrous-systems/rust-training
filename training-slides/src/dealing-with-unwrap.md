# Dealing with Unwrap

## Handling your errors

* Rust is *intentionally strict*: when failue modes happen, you have to decide how to handle them *right there*
* Recall:
  * `Option<T>` gives you information on if your operation produced something or nothing
  * `Result<T, E>` lets you know if something succeeded or something else (`E`) happened
* We can propagate the appropriate error context by transforming one into the other and vice versa

## Unwrap -> ?

* `.unwrap()`'ing both `Option` and `Result` *seems* like an easy way out
* Switching from `.unwrap()` calls often leads to changes in function signatures, and the refactoring becomes
wider and difficult with time and code

Instead, prefer using the early return `?` operator where possible, or at least `.expect()`

## `?` Examples

Let's see how we can get to `?` as quickly as possible in cases where

* You have many eager returns
* You have `match` statements where all cases must succeed to go forward

## `?` vs Eager: Before

`?` turns this

```rust [], ignore
fn write_info(info: &Info) -> io::Result<()> {
    // Early return on error
    let mut file = match File::create("my_best_friends.txt") {
           Err(e) => return Err(e),
           Ok(f) => f,
    };
    if let Err(e) = writeln!("name: {}", info.name) {
        return Err(e)
    }
    if let Err(e) = writeln!("age: {}", info.age) {
        return Err(e)
    }
    if let Err(e) = writeln!("rating: {}", info.rating) {
        return Err(e)
    }
    Ok(())
}
```

## `?` vs Eager Returns: After

Into this

```rust [], ignore
fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt")?;
    // Early return on error
    writeln!("name: {}", info.name)?;
    writeln("age: {}", info.age)?;
    writeln!("rating: {}", info.rating))?;
    Ok(())
}
```

<br>

## `?` vs Pattern Matching

As well as this

```rust []
fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    let a = stack.pop();
    let b = stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}
```

## `?` vs Pattern Matching 2

```rust []
fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?)
}
```

We prefer using `?` instead of highly nested pattern matching

## Option into Result: Before

* Sometimes we return `Option`, but we want a `Result` because it adds more context:

```rust [], ignore
struct UserId {
    name: String,
    num: u32,
}
fn find_user(username: &str) -> Option<&str> {
    let f = match std::fs::File::open("/etc/password") {
        // ...
    }
}
```

* We will use `or_else()` to change `Option<T>` into `Result<T, E>`

## Option into Result: After

```rust [], ignore
struct UserId {
    name: String,
    num: u32,
}
pub fn find_user(username: &str) -> Result<UserId, i32> {
    let f = std::fs::File::open("/etc/passwd")
        .or_else(|_| Err(0))?;
    Ok(UserId{name: "John".into(), num: 1})
}
```

* As applications grow, they tend to have a higher proportion of `Result`s rather than `Option`s

## Result to Result: Before

* We can process the context of the error to produce something more meaningful than `i32`
* Concretely: use `map_err()` on a `Result<_, A>` to get a `Result<_, B>`

```rust [], ignore
pub fn find_user(username: &str) -> Result<UserId, String>  {
    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| format!("Failed to open password file: {:?}", e))?;
    // ...
}
```


## Result to Result: After

* However, `String`y based errors are a code smell
* Prefer idiomatic error types that use `enum`s:

```rust [9], ignore
enum MyError {
    BadPassword(String),
    IncorrectID,
    // ...
}
impl std::error::Error for MyError {}
pub fn find_user(username: &str) -> Result<UserId, MyError>  {
    let f = std::fs::File::open("/etc/passwd")
        .map_err(|e| MyError::BadPassword(format!("Failed to open password file: {:?}", e)))?;
    // ...
}
```

## To be `?` or not to be `?`


* Using `?` means we deal with the error right now, but not *right here*
* Don't apply `?` blindly. There may be cases where other choices make sense
  * It's undesirable for long-running processes, or if we don't care about the failure
  * Handle the error instead instead of propagating it
  * Combine multiple `Result`s/`Option`s via pattern matching

## When to not `?`: Before

```rust [], ignore
for stream in tcp_listener.incoming() {
    // Should I use `stream?` here?
    // No, because my whole server would stop accepting connections
    let Ok(stream) = stream else {
        eprintln("Bad connection");
        continue;
    }
}
```

## When to not `?`: After

```rust [], ignore
if let (Ok(a), Ok(b)) = (job_a(), job_b()) {
    // run this code only when both jobs succeeded
}
```

* If you only care about moving on in the happy path, try judicious pattern matching with `if let`s

* *Note*: This throws away and ignores errors from `job_a()` and `job_b()`!

## Iterators: `Result` into `Option`

* Iterators usually just care about processing or finding certain elements and throwing out the uninteresting data
* Use `.filter_map()` for this:

```rust [], ignore
let a = ["1", "two", "NaN", "four", "5"];

// I don't care about bad results, I filter them out
let mut iter = a.iter()
    .filter_map(|s| s.parse::<i32>()
    .ok());
// Instead of
let mut iter = a.iter()
    .map(|s| s.parse())
    .filter(|s| s.is_ok())
    .map(|s| s.unwrap());
```

* Concretely, this means turning `Result<T, E>` into an `Option<T>` by using the `.ok()` method

## Iterators and collecting errors

* `Option` and `Result` support transposition: they can wrap collections or be elements of them
* If you want to process each error separately, use `Vec<Result<T, _>>`:

```rust [], ignore
let vec_of_results: Vec<Result<i32, _>> = inputs.iter()
    .map(|s| s.parse::<i32>())
    .collect();
```

## Iterators and collecting errors 2

* If you only care about all of them succeeding, you can `.collect()` them into a `Result<Vec<i32>, _>`:

```rust [], ignore
let result_of_vec: Result<Vec<i32>, _> = inputs.iter()
    .map(|s| s.parse::<i32>())
    .collect()?;
```

## Which way to wrap?

In general, we prefer wrapping the collection with an error (`Result<Vec<T>, _>` and `Option<Vec<T>>` )
rather than the other way around

## Recap

We've gone over many transformations:

* `Option<T>` to `Result<T, E>` and vice versa
* `Result<T, E>` to `Result<T, U>`

Many more variants exist depending on if you ignore the error, replace its value, provide a default, etc.

To deal with references, use `.as_ref()`.

## As Ref

It's arguably always better to pass `Option<&T>` than `&Option<T>` if `T` is immutable.

If your function accepts `Option<&T>` and you have `foo: &T`, you can pass in

*  `foo.as_ref()`, `foo`, `Some(&foo)` and `Some(foo)`

But if your function accepts `&Option<T>`, you can only accept

* `&foo` and `&Some(foo)`

which is more restrictive

## Useful References

* [Result's stdlib docs](https://doc.rust-lang.org/stable/std/result/index.html)
* [Option's stdlib docs](https://doc.rust-lang.org/stable/std/option/index.html)
* A **very useful** [diagram](https://docs.google.com/drawings/u/1/d/1EOPs0YTONo_FygWbuJGPfikO9Myt5HwtiFUHRuE1JVM/preview) is given in [the Effective Rust book](https://effective-rust.com/transform.html) for all these conversions and methods

## Conclusion

Worry about

* `Result<T, E>` <=> `Option<T>` and
* `Result<T, E>` <=> `Result<T, F>`

until you need something else
