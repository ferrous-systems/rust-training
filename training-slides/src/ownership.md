# Ownership and Borrowing

## Ownership

Ownership is the basis for the memory management of Rust.

## Rules

- Every value has exactly one owner
- Ownership can be passed on, both to functions and other types
- The owner is responsible for removing the data from memory
- The owner always has full control over the data and can mutate it

## These Rules are

- fundamental to Rust’s type system
- enforced at compile time
- important for optimisations

## Example

```rust [1-9|2|3|7-9|4]
fn main() {
    let s = String::from("Hello 😀");
    print_string(s);
    // s cannot be used any more - you gave it away
}

fn print_string(s: String) {
    println!("The string is {s}")
}
```

Note:

The statement `let s = ...;` introduces a *variable binding* called `s` and gives it a *value* which is of type `String`. This distinction is important when it comes to transferring ownership.

The function `String::from` is an associated function called `from` on the `String` type.

The `println!` call is a macro, which is how we are able to do to Python-style `{}` string interpolation.

## Does this compile?

```rust compile_fail [1-9|2|3|7-9|4]
fn main() {
    let s = String::from("Hello 😀");
    print_string(s);
    print_string(s);
}

fn print_string(s: String) {
    println!("The string is {s}")
}
```

## It does not...

```text
error[E0382]: use of moved value: `s`
 --> src/main.rs:4:18
  |
2 |     let s = String::from("Hello 😀");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     print_string(s);
  |                  - value moved here
4 |     print_string(s);
  |                  ^ value used here after move
  |
```

## Background

- When calling `print_string` with `s`, the value *in* `s` is *transferred* into the arguments of `print_string`.
- At that moment, ownership passes to `print_string`. We say the function *consumed* the value.
- The *variable binding* `s` ceases to exist, and thus `main` is not allowed to access it any more.

## Mutability

- The *variable binding* can be *immutable* (the default) or *mutable*.
- If you own it, you can rebind it and change this.

```rust
fn main() {
    let x = 6;
    // x += 1; ❌
    let mut x = x;
    x += 1; // ✅
}
```

## Borrowing

- Transferring ownership back and forth would get tiresome.
- We can let other functions *borrow* the values we own.
- The outcome of a *borrow* is a *reference*
- There are two kinds of *reference* - *Shared/Immutable* and *Exclusive/Mutable*

## Shared References

- Also called an *immutable reference*.
- Use the `&` operator to borrow (i.e. to make a reference).
- It's like a C pointer but with special compile-time checks.
- Rust also allows type-conversion functions to be called when you take a reference.

## Making a Reference

```rust []
fn main() {
    let s = String::from("Hello 😀");
    // A reference to a String
    let _string_ref: &String = &s;
    // The special string-slice type (could also be a reference
    // to a string literal)
    let _string_slice: &str = &s;
}
```

Note:

The `_` prefix just stops a warning about us not using the variable.

## Taking a Reference

- We can also say a function takes a reference
- We use a type like `&SomeType`:

```rust
fn print_string(s: &String) {
    println!("The string is {s}")
}
```

## Full Example

```rust
fn main() {
    let s = String::from("Hello 😀");
    print_string(&s);
    print_string(&s);
}

fn print_string(s: &String) {
    println!("The string is {s}")
}
```

## Exclusive References

- Also called a *mutable reference*
- Use the `&mut` operator to borrow (i.e. to make a reference)
- Even stricter rules than the `&` references
- Only a *mutable binding* can make a *mutable reference*

## Exclusive Reference Rules

- Must be only one exclusive reference to an object at any one time
- Cannot have shared and exclusive references alive at the same time
- => the compiler knows an `&mut` reference cannot alias anything

# Rust forbids *shared mutability*

## Making an Exclusive Reference

```rust []
fn main() {
    let mut s = String::from("Hello 😀");
    let s_ref = &mut s;
}
```

Note:

The binding for `s` now has to be mutable, otherwise we can't take a mutable reference to it.

## Taking an Exclusive Reference

- We can also say a function takes an exclusive reference
- We use a type like `&mut SomeType`:

```rust
fn add_excitement(s: &mut String) {
    s.push_str("!");
}
```

## Full Example 2

```rust []
fn main() {
    let mut s = String::from("Hello 😀");
    add_excitement(&mut s);
    println!("The string is {s}");
}

fn add_excitement(s: &mut String) {
    s.push_str("!");
}
```

Note:

Try adding more excitement by calling `add_excitement` multiple times.

## A Summary

|               | Borrowed            | Mutably Borrowed | Owned    |
| ------------- | ------------------- | ---------------- | -------- |
| Type `T`      | `&T`                | `&mut T`         | `T`      |
| Type `i32`    | `&i32`              | `&mut i32`       | `i32`    |
| Type `String` | `&String` or `&str` | `&mut String`    | `String` |

- *Mutably Borrowing* gives more permissions than *Borrowing*
- *Owning* gives more permissions than *Mutably Borrowing*

Note:

Why are there two types of Borrowed string types (`&String` and `&str`)? The first is a reference to a `struct` (`std::string::String`, specifically), and the latter is a built-in slice type which points at some bytes in memory which are valid UTF-8 encoded characters.

## An aside: Method Calls

- Rust supports *Method Calls*
- The first argument of the method is either `self`, `&self` or `&mut self`
- They are converted to function calls by the compiler

```rust []
fn main() {
    let mut s = String::from("Hello 😀");
    // This method call...
    s.push_str("!!");
    // is the same as...
    // String::push_str(&mut s, "!!");
    println!("The string is {s}");
}
```

Note:

We use `Type::function()` for associated functions, and `variable.method()` for method calls, which are just `Type::method(&variable)` or `Type::method(&mut variable)`, or `Type::method(variable)`, depending on how the method was declared).

## Avoiding Borrowing

If you want to give a function their own object, and keeps yours separate, you have two choices:

- Clone
- Copy

## Clone

Some types have a `.clone()` method.

It makes a new object, which looks just like the original object.

```rust []
fn main() {
    let s = String::from("Hello 😀");
    let mut s_clone = s.clone();
    s_clone.push_str("!!");
    println!("s = {s}");
    println!("s_clone = {s_clone}");
}
```

## Making things Cloneable

You can mark your `struct` or `enum` with `#[derive(Clone)]`

(But only if every value in your `struct`/`enum` itself is `Clone`)

```rust []
#[derive(Clone)]
struct Square {
    width: i32
}

fn main() {
    let sq = Square { width: 10 };
    let sq2 = sq.clone();
}
```

## Copy

- Some types, like integers and floats, are `Copy`
- Compiler copies these objects automatically
- If cloning is very cheap, you could make your type `Copy`

```rust []
fn main() {
    let x = 6;
    do_stuff(x);
    do_stuff(x);
}

fn do_stuff(x: i32) {
    println!("Do I own x, with value {x}?");
}
```

Note:

If your type represents ownership of something, like a `File`, or a `DatabaseRecord`, you probably don't want to make it `Copy`!

## Cleaning up

A value is cleaned up when its owner goes out of scope.

We call this *dropping* the value.

## Custom Cleaning

You can define a specific behaviour to happen on *drop* using the *Drop* trait.

For example, the memory used by a `String` is freed when dropped:

```rust []
fn main() {
    // String created here (some memory is allocated on the heap)
    let s = String::from("Hello 😀");
} // String `s` is dropped here and heap memory is freed
```

## More drop implementations:

- `MutexGuard` unlocks the appropriate `Mutex` when dropped
- `File` closes the file handle when dropped
- `TcpStream` closes the connection when dropped
- `Thread` detaches the thread when dropped
- etc...
