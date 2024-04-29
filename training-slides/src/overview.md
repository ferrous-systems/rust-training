# Overview

---

```rust []
fn main() {
    let random_number = generate_random_number();
    let mut my_choice = 10;
    my_choice += random_number;
    println!("{my_choice}");
}

fn generate_random_number() -> i32 {
    4 // chosen by dice roll, guaranteed to be random
}
```

## What is Rust?

Rust is an empathic systems programming language that is determined to not let you shoot yourself in the foot.

## A Little Bit of History

-   Rust began around 2008
-   An experimental project by Graydon Hoare
-   Adopted by Mozilla
-   Presented to the general public as version 0.4 in 2012
-   Looked a bit Go-like back then

## Focus

-   Rust has lost many features from 2012 to 2014
    -   Garbage collector
    -   evented runtime
    -   complex error handling
-   Orientation towards a usable systems programming language

## Development

-   Always together with a larger project (e.g. Servo)
-   Early adoption of regular releases, deprecations and an RFC process

## Release Method

-   Nightly releases
-   experimental features are only present on nightly releases
-   Every 6 weeks, the current nightly is promoted to beta
-   After 6 weeks of testing, beta becomes stable
-   Guaranteed backwards-compatibility
-   Makes small iterations easier

Note:

- Cargo's "stabilization" section https://doc.crates.io/contrib/process/unstable.html#stabilization
- Crater tool
- Editions

## Goals

-   Explicit over implicit
-   Predictable runtime behaviour
-   Supporting stable software development for programming at large
-   Pragmatism and easy integration
-   Approachable project

Many examples in this course are very small, which is why we will also
spend time discussing the impact of many features on large projects.

## The Three Words

-   Safety
-   Performance
-   Productivity

## Safety

-   Rust is memory-safe and thread-safe
    - Buffer overflows, use-after-free: all impossible
    - Unless you tell the compiler you know what you're doing
-   De-allocation is automated
    - Great for files, mutexes, sockets, etc

## Performance

-   These properties are guaranteed at compile time and have no runtime
    cost!
-   Optimizing compiler based on LLVM
-   Features with runtime cost are explicit and hard to activate "by
    accident"
-   Zero-cost abstractions
-   Use threads with *confidence*

## Productive

-   User-focused tooling
-   Comes with a build-system, dependency manager, formatter, etc
-   Compiler gives helpful error messages
-   FFI support to interface with existing systems

## Where do Rustaceans come from?

From diverse backgrounds:

-   Dynamic languages (JS, Rubyists and Pythonistas)
-   Functional languages like Haskell and Scala
-   C/C++
-   Safety critical systems
