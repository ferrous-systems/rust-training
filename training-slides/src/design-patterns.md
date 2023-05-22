# Basic Design Patterns

## `.clone()` before Lifetime Annotations

- As a beginner, use `.clone()` to overcome compiler struggle.
- It is alright! Refactor later.

## `String` before `&str`

- Use "owned" types before references.
- It is alright! Refactor later.

## String concatenation: Use `format!()`

- Owned type `String` can be generated easily.
- `let s: String = format!("No fear from {}", "Rust Strings")`

## Clippy is your friend in linting

- A collection of lints to catch common mistakes and improve your Rust code.
- Installation: `rustup component add clippy`
- Run: `cargo clippy`
- Documentation: https://rust-lang.github.io/rust-clippy/stable/index.html

## Pattern: `From<T>`, `Into<T>`

Conversion of one Type into another.

If `X` is `From<T>`, then `T` is `Into<X>` automatically.

The usage depends on the context.

## Pattern: `From<T>`, `Into<T>` - Example

```rust
fn main() {
    let string = String::from("string slice");
    let string2: String = "string slice".into();
}
```

## Pattern: What does `?` do?

```rust []
use std::fs::File;
use std::io::{self, Write};

enum MyError {
    FileWriteError,
}

impl From<io::Error> for MyError {
    fn from(e: io::Error) -> MyError {
        MyError::FileWriteError
    }
}

fn write_to_file_using_q() -> Result<(), MyError> {
    let mut file = File::create("my_best_friends.txt")?;
    file.write_all(b"This is a list of my best friends.")?;
    println!("I wrote to the file");
    Ok(())
}
// This is equivalent to:
fn write_to_file_using_match() -> Result<(), MyError> {
    let mut file = File::create("my_best_friends.txt")?;
    match file.write_all(b"This is a list of my best friends.") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }
    println!("I wrote to the file");
    Ok(())
}

fn main() {}
```

## Pattern: `AsRef<T>`

Reference-to-reference-conversion. Indicates that a type can easily produce references to another type.

## Pattern: `AsRef<T>` - Example

```rust []
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    open_file(&"test");
    let path_buf = PathBuf::from("test");
    open_file(&path_buf);
}

fn open_file<P: AsRef<Path>>(p: &P) {
    let path = p.as_ref();
    let file = File::open(path);
}
```

## Pattern: Constructor `new()`

- No constructors, but there is a convention.
- An associated function to construct new "instances".
- Use [`Default` trait](https://doc.rust-lang.org/stable/std/default/trait.Default.html). Try using `#[derive(Default)]` first.

```rust []
pub struct Stuff {
    value: i64,
}

impl Stuff {
    /// constructor by convention
    fn new(value: i64) -> Self {
        Self { value: value }
    }
}
```

## Pattern: NewType

- Use Rust type system to convey meaning to the user.
- Especially for Types that should be similar to other Types.
- Also used to `impl` external Traits on external Types

```rust []
struct MyString(String);

impl MyString {
    //... my implementations for MyString
}
```

## Pattern: Extending external Types

- Recall that at least one of Trait or Type should be local to `impl`.
- This pattern allows you to extend external Type using a local Trait.

```rust []
trait VecExt {
    fn magic_number(&self) -> usize;
}

impl<T> VecExt for Vec<T> {
    fn magic_number(&self) -> usize {
        42
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Magic Number = {}", v.magic_number());
}
```

## Pattern: Narrowing variable's scope

- Shadowing allows you to redefine a variable with `let` keyword again.
- Use it to get the inner Type, say in `Option`.
- Use it to your advantage to make variable immutable after it's served its purpose.

```rust [] ignore
// Get the inner type from Option
let array = [1, 2, 3, 4];
let item = array.get(1);
if let Some(item) = item { 
    println!("{:?}", item);
}

// Use shadowing to make the variable immutable outside of 
// where it needs to be mutable
let mut data = 42;
// change the data 
data += 1;
// Shadow using `let` again
let data = data; 
// data is immutable from now on
```
