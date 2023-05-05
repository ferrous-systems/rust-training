# Good Design Practices

# Building Software can be hard

But certain standard practices can make your life easier.
Here are some

* packaging code into a reusable library
* writing and running unit tests
* reusing your own types across modules

## Basic building blocks

To achieve this goal, we'll use an established library design pattern of
* custom error handling through `enum`s
* using `Derive`s for access to easy functionality
* leaning on `rust-analyzer` to save us from papercuts

## The problem

We'll walk you through solving a small problem: building a library that can detect if a sentence is a [pangram](https://en.wikipedia.org/wiki/Pangram). A pangram string contains all letters in a given alphabet.

* `"The quick brown fox jumps over the lazy fox"` is a pangram

## Building a library

```console
cargo new --lib pangram
```

This will generate a new folder with a `src/lib.rs` file.

```text
$ tree pangram
pangram/
├── Cargo.toml
└── src
    └── lib.rs
```

## Pros and cons

* are designed to be reused by other crates
* have the same structure
* don't need to have a `main` function
* don't run with `cargo run`
* don't build an executable binary in `src/target`

## Writing the library

Let's try our first attempt at writing a pangram in `src/lib.rs`:

```rust [1|4|6]
use std::collections::HashSet;

fn is_pangram(input: &str) -> bool {
    let chars = input.chars().filter(|c| c.is_alphabetic());
    let char_set: HashSet<char> = HashSet::from_iter(chars);
    char_set.len() == 26    
}
```

Approach:

(We know it's flawed  - we're going to build it up!)

* Filter the `input` string for all `char`s that are letters
* from a `char`s iterable, make a `HashSet` - this data structure guarantees unique entries
* return `true` if the length of the set is 26, our desired alphabet length

Note:

* encourage trainees to have `rust-analyzer` auto-import `HashSet` by removing the import line
* remind students that `&str`s are "vectors with a hat on" and that to get out "characters" it's best to use `.chars()`, not raw indexing
* at this point, if people haven't pointed out glaring design flaws, suggest inputs to test baked in assumptions of single uppercase and the alphabet (English!) being 26 letters long.
* additionally, probe students about how they could use previous tooling to make this work over different alphabetic systems

## Writing unit tests

Rust's standard library and tooling allows us to write inline `test`s to check our code's functionality. Add this to your code in `src/lib.rs`:

```rust [9-12]
use std::collections::HashSet;

fn is_pangram(input: &str) -> bool {
    let chars = input.chars().filter(|c| c.is_alphabetic());
    let char_set: HashSet<char> = HashSet::from_iter(chars);
    char_set.len() == 26    
}

#[test]
fn first_test() {
    assert!(is_pangram("the quick brown fox jumps over the lazy dog"));
}
```

A typical testing setup needs
* a separate function with a `#[test]` macro on top
* an [`assert!`](https://doc.rust-lang.org/stable/std/macro.assert.html) or equivalent to check a boolean value at the end.
* mention that tests are compiled separate from the library and incur no runtime cost
* mention that tests are run in parallel by default

Try running these tests with

```console
cargo test
```

Note:

* Make sure to demo the `rust-analyzer` functionality to `Run Test` by clicking on the button above the test in their IDE

## Moar tests

It's useful to name our corner cases appropriately because they show up in `cargo test` in a more helpful manner.

```rust nocompile
#[test]
fn smallest_pangram() {
    assert!(is_pangram("abcdefghijklmnopqrstuvwxyz"));
}
#[test]
fn pangram_with_spaces() {
    assert!(is_pangram("the quick brown fox jumps over the lazy dog"));
}
#[test]
fn pangram_with_uppercase() {
    assert!(is_pangram("tHe quick brown fox jumps over the lazy dog"));
}
#[test]
fn pangram_with_uppercase_and_punctuation() {
    assert!(is_pangram("The quick brown fox jumps over the lazy dog."));
}
#[test]
fn pangram_in_spanish() {
    assert!(is_pangram("Un jugoso zumo de piña y kiwi bien frío es exquisito y no lleva alcohol"));
}
```

Running these tests results in

```text
running 5 tests
test pangram_with_spaces ... ok
test pangram_with_uppercase ... FAILED
test pangram_in_spanish ... FAILED
test smallest_pangram ... ok
test pangram_with_uppercase_and_punctuation ... FAILED
...
```

Clearly, we need to rethink our approach if we want to handle scale up our system to international Rust users!

We will do this by

* using enums to granulate our error handling
* implementing the `Error` trait for known corner cases

Note:

* mention `cargo run --quiet`

## Enums for Errors

