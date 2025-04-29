# Cpp Cheatsheet

# Rust Fundamentals

## Overview

* In many ways, Rust is "Modern C++ best practices distilled into a new language"
    * Smart pointers & Move semantics by default - explicit copy construction
    * Everything is `const` by default, opt-out with `mut`
    * Value Semantics & Data-oriented programming over complex object graphs
* Writing Safe Rust gets you the following benefits instantly:
    * References are checked to be valid while in use
    * running UBSAN, ASAN, THREADSAN and RAII analysis at compile time, without the performance penalty at runtime
    * all code you depend on is also analyzed under the same constraints by the compiler

## Installation

* Cargo is the package manager, not the compiler
    * Like e.g. CMake, Cargo manages the compiler (rustc) and linker for you
    * Uses the system linker
    * Cargo does not have a separate "configure" stage like CMake
    * It's possible to use rustc directly, but very rarely needed
* rustc is the compiler
    * LLVM is the default codegen backend
    * [experimental gcc backend](https://rust-gcc.github.io) has not yet stabilized
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a reliable debugger setup
* Rust comes with an auto-formatter (rustfmt) that should be used by default
    * More consistent & reliable than e.g. clang-format
    * Most code in the Rust ecosystem uses rustfmt style

## Basic Types

### Integers

* No `int` type, use `i32` instead
* Use `usize` wherever you would use `std::size_t` in C++
* Integers in Rust cannot be used as booleans
    * Use explicit `if my_number != 0` instead of `if (my_number)`

### Strings and arrays

* `char` in Rust represents an actual Unicode Scalar Value (21-bit)
    * It cannot be used to represent plain "bytes" - use `u8`/`i8` instead
    * For C/C++ FFI, use `std::os::raw::c_char`
* Rust Strings and string slices
    * No nul-terminator! - Not compatible with C strings!
        * Use [`std::ffi::CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html)/[`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html) for C compatibility
    * String is not Small String optimized, like in C++
        * Try a drop in replacement like [smallstr](https://crates.io/crates/smallstr) instead
* Array types (`[T;n]`) are closer to `std::array` than to C-style arrays
    * Length is always known, includes bounds checking, etc.

### Miscellaneous

* Rust's `println!` [semantics for non-numerics follow those](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/strings.html) of `sprintf`, but with `{}`:
    * `%-10s` to format a left aligned string padded to minimum 10 spaces becomes `{:<10}`
    * `%04` to pad a number with zeros up to a width of 4 becomes `{:04}`, etc.
* Rust does not have user defined literals so you need a macro to make `let duration = 5_milliseconds;` work in Rust
* Raw pointers do exist but are rarely used
    * References or smart pointers are preferred for added semantics and safety

## Control Flow

### `if` statements

* No ternary operator in Rust
    * `cond ? a : b` becomes `if cond { a } else { b }`

### `match` vs `switch`

* `match` can match arbitrary types, not just integers
* No fall-through and no `break` in `match` statements
    * Use `"hello" | "world" => { ... }` to match multiple things to the same result
* `_` is equivalent to `default:`
* `match` is an expression like `if` - evaluates to the value of the match arm


### `do-while` in Rust

There is no `do {} while();` loop in Rust.
It can be approximated with `loop`.

```cpp
do {
    do_thing();
} while(condition());
```

becomes

```rust
# fn do_thing() {}
# fn condition() -> bool { false }
loop {
    do_thing();
    if (!condition()) {
        break;
    }
}
```

### For loops

This C++ loop

```cpp
const auto list = {1,2,3,4};
for (const auto &value: list) {
    //...
}
```

is equivalent to this Rust loop

```rust [], ignore
for value in &list {
    //...
}
```

## Compound Types

### Structs

No `class` type in Rust
* Use `struct` instead
* Only data members are declared inside the struct
* Member functions are declared outside the struct itself
* No inheritance, Rust uses composition and traits instead (foreshadowing 👻)

### Construction

Construction in Rust is similar to [aggregate initialization](https://en.cppreference.com/w/cpp/language/aggregate_initialization) with designated initializers.

There are no constructors in Rust, use "static" member functions (Rust calls them "associated functions") instead to uphold invariants before construction.

### Enums

A Rust `enum` is most similar to a `std::variant`, it can hold data in each of the variants.

* By default, the compiler chooses an optimal layout
    * Representation can be chosen explicitly with an attribute, e.g. `#[repr(u8)]`
    * Enum values can only be of the declared variants - not any integer
    * Cannot use enums as flags directly - use libraries like [`bitflags`](https://crates.io/crates/bitflags)

## Ownership and Borrowing

Like C++, Rust fundamentally has three ways to pass ownership around:

1. Moving the value
2. Copying the data into a new value
3. Handing out a reference to the value

The difference is in the defaults:

Taking something by-value in Rust always means **moving** the value, not copying.
In comparison, copying is explicit with `.clone()` and references are explicit with `&` and `&mut`.

The second important difference is that a move does not leave an object behind, when moving out of an object, the moved-from object is no longer accessible.

### References Cheat Sheet

|                                 | C++                      | Rust                               |
|---------------------------------|--------------------------|------------------------------------|
| Shared Reference Declaration    | `const std::string &arg` | `arg: &String`<br/>or: `arg: &str` |
| Shared Reference Passing        | `foo(arg);`              | `foo(&arg);`                       |
| Exclusive Reference Declaration | `std::string &arg`       | `arg: &mut String`                 |
| Exclusive Reference Passing     | `foo(arg);`              | `foo(&mut arg);`                   |

### Rules around References/Borrowing

Rust's references are similar to C++ references, but many rules/best-practices that C++ holds you responsible for are enforced at compile time:

- The referenced object must outlive the reference
- Only one exclusive (mutable) reference can exist at any given point in the program
- There can only be an exclusive reference, if there are no shared (`const`) references

In C++, these were already good to adhere to, in Rust they are mandatory - a safe Rust program will not compile otherwise.


### RAII and Drop

Rust uses very similar RAII rules to C++.
Instances that go out of scope are dropped (i.e. destructed).

The `Drop` trait acts like the destructor in C++, it can run code just before the instance is deleted.

Rust uses this to implement automatic clean up, similar to C++ (e.g. `String`, `Vec`, etc. also free their resources when they go out of scope).

## Error Handling

Think of all functions as `noexcept`, unless they return a `Result`.

They may still `panic!`, thereby aborting the program, similar to a `noexcept` function that throws an exception.
However, `panic!` should be used rarely and documented well in public API.

## Collections

**Rust/C++ equivalents of common collections**
| C++                       | Rust                              | Notes                                    |
|---------------------------|-----------------------------------|------------------------------------------|
| `std::array<T, n>`        | `[T;n]`                           |                                          |
| `std::span<const T>`      | `&[T]`                            |                                          |
| `std::span<T>`            | `&mut [T]`                        |                                          |
| `std::vector<T>`          | `Vec<T>`                          |                                          |
| `std::string_view`        | `&str`                            | Rust: UTF-8                              |
| `const char *`            | `&'static str`                    | String literals only<br/> Rust: UTF-8    |
| `std::string`             | `String`                          | Rust: UTF-8                              |
| `std::deque`              | `VecDeque`                        | Best match, slightly different internals |
| `std::unordered_map<K,V>` | `std::collections::HashMap<K,V>`  |                                          |
| `std::map<K,V>`           | `std::collections::BTreeMap<K,V>` | Best match, slightly different internals |
| `std::unordered_set<T>`   | `std::collections::HashSet<T>`    |                                          |
| `std::set<T>`             | `std::collections::BTreeSet<T>`   | Best match, slightly different internals |

## Iterators

Iterators in Rust are self-contained. No need for an end iterator.

Most algorithms are implemented directly on Iterator trait, not as separate functions.

So this C++ code:

```cpp
auto numbers = std::vector{ 1, 2, 3 };

auto odd = std::find_if(
                numbers.begin(),
                numbers.end(),
                [](const auto& number) { return number % 2 == 0; });

if (odd != numbers.end()) {
    std::cout << *odd << std::endl;
}
```

becomes:

```rust
let numbers = vec![1, 2, 3];

let odd = numbers
            .iter()
            .find(|number| *number % 2 == 0);

if let Some(odd) = odd {
    println!("{odd}");
}
```

## Imports and Modules

Rust modules work like C++ modules, not like `#include` files.
They export certain symbols (i.e. types/functions) under a given name.
They and are not included as in-place text.

Modules are also important for scoping in Rust.
Unlike namespaces, modules and all items in them have their own visibility (e.g. `pub` or not).

Inside a module, all items can access each other, but items are only accessible from outside the module if they are `pub`.
You can think of everything inside the same module as being a `friend` (as in the C++ keyword `friend`) of everything else inside the same module.

e.g. this works in Rust:

```rust
mod config {
    // private struct inside the module
    struct Config {
        // with private members
        color_enabled: bool,
        unicode_supported: bool,
    }

    // non-member function can still access the members inside the same module
    fn is_color_enabled(config: &Config) -> bool {
        config.color_enabled
    }
}

// But this would not work, as it's outside of `mod config`:
// fn is_unicode_supported(config: &Config) -> bool {
//     config.is_unicode_supported
// }
```

## Good Design Practices

### Operator Overloading with Traits

Traits like `PartialEq`, `PartialOrd`, etc. are Rusts way of operator overloading.
If a type implements the right trait, the operator (e.g. `==`, `<`, etc.) are available for the type.

`#[derive(...)]`, implements traits automatically (comparable to `= default` in C++).
You can also implement them manually.

See the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module for details.


# Applied Rust
## Methods and Traits
## Rust I/O Traits
## Generics
## Lifetimes
## Cargo Workspaces
## Heap Allocation (Box and Rc)
## Shared Mutability (Cell, RefCell, OnceCell)
## Thread Safety (Send/Sync, Arc, Mutex)
## Closures and the Fn/FnOnce/FnMut traits
## Spawning Threads and Scoped Threads

# Advanced Rust
## Advanced Strings
## Building Robust Programs with Kani
## Dealing with Unwrap
## Debugging Rust
## Deconstructing Send, Arc, and Mutex
## Dependency Management with Cargo
## Deref Coercions
## Design Patterns
## Documentation
## Drop, Panic and Abort
## Dynamic Dispatch
## Macros
## Property Testing
## Rust Projects Build Time
## Send and Sync
## Serde
## Testing
## The stdlib
## Using Cargo
## Using Types to encode State

# Rust and Web Assembly
## WASM
