# How CheatSheets Work

The cheatsheets provided in this section are a bridge to help programmers coming from other languages learn Rust.

Here's a brief example of how they work.

## Example usage

Let's say you want to create a cheatsheet for `MyLang`.

From the `src` directory, you type `cargo xtask make-cheatsheet mylang` where something like

```text
# MyLang Cheatsheet

# Rust Fundamentals

## Overview
## Basic Types
## Installation
...
```
will be produced in `rust-training/training-slides/src/mylang-cheatsheet.md`. 

Make sure to add that file under the `CheatSheets` section towards the bottom of `SUMMARY.md`.

## Example Usage 2

Notice that headers map to our syllabus under [rust-trianing/training-slides](https://rust-training.ferrous-systems.com/latest/slides/).

You *must* provide:

* An initial header of `# MyLang Cheatsheet`
* All the level 1 headers of our slide sections (`# Rust Fundamentals`, `# Applied Rust`, `# Advanced Rust` and `# No-Std Rust` for now), in order
* At least the slides that our syllabus covers as second level headers, (e.g., `## Overview`, `## Installation`, etc) but additional slide sections are allowed

## Good extra material

That is, this is allowed

```text
# MyLang Cheatsheet 

# Rust Fundamentals

## Overview
## Basic Types
## More Basic Types
## Basic Types Part 3
## Top 10 Myths MyLang Programmers Believe About Rust
## Installation
```

## Bad missing material

But this is not allowed

```text
# MyLang CheatSheet

# Rust Fundamentals

## Overview
## Basic Types
## MyLang Installation Specifics
```

Since the `## Installation` header is missing from the `# Rust Fundamentals` block.

## Bad missing header

Nor is this

```text
# MyLang CheatSheet

## Overview
## Basic Types
## Installation
```

Since the `# Rust Fundamentals` header is missing from the first block.

## Tooling

We have a tool that checks this compliance and you invoke it with

```console
cargo xtask test-cheatsheet mylang
```

It will panic as soon as one of these invariants is not met.

We mainly suggest you avoid lines starting with `# ` or `## ` in your cheatsheet as they will be picked up as headers and mess with parsing logic.

Note:

Which programming languages we support right now is a hardcoded number.

Adding a non-supported language requires some small additional logic to be handled when adding said cheatsheet under `xtask/main.rs`.

