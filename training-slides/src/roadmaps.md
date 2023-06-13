# Roadmaps

Your trainer will guide you on your journey through the World of Rust. This journey is planned and executed by your trainer, according to their experiences and the needs of your group.

What follows here is the "default roadmap" for each *Training*. You may follow this 

## Module: Rust Fundamentals

### Day 1, Session 1: Introduction

* [Overview](./overview.md)
* Hello World

### Day 1, Session 2: Installation and Usage

* [Installation](./installation.md)
* Look at the Rust Docs

### Day 1, Session 3: Types, Functions and Control Flow

* [Basic Types](./basic-types.md)
* [Control Flow](./control-flow.md)
* Exercise: Fizz Buzz

### Day 2, Session 1: Compound Types & Pattern Matching

* [Compound Types](./compound-types.md)
* Exercise: Fizz Buzz with `match`

### Day 2, Session 2: Ownership

* [Ownership](./ownership.md)
* Exercise: Rustlings mini-exercises (link TBD)

### Day 2, Session 3: Error Handling

* [Error Handling](./error-handling.md)
* Exercise: File, Match and Result

### Day 3, Session 1: Collections

* [Collections](./collections.md)
* Exercise: TBD [how about an introduction to VecDequeue]

### Day 3, Session 2: Iterators

* [Iterators](./iterators.md)
* Exercise: Rust Latin

### Day 3, Session 3: Library Design

* [Imports and Modules](./imports-and-modules.md)
* [Good Design Practices](./good-design-practices.md)
* Exercise: Simple DB

## Module: Applied Rust

### Day 1, Session 1:

* [Methods and Traits](./methods-traits.md)
* Exercise: Shapes, Part I - adding methods to structs
* Learning Goals:
  * Writing methods inside an `impl` block
  * Talking about the difference between `&self`, `&mut self` and `self` (and when to use them)

### Day 1, Session 2:

* [Cargo Dependencies and Workspaces](./using-cargo.md)
* [Rust I/O Traits](./io.md)
* Exercise: Connected Mailbox
* Learning Goals:
  * Writing a struct which is generic over `T`
  * Writing a function which is generic over `T: where <some bound>`
  * Knowing when you need to `use SomeTrait;`

### Day 1, Session 3:

* [Generics](./generics.md)
* [Lifetimes](./lifetimes.md)
* Learning Goals:
  * Adding lifetime specifiers to fix the *needle in a haystack* function
  * Talking about the difference between a *Lifetime* and a *Lifetime Specifier*

### Day 2, Session 1:

* [Heap Allocation (Box and Rc)](./heap.md)
* [Shared Mutability (Cell, RefCell)](./shared-mutability.md)
* Learning Goals:
  * Describe the stack vs the heap
  * Use `Cell` to make a `Copy` type mutable though a shared reference
  * Use `RefCell` to make a non-`Copy` type mutable though a shared reference

### Day 2, Session 2:

* [Thread Safety (Send/Sync, Arc, Mutex)](./thread-safety.md)
* [Closures and the Fn/FnOnce/FnMut traits](./closures.md)
* Learning Goals:
  * Understand why `Send` and `Sync` exist
  * Write a closure which borrows from the environment

### Day 2, Session 3:

* [Spawning Threads and Scoped Threads](./spawning-threads.md)
* Exercise: Multi-threaded Mailbox
* Learning Goal:
  * Able to write a program that spawns threads

### Day 3:

* No scheduled content - the trainer will pick topics with the team
