# Cpp Cheatsheet

# Rust Fundamentals

## Overview

* Writing Safe Rust gets you the following benefits instantly:
  * running UBSAN, ASAN, THREADSAN and RAII analysis at compile time, without the performance penalty at runtime
  * all bindings are `const`ed by default unless they specifically opt out with `let mut`
  * all code you depend on is also analyzed under the same constraints by the compiler

## Installation

* LLVM is the default codegen backend but the [experimental gcc](https://rust-gcc.github.io) has not yet stabilized
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a reliable debugger setup

## Basic Types

* Raw pointers do exist but are rarely used. Their typed variants with added semantics and safety are preferred, like `&`.
* Rust Strings do not have the Small String Optimization that C++ does
    * Try a drop in replacement like [smallstr](https://crates.io/crates/smallstr) instead

## Basic Types 2

* Rust's `println!` [semantics for non-numerics follow those](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/strings.html) of `sprintf`, but with `{}`:
    * `%-10s` to format a left aligned string padded to minimum 10 spaces becomes `{:<10}`
    * `%04` to pad a number with zeros up to a width of 4 becomes `{:04}`, etc.
* Rust does not have user defined literals so you need a macro to make `let duration = 5_milliseconds;` work in Rust.

## Basic Types 3

* A Rust `enum` is most similar to an `std::variant`
* `=` and `+=` like operators return the value that was set, whereas in Rust they do not.

## Control Flow

* This loop

```cpp
const auto v = {1,2,3,4};
for (const auto &: list) {
    //...
}
```

is equivalent to

```rust [], ignore
for value in &list {
    //...
}
```

## Compound Types
## Ownership and Borrowing
## Error Handling
## Collections
## Iterators
## Imports and Modules
## Good Design Practices
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
