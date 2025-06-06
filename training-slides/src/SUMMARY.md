# Front Matter

[Introduction](./introduction.md)

[Glossary](./glossary.md)

# Rust Fundamentals

* [Overview](./overview.md)
* [Installation](./installation.md)
* [Basic Types](./basic-types.md)
* [Control Flow](./control-flow.md)
* [Compound Types](./compound-types.md)
* [Ownership and Borrowing](./ownership.md)
* [Error Handling](./error-handling.md)
* [Collections](./collections.md)
* [Iterators](./iterators.md)
* [Imports and Modules](./imports-and-modules.md)
* [Good Design Practices](./good-design-practices.md)

# Applied Rust

Using Rust on Windows/macOS/Linux. Requires [Rust Fundamentals](#rust-fundamentals).

* [Cargo Workspaces](./cargo-workspaces.md)
* [Methods and Traits](./methods-traits.md)
* [Rust I/O Traits](./io.md)
* [Generics](./generics.md)
* [Lifetimes](./lifetimes.md)
* [Closures and the Fn/FnOnce/FnMut traits](./closures.md)
* [Heap Allocation (Box and Rc)](./heap.md)
* [Shared Mutability (Cell, RefCell, OnceCell)](./shared-mutability.md)
* [Thread Safety (Send/Sync, Arc, Mutex)](./thread-safety.md)
* [Spawning Threads and Scoped Threads](./spawning-threads.md)

# Advanced Rust

Topics that go beyond [Applied Rust](#applied-rust).

* [Advanced Strings](./advanced-strings.md)
* [Building Robust Programs with Kani](./kani.md)
* [Dealing with Unwrap](./dealing-with-unwrap.md)
* [Debugging Rust](./debugging-rust.md)
* [Deconstructing Send, Arc, and Mutex](./deconstructing-send-arc-mutex.md)
* [Dependency Management with Cargo](./dependency-management.md)
* [Deref Coercions](./deref-coercions.md)
* [Design Patterns](./design-patterns.md)
* [Documentation](./documentation.md)
* [Drop, Panic and Abort](./drop-panic-abort.md)
* [Dynamic Dispatch](./dynamic-dispatch.md)
* [Macros](./macros.md)
* [Property Testing](./property-testing.md)
* [Rust Projects Build Time](./rust-build-time.md)
* [Send and Sync](./send-and-sync.md)
* [Serde](./serde.md)
* [Testing](./testing.md)
* [The stdlib](./std-lib-tour.md)
* [Using Cargo](./using-cargo.md)
* [Using Types to encode State](./type-state.md)

# Rust and Web Assembly

* [WASM](./wasm.md)

# No-Std Rust

Rust for the Linux Kernel and other no-std environments with an pre-existing C API. Requires [Applied Rust](#applied-rust).

* [Unsafe Rust](./unsafe.md)
* [Foreign Function Interface](./ffi.md)
* [Working with Nightly](./working-with-nightly.md)
* [The Shape of a Rust Program](./shape-rust-program.md)

## Under development

* [Overview of no-std Rust]()
* [Rust in the Linux Kernel]()
* [Rust on an RTOS]()
* [Writing a new target]()

# Async Rust

* [Async Building Blocks](./async-building-blocks.md)
* [Intro to Tokio](./async-intro-to-tokio.md)
* [Async Implementation Details](./async-implementation-details.md)
* [Async Component Interaction](./async-component-interaction.md)
* [Async: Avoiding Disaster and Unbounded Growth](./async-avoiding-disaster.md)

# Bare-Metal Rust

Topics about using Rust on ARM Cortex-M Microcontrollers (and similar). Requires [Applied Rust](#applied-rust).

* [Overview of Bare-Metal Rust](./rust-bare-metal.md)
* [Booting a Cortex-M Microcontroller](./booting-cortex-m.md)
* [PACs and svd2rust](./pac-svd2rust.md)
* [Writing Drivers](./writing-drivers.md)
* [The Embedded HAL and its implementations](./embedded-hals.md)
* [Board Support Crates](./board-support.md)
* [Using defmt](./defmt.md)

## Under development

* [Exceptions and Interrupts on a Cortex-M Microcontroller]()
* [Using RTIC v1]()

# Ferrocene

Topics around [Ferrocene](https://ferrous-systems.com/ferrocene/), the qualified toolchain for writing safety-critical systems in Rust.

* [What is Ferrocene](./ferrocene-what-it-is.md)
* [Installing and Using Ferrocene](./ferrocene-installing.md)

# Why Rust?

* [What is Rust?](./what-is-rust.md)
* [Safety, Performance, and Productivity](./safety-performance-productivity.md)
* [Tradeoffs](./tradeoffs.md)
* [Where Next](./where-next.md)

# Cheatsheets

* [How CheatSheets Work](./how-cheatsheets-work.md)
* [C++](./cpp-cheatsheet.md)
