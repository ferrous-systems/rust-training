# Summary

[Start Here](./start_here.md)

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

* [Methods and Traits](./methods-traits.md)
* [Rust I/O Traits](./io.md)
* [Generics](./generics.md)
* [Lifetimes](./lifetimes.md)
* [Cargo Workspaces](./cargo-workspaces.md)
* [Heap Allocation (Box and Rc)](./heap.md)
* [Shared Mutability (Cell, RefCell)](./shared-mutability.md)
* [Thread Safety (Send/Sync, Arc, Mutex)](./thread-safety.md)
* [Closures and the Fn/FnOnce/FnMut traits](./closures.md)
* [Spawning Threads and Scoped Threads](./spawning-threads.md)

# Advanced Rust

Topics that go beyond [Applied Rust](#applied-rust).

* [Advanced Strings](./advanced-strings.md)
* [Debugging Rust](./debugging-rust.md)
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
* [Using Types to encode State (TBC)](./type-state.md)
* [WASM](./wasm.md)

# No-Std Rust

Rust for the Linux Kernel and other no-std environments with an pre-existing C API. Requires [Applied Rust](#applied-rust).

* [Overview of no-std Rust (TBC)](./rust-no-std.md)
* [Unsafe Rust](./unsafe.md)
* [Foreign Function Interface](./ffi.md)
* [Working with Nightly](./working-with-nighly.md)
* [Rust in the Linux Kernel (TBC)](./rust-linux-kernel.md)
* [Rust on an RTOS (TBC)](./rust-rtos.md)
* [Writing a new target (TBC)](./custom-target.md)

# Bare-Metal Rust

Topics about using Rust on ARM Cortex-M Microcontrollers (and similar). Requires [Low-Level Rust](#low-level-rust).

* [Overview of Bare-Metal Rust (TBC)](./rust-bare-metal.md)
* [Booting a Cortex-M Microcontroller (TBC)](./cortex-m-booting.md)
* [Execptions and Interrupts on a Cortex-M Microcontroller (TBC)](./cortex-m-exceptions-interrupts.md)
* [PACs and svd2rust (TBC)](./pac-svd2rust.md)
* [The Embedded HAL and its implementations (TBC)](./embedded-hals.md)
* [Using RTIC v1 (TBC)](./rtic-v1.md)

# Ferrocene

Topics around [Ferrocene](https://ferrous-systems.com/ferrocene/), the qualified toolchain for writing safety-critical systems in Rust.

* [Installing and Using Ferrocene (TBC)](./installing-using-ferrocene.md)
