# Cheatsheets

## Why

Teaching Rust can be hard, and it's harder when trainees we teach come from different programming language backgrounds.

A person in Python might have more questions about memory safety in general, since the GC allows them not to worry about that, but a person from C++ would be confused by the keyword `move` in a closure.

To alleviate this, we've created some cheatsheets of the form 

```
# Applied Rust

## Methods and Traits
...
## Rust I/O Traits
...
## Generics
...
```

per programming language that match each slide we have in our normal syllabus with an entry on the second header (`## Methods and Traits`, for example) level.

## How

As `training-material` grows and changes, maintenance could be a nightmare. We basically don't want to be in the business of remember that a certain slide got reordered or moved from one section to another and thus needs changing in all the cheathseets as well. Therefore, this tool seeks to alleviate that with the following workflow:

* call `cargo xtask make-cheatsheet python` at the root folder
* scrape Markdown headers in `SUMMARY.md` and segment topics by `Rust Fundamentals`, `Applied Rust` and `Advanced Rust`
* write out to `src/python-cheatsheet.md` if it doesn't exist
* if it does exist, check that it in sync: all headers in `python-cheatsheet.md` are in the appropriate sections, in order, and none are missing.

Specifically, `make-cheatsheet` and `test-cheatsheet` are defined in `xtask/src/tasks.rs` with utility functions to take our `SUMMARY.md`

```
# Rust Fundamentals
    * [Overview](./src/overview.md)
    * [Installation](.src/installation.md)
    * [Basic Types](./src/basic-types.md)
...
# Applied Rust
    * [Methods and Traits](./src/methods-and-traits.md)
    * [Rust I/O Traits](./src/rust-io-traits.md)
    * [Generics](./src/generics.md)
```

and convert it into a `Vec<SlidesSection>`:

```
    vec![SlideSection {header: "Rust Fundamentals",
            slide_titles: vec!["Overview", "Installation", "Basic Types"]},
         SlideSection {header: "Applied Rust",
            slide_titles: vec!["Methods and Traits", "Rust I/O Traits", "Generics"]}]
```

From there we can 

* create the cheatsheet for Python and have it written out to `training-slides/src/python-cheatsheet.md` by just iterating over `Vec<SlideSection>` and prefixing with the appropriate header level before printing 
* test that the cheathseet is in sync by scraping for all the lines that start with `#` in `python-cheatsheet.md` and check that they match, in order, those we scraped from `SUMMARY.md`.

