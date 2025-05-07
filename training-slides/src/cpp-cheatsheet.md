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

Taking something by-value in Rust by default means **moving** the value, not copying.
In comparison, copying is usually explicit with `.clone()` and references are explicit with `&` and `&mut`.

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

### Copy trait

The `Copy` trait changes Rust's default semantics back.

If a type implements `Copy`, it does not use move semantics, but copy semantics for assignment, passing by value, etc.
`Copy` types behave very closely to the C++ defaults, but without the ability to be moved (i.e. similar to pre-C++11).

`Copy` is usually only used for plain-old-data types that are cheap to copy.

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

Most algorithms are implemented directly on the Iterator trait, not as separate functions.

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
They are not included as in-place text.

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

Traits like `PartialEq`, `PartialOrd`, etc. are Rust's way of operator overloading.
If a type implements the right trait, the corresponding operator (e.g. `==`, `<`, etc.) is available for the type.

`#[derive(...)]`, implements traits automatically (comparable to `= default` in C++).
You can also implement them manually.

See the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module for details.


# Applied Rust
## Methods and Traits

A note on terminology: Rust uses "methods" where C++ developers would usually say "member functions".
Both concepts are similar, many Rust developers will know what is meant by "member function".

In Rust, "static member functions" are called "associated functions".

### Method Receivers

Instead of class member functions, Rust declares static/non-static methods inside `impl T` blocks:

Mappings from "member functions" to "methods"/"associated functions":

| C++                                                            | Rust                      |                                                                                   |
|----------------------------------------------------------------|---------------------------|-----------------------------------------------------------------------------------|
| `void my_fun() const;`<br/> or: `void my_fun() const &;`        | `fn my_fun(&self) {}`     |                                                                                   |
| `void my_fun();`<br/> or: `void my_fun() &;`                    | `fn my_fun(&mut self) {}` |                                                                                   |
| *No direct equivalent!*<br/> Closest: `void my_fun() &&;` | `fn my_fun(self) {}`      | Calling a `self`<br/> method consumes<br/> the value, it is<br/> no longer available. |
| `static void my_fun();`                                         | `fn my_fun() {}`          |                                                                                   |

Note: Instead of constructors, use associated functions that return `-> Self`.

Note: Methods can also be implemented on `enum` and `union` types, not just `struct`.


## Name Resolution inside methods

Rust uses `self` instead of `this`.
The type of `self` depends on the declaration, and is usually a reference, not a pointer.

Inside method you must `self.` to access other methods/members explicitly.
Unlike C++, members/methods are not callable implicitly.

So the C++ member function `area`:

```cpp
struct Square {
    float width() const { return m_width; }

    float area() const {
        return width() * width();
    }

    float m_width;
}
```

becomes:

```rust
struct Square {
    width: f64
}

impl Square {
    fn width(&self) -> f64 { self.width }

    fn area(&self) -> f64 {
        // Note: self is a &Square, not a *const Square
        self.width() * self.width()
    }
}
```

Rust differentiates between `self.width` (the member) and `self.width()` (the method) and `width` (parameter/variable).
The name resolution only searches for methods/functions if the item is called with `()` and members/variables in the other cases.

**Takeaway: No need to prefix members with `m_` or similar!**
Prefixing members is considered bad practice in Rust.

Advanced note: If a member is itself a callable function, force member resolution first, by enclosing the member access in parentheses:

```rust,ignore
(self.callable_member)();
```

### Interfaces without Inheritance

Rust is not a purely object-oriented language - only some object-oriented concepts are supported.
Specifically, Rust does not support inheritance!

Then how to build abstractions in Rust?

**Use composition and interfaces instead.**

Compound types like Structs/enums take care of the composition part of the equation.

**Traits represent the interface part**.

### Traits as interfaces

Traits are Rust's way of declaring interfaces - they are (very roughly) comparable to (abstract) base classes without members.

Key differences:

* Traits don't describe an "is a" relationship, but a "supports" relationship
    * e.g.: `String` "supports" `Format`
* Traits do not change members/memory layout of their implementors
* By default: No `virtual` dispatch
    * Rust usually prefers generics over dynamic dispatch
    * Dynamic dispatch is opt-in by the trait user with `dyn`
* Traits can be implemented on any type, not just `struct` types
    * Even reference/pointer types like `&SomeType`, `*const SomeType`, etc.

### Using Traits statically

Static trait dispatch is roughly equivalent to C++ templates with concepts.
Think of `impl Trait` as a concept that matches any type that implements `Trait`.

"Monomorphisation" is Rust speak for "template instantiation".

### Using Traits dynamically

Rust `dyn` vs. C++ `virtual`.

* Both use vtables for dynamic dispatch
* Key differences
    * `dyn` is specified at the usage site, not the trait implementation
    * `dyn` applies to the whole trait, not per-function
    * vtable is stored in the pointer itself (`&dyn Trait`), not the struct type
* Takeaways
    * Rust prefers static dispatch
    * Typically faster at runtime - can inflate binary size
    * Rust allows mixing static & dynamic dispatch depending on the usage

## Rust I/O Traits

Rust separates between buffered and unbuffered I/O.

`Read`/`Write` take care of the underlying unbuffered I/O.
`BufReader`/`BufWriter` can wrap any type that implements `Read`/`Write` and themselves also implement `Read`/`Write`.

In a sense the `Read`/`Write` define a basic interface, similar to C++ `std::istream`/`std::ostream` that other types implement.

## Generics

Generics are basically C++ templates, but without the confusing pitfalls and terrible error messages (or at least a lot fewer of them).

For those reasons they are used in Rust widely, especially over dynamic dispatch with `dyn`.

### Type Inference

The Rust compiler is a lot smarter about type inference than the C++ compiler.
In many cases, explicit type annotations are not needed, which can sometimes seem like magic.

To demystify this, it's important to know that Rust type inference can work "backwards" and only needs the missing bit of information.
E.g. Rust can detect the type on a return value by going backwards from where the type is used to where it is created.
Anything that can be inferred automatically can be left out of the type declaration with `_`.

Example:
```rust
let numbers = vec![1, 2, 3, 4];

// No need to supply the item type of the `Vec`, leave out with `_`
let odds: Vec<_> = numbers
    .into_iter()
    .filter(|num| *num % 2 != 0)
    // `.collect()` on this iterator can return anything that implements `FromIterator<i32>`.
    // Rust determines to use `Vec<i32>` because the result is assigned to `odds`,
    // which must be a `Vec` of something.
    // Because only `Vec<i32>` implements `FromIterator<i32>`, it must be `Vec<i32>`.
    .collect();

assert_eq!(odds, vec![1,3]);
```

### Adding Bounds

Rust trait bounds are roughly comparable to C++20 concepts.
They require a type to implement the given traits to be used with the generic.

Important difference: The **generic can only access functions/items that are declared in the bounds!**
The compiler checks the generic in isolation, **not for each specialization individually!**

C++ template type-checking:

1. Check any concepts
2. Insert the type into the template
3. Type check (may still fail)

Rust generic type check:

1. Type check the generic with the given bounds
2. Check that the concrete type actually implements the bounds
3. Insert the type into the template (can no longer fail)

This re-ordering means error messages are much cleaner, as the generic itself is checked for correctness, not every concrete instantiation.

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
