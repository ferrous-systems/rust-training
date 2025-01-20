# Rust Fundamentals

## Overview

* Writing Safe Rust gets you the following benefits instantly:
  * running UBSAN, ASAN, THREADSAN and RAII analysis at compile time, without the performance penalty at runtime
  * all bindings are `const`ed by default unless they specifically opt out with `let mut`
  * all code you depend on is also analyzed under the same constraints by the compiler
* C++ copies variables by default, Rust transfers ownership (moves) by default. Custom types may opt into implicit `Copy` or explicit `Clone` copy semantics.

## Installation

* LLVM is the default codegen backend but the [experimental gcc](https://rust-gcc.github.io) has not yet stabilized
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a reliable debugger setup

## Basic Types

* Raw pointers do exist but are rarely used. Their typed variants with added semantics and safety are preferred, like `&`.
* Rust Strings do not have the Small String Optimization that C++ does, but you can use a drop in replacement like [smallstr](https://crates.io/crates/smallstr)
* Rust's `println!` [semantics for non-numerics follow those](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/strings.html) of `sprintf`, but with `{}`: `%-10s` to format a left aligned string padded to minimum 10 spaces becomes `{:<10}`, `%04` to pad a number with zeros up to a width of 4 becomes `{:04}`, etc.
* Rust does not have user defined literals so you need a macro to make `let duration = 5_milliseconds;` work in Rust.
* A Rust `enum` is most similar to an `std::variant`


## Control Flow

* `match` statements compile to an efficient jump table when the cases are statically known
* This loop

```cpp
const auto v = {1,2,3,4};
for (const auto &: list) {
    //...
}
```

is equivalent to

```rust
for value in &list {
    //...
}
```

## Compound Types

* Rust's `Option` is most similar to the C++ `std::Optional`.
* There's no default constructors in Rust.
* Destructors are handled through the `Drop` trait
* All `impl` blocks are `final` for Rust structs, so to speak, and so allow for aggressive devirtualization optimizations.

## Ownership and Borrowing

* Function calls do not copy values by default, so call by ref is not as prevalent in Rust.
* Rust does have Named Return Value Optimization and Named Value Optimization
* There's very little need to know of lvalue, rvalue or C++ move semantics in Safe Rust. Read more about unsafe Rust if need be.

## Error Handling

* Rust explicitly did not want to adopt exceptions because of the non-local logic it introduces

## Collections

* The [Entry API](https://doc.rust-lang.org/std/collections/index.html) is generic over many collections, a design inspired by the STL

* The following are roughly equivalent

| C++           | Rust                                                        |
|---------------|-------------------------------------------------------------|
| `std::vector` | `Vec` or `VecDeque`                                         |
| `std::list`   | `std::collections::LinkedList`                              |
| `std::set`    | `std::collections::HashSet` or `std::collections::BTreeSet` |
| `std::map`    | `std::collections::HashMap`                                 |

## Iterators

* C++ asks all iterators to be: copy constructible, copy assignable, destructible, provide a prefix `operator++` and an `operator*`: Rust only asks you to implement `.next()`.
* Iterators chains and their adapters are closest in spirit to the `ranges` proposal, but with added memory safety
* Prefer iterators to for loops, as indexing in Rust by default causes bounds checking

## Imports and Modules

* The Rust compiler has a bolted on [include-what-you-use](https://github.com/include-what-you-use/include-what-you-use) check at compile time
* Rust does not have SFINAE, as coherence (only a unique method for a given type instantiation is allowed) by the trait resolver

## Good Design Practices

* No testing code is included in released binaries, but modifying a unit test in a Rust file does cause you to recompile your binary

# Applied Rust

## Methods and Traits

* Rust binds functions to data with `impl` blocks but also allows free standing method definitions
* Traits bound generic code, and so are similar to C++'s concepts, but these fail earlier if it happens due to no SFINAE

## Rust I/O Traits

* Use a `BufReader`/`BufWriter` if you face performance issues when reading/writing files

## Generics

* Generic code is bounded at compile time with the `<T: Foo>` notation, where the given type `T` must implement the `Foo` trait
* Generics do not allow for var args in Rust, but those can be obtain through macros

## Lifetimes

* C++ actually has one place where it has more granular lifetime control than Rust: lambda captures. `[](move int x, int y){ x + y}` allows you to specify the moving capture of `x` but not `y`, wherease Rust only (for now) allows both or none.

## Cargo Workspaces

* It's much easier to add dependencies in Rust, via a single `cargo new foo@0.2.3` that interacts with the package manager, but the workplace resolver still needs to know of said workspace in the `Cargo.toml`.

## Heap Allocation (Box and Rc)

* Heap allocations usually are explicit in Rust, with `Foo::new` or `.clone()`. Those are useful to fish for later when reducing allocations.
* Because Rust can guarantee thread safety at compile time, `Rc<T>` in Rust does not need atomics. C++ had that in `shared_ptr`, but had to turn it off because it was too easy to misuse.

## Shared Mutability (Cell, RefCell, OnceCell)


## Thread Safety (Send/Sync, Arc, Mutex)

* How long a mutex in Rust is held for is exactly equivalent to when it is in scope, due to the `Drop` trait interataction with ownership rules.

## Closures and the Fn/FnOnce/FnMut traits

* Function pointers in Rust are typed more rigorously than C++.

## Spawning Threads and Scoped Threads

* Multithreading inside of `range` adapters in C++ can lead to data races, whereas such errors are impossible in Rust.

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
