# Documentation

## `rustdoc`

Rust provides a standard documentation tool called `rustdoc`. It is commonly used through `cargo doc`.

Because of this Rust code is almost always documented in a common format.

## `std` Documentation

The standard library documentation is hosted at https://doc.rust-lang.org/std/.

A local, offline version can be opened with:

```console
rustup doc --std
```

## Crate Documentation

Documentation for crates hosted on http://crates.io/ can be found at https://docs.rs/.

Some crates may also have other documentation found via the "Documentation" link on their listing in http://crates.io/.

## Example: A Module

<https://doc.rust-lang.org/std/vec>

This page documents the `vec` module.

It starts with some examples, then lists any `struct`s, traits, or functions the module exports.

## How is it Generated?

`rustdoc` can read Rust code and Markdown documents.

`//!` and `///` comments are read as Markdown.

```rust []
//! Module documentation. (e.g. the 'Examples' part of `std::vec`).

/// Document functions, structs, traits and values.
/// This documents a function.
fn function_with_documentation() {}

// This comment will not be shown as documentation.
// The function itself will be.
fn function_without_documentation() {}
```

## Example: Components

<https://doc.rust-lang.org/std/string/#structs>

## Example: Functions

<https://doc.rust-lang.org/std/string/struct.String.html#method.new>

## Code Examples

By default code blocks in documentation are tested.

```rust
/// ```rust
/// assert_eq!(always_true(), true)
/// ```
fn always_true() -> bool { true }
```

## No-Run Examples

This code is marked 'do not run', as it doesn't terminate.

```rust
/// ```rust,no_run
/// serve();
/// ```
fn serve() -> ! { loop {} }
```

## Navigation

The arguments and return types of functions are links to their respective types.

The sidebar on the left offers quick navigate to other parts of the module.

## Cargo integration

This command builds and opens the docs to your current project:

```sh
cargo doc --open
```

---

Normally only `pub` items are documented. You can change this:

```sh
cargo doc --document-private-items --open
```
