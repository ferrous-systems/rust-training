//! This file implements the `make-cheatsheets`, `test-cheetsheets` and, `test-all-cheatsheets` `xtasks` commands.
//! These commands offer us the ability to
//! 1. Make a cheatsheet for a predetermined language, coded in `main.rs` and which will mainly be c/c++/python/java
//! 2. Test that said cheatsheet does not fall out of sync with the rest of the `training-material`.
//!
//! How?
//! The approach is to chunk the appropriate info from a `SUMMARY.md` that could look like this:
//! ```
//!# Rust Fundamentals
//!
//!* [Overview](./overview.md)
//!* [Basic Types](./basic-types.md)
//!
//!# Applied Rust
//!
//! ...
//!```
//! A cheatsheet is then just as a collection of `Vec<SlideSections>`, where the `headers`
//! are strings like `# Rust Fundamentals` and the corresponding `slide_titles`
//! is a `vec!["Overview", "Basic Types"];`, for this example.
//!
//! That's the logic behind `make-cheatsheets` which is half the functionality here.
//!
//! It then gets crafted into a single markdown file `src/lang-cheatsheet.md` so that
//! we (trainers) can fill it out with the pertinent info for Python Basic Types,
//! Python Strings, etc.
//!
//! -----------
//!
//! The other half of the functionality is `test-cheatsheets`, which requires thinking
//! about what would make our `cheatsheets` fall out of sync.
//!
//! Assume we have a folder with `src/mylang-cheatsheets.md`.
//!
//! In theory, the cheatsheets and the files in `SUMMARY.md` need to have
//!
//! * matching of headers (in the right order, as we're bound to have slides migrate)
//! * at least as many slides as the source files
//!
//! We allow more slides than in the source files because we want to allow ourselves
//! to add extra info that doesn't make sense as part of the larger Rust training-materials.
//!
//! Any condition that fails will emit a message as to why it did, but we'll continue listing the other ones
//! to streamline UX.
//!
//! Lastly, `test-all-cheatsheets` is just a convenience function to test all existing cheatsheets at once.

use eyre::WrapErr;
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::LANG_LIST;

/// We ignore anything before this header.
///
/// We also verify that this header exists.
const INITIAL_HEADER: &str = "# Rust Fundamentals";

/// We ignore anything after this header
///
/// We also verify that this header exists.
const LAST_HEADER: &str = "# No-Std Rust";

/// How many headers we are processing.
///
/// We use this to split the cheatsheet into `NUM_HEADERS` headers
const NUM_HEADERS: usize = 4;

/// All the headers
const HEADERS: [&str; 4] = [
    INITIAL_HEADER,
    "# Applied Rust",
    "# Advanced Rust",
    "# Rust and Web Assembly",
];

/// Describes a section of the training material.
#[derive(Debug, Eq, PartialEq, Default)]
struct SlidesSection {
    /// The heading name from SUMMARY.md
    header: String,
    /// The individual slide deck names under that heading
    ///
    /// Note this is human-readable titles, not filenames.
    deck_titles: Vec<String>,
}

/// Parse the slide deck name from a Markdown URL.
///
/// Converts a string like "* [Overview](./overview.md)" into "Overview"
fn get_deck_title(line: &str) -> String {
    assert!(line.starts_with("* ["));
    assert!(line.ends_with(".md)"));
    // SAFETY
    // This line should be a well formed mdbook entry: `* [TEXT](./foo.md)`
    let top = line
        .rfind(']')
        .expect("the markdown file entry did not have a ']'");
    let bot = line
        .find('[')
        .expect("the markdown file entry did not have a '['");
    String::from(&line[bot + 1..top])
}

/// Pull a list of interesting headings and slide deck titles from a Markdown
/// blob.
///
/// There's lines we don't care about in `SUMMARY.md` for the purposes of our
/// cheatsheet.
///
/// Let's filter those out, and collect into a `Vec<Vec<String>>`.
///
/// The first entry in each `Vec<String>` is the heading. The rest are the deck
/// titles.
fn focus_regions(text: &str) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current_section: Vec<String> = Vec::new();

    // These are loud checks to see if we've migrated any of our Big Sections away from their conventional names.
    if !text.contains(INITIAL_HEADER) {
        panic!("Your INITIAL_HEADER ({INITIAL_HEADER:?}) is not part of the input. Check your `SUMMARY.md` for {INITIAL_HEADER}");
    }
    if !text.contains(LAST_HEADER) {
        panic!("Your LAST_HEADER ({LAST_HEADER:?}) is not part of the text input. CHECK your `SUMMARY.md` for {LAST_HEADER}");
    }

    let Some(first_header) = text.find(INITIAL_HEADER) else {
        panic!("Could not find the initial header {INITIAL_HEADER:?}. Check your `SUMMARY.md`.")
    };
    let Some(last_header) = text.rfind(LAST_HEADER) else {
        panic!("Could not find the last header {LAST_HEADER:?}. Check your `SUMMARY.md`.")
    };

    let text = &text[first_header..last_header];

    // Skip anything that doesn't start with '*' or '#', we don't care about them.
    for line in text.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty()
            || (!trimmed_line.starts_with('*') && !trimmed_line.starts_with('#'))
        {
            continue;
        }
        // We found a new header if it starts with "# ", so start a new section/Vec<String> we can slide names into
        if trimmed_line.starts_with("# ") && !current_section.is_empty() {
            result.push(current_section);
            current_section = Vec::new();
        }
        current_section.push(trimmed_line.to_string());
    }

    if !current_section.is_empty() {
        result.push(current_section);
    }

    result
}

/// Convert a list of strings into a [`SlidesSection`]
///
/// The first string is the heading and the test are slide deck titles
fn list_of_strings_to_slide_section(chunk: &[String]) -> SlidesSection {
    assert!(chunk.len() >= 2);
    // '# Rust Fundamentals' -> 'Rust Fundamentals'
    let Some(header) = chunk[0].strip_prefix("# ") else {
        panic!("Malformed header '{:?}'", chunk[0]);
    };

    let deck_titles = chunk[1..]
        .iter()
        .map(|l| get_deck_title(l))
        .collect::<Vec<String>>();

    SlidesSection {
        header: header.to_string(),
        deck_titles,
    }
}

/// Collect all the headers and write them out into a new `src/lang-cheatsheet.md`.
pub fn make_cheatsheet(lang: &str) -> Result<(), eyre::Report> {
    // Collect slide sections, chunked by header
    let text =
        read_to_string("./training-slides/src/SUMMARY.md").context("SUMMARY.md not found")?;
    let slide_texts = focus_regions(&text);
    let uppercase_lang_name = lang.chars().nth(0).unwrap().to_uppercase().to_string() + &lang[1..];
    let mut slide_sections = vec![SlidesSection {
        header: format!("{uppercase_lang_name} Cheatsheet"),
        deck_titles: vec![],
    }];
    slide_texts
        .iter()
        .for_each(|l| slide_sections.push(list_of_strings_to_slide_section(l)));

    // Check to see if a file exists
    let file_str = format!("./training-slides/src/{lang}-cheatsheet.md");
    let new_file = Path::new(&file_str);

    // If lang-cheatsheet.md exists, check if any headers are missing
    // Otherwise, create the lang-cheatsheet.md
    match File::create_new(new_file) {
        Ok(mut f) => {
            let result_text = render_cheatsheet(slide_sections);
            f.write_all(result_text.as_bytes())?;
            eprintln!("Cheatsheat for {lang} written at {file_str}");
            eprintln!("Make sure to add it to SUMMARY.md!")
        }
        Err(_) => {
            eprintln!("File {lang}-cheatsheet.md already exists - checking it's in sync");
            setup_cheatsheet_tester(lang)?;
        }
    }
    Ok(())
}

/// Verify that a cheatsheet has all the right headings in the right order.
/// This function was refactored out of `cheatsheet_tester` to separate the file handling logic
/// from the unit tests of `cheatsheet_tester`.
pub fn setup_cheatsheet_tester(lang: &str) -> Result<(), eyre::Report> {
    // Collect Vec<SlideSection> from SUMMARY.md
    let text = read_to_string("./training-slides/src/SUMMARY.md")
        .context("could not read_to_string - SUMMARY.md not found")?;
    let slide_texts = focus_regions(&text);
    let slide_sections: Vec<SlidesSection> = slide_texts
        .iter()
        .map(|l| list_of_strings_to_slide_section(l))
        .collect();

    let file_name = format!("./training-slides/src/{lang}-cheatsheet.md");
    let cheatsheet_text = read_to_string(file_name)
        .with_context(|| eyre::eyre!("{lang}-cheatsheet.md not found."))?;

    cheatsheet_tester(lang, slide_sections, &cheatsheet_text)
}

/// Test that the cheatsheet respects the following invariants:
/// * It starts with `# MyLang Cheatsheet`
/// * It contains all headers in `HEADERS`
/// * It contains all subheaders in each header
///
/// It does this by
/// 1. Collecting the headings and subheadings into a `Vec<String>`
/// 2. Checking that each heading has all its subheadings
/// 3. Checking all `HEADERS` are present
fn cheatsheet_tester(
    lang: &str,
    slide_sections: Vec<SlidesSection>,
    cheatsheet_text: &str,
) -> Result<(), eyre::Error> {
    // Unwrap note: We know `lang` is part of our blessed lang-names, so it must be at least of size 1.
    let lang_uppercase = lang.chars().nth(0).unwrap().to_uppercase().to_string() + &lang[1..];
    let cheatsheet_name = format!("# {lang_uppercase} Cheatsheet");
    if !cheatsheet_text.contains(&cheatsheet_name) {
        panic!("{lang}-cheatsheet.md does not contain a starting header `{cheatsheet_name}`");
    }

    // Collect Vec<String> from lang-cheatsheet.md into chunks
    let mut chunked_cheatsheet: Vec<Vec<String>> = Vec::new();
    let mut current_section: Vec<String> = Vec::new();
    let mut in_code_block = false;

    for line in cheatsheet_text.lines() {
        // Code blocks start and end with ```
        // Make sure to recognize these blocks so we can ignore anything inside
        if line.starts_with("```") {
            in_code_block = !in_code_block;
        }

        // Skip anything that doesn't start with '#', we don't care about them.
        // Also, skip the line if it's `# Mylang Cheatsheet` or inside a code block
        if line.is_empty() || !line.starts_with('#') || line == cheatsheet_name || in_code_block {
            continue;
        }
        // We found a new header if it starts with "# ", so start a new section/Vec<String> we can slide names into
        if line.starts_with("# ") && !current_section.is_empty() {
            chunked_cheatsheet.push(current_section);
            current_section = Vec::new();
        }
        current_section.push(line.to_string());
    }

    if !current_section.is_empty() {
        chunked_cheatsheet.push(current_section);
    }
    // End collection

    let _first_line = chunked_cheatsheet
        .first()
        .expect("Cheatsheet should not be empty");

    let mut missing_files = false;
    if chunked_cheatsheet.len() != NUM_HEADERS {
        eprintln!("You are missing headers in {lang}-cheatsheet.md");
        missing_files = true;
    }

    // Checking Logic
    for (idx, section) in chunked_cheatsheet.iter().enumerate() {
        // Check that headers from SUMMARY.md are in the lang-cheatsheet.md
        if HEADERS[idx] != section[0] {
            eprintln!(
                "Header Error: '{}' should be in {lang}-cheatsheet.md",
                HEADERS[idx]
            );
            missing_files = true;
        }

        // If there's only a header, just skip the next part
        if section.len() == 1 {
            eprintln!(
                "You are missing *ALL* the slide decks under the {} header",
                HEADERS[idx]
            );
            missing_files = true;
            continue;
        }
        // Check that slides from slide section are under correct header in lang-cheatsheet.md
        for deck in &slide_sections[idx].deck_titles {
            if !section[1..]
                .iter()
                .any(|l| *l == (String::from("## ") + deck))
            {
                eprintln!(
                    "Slide Section '{}' in {lang}-cheatsheet.md is not under header {}",
                    deck, HEADERS[idx]
                );
                missing_files = true;
            }
        }
    }

    if missing_files {
        panic!("You have missing slides in {lang}-cheatsheet.md");
    } else {
        eprintln!("Neat! {lang}-cheatsheet.md is in sync");
        Ok(())
    }
}

/// Test all the cheatsheets
pub fn test_all_cheatsheets() -> Result<(), eyre::Report> {
    for lang in LANG_LIST.iter() {
        let file_name = format!("./training-slides/src/{lang}-cheatsheet.md");
        let file_path = Path::new(&file_name);
        if Path::exists(file_path) {
            setup_cheatsheet_tester(lang)?;
        } else {
            continue;
        }
    }
    Ok(())
}

/// Format a cheatsheet from a bunch of `SlideSection` entries
fn render_cheatsheet(slide_sections: Vec<SlidesSection>) -> String {
    let mut res = String::new();
    for slide in slide_sections.iter() {
        let mut section_str_buf = format!("# {}\n", slide.header);
        for entry in slide.deck_titles.iter() {
            let slide_title = format!("## {entry}\n");
            section_str_buf.push_str(&slide_title);
        }
        section_str_buf.push('\n');
        res.push_str(&section_str_buf);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_slide_name() {
        let test = "* [Methods and Traits](./methods-traits.md)";
        let res = "Methods and Traits";
        assert_eq!(res, get_deck_title(test));

        let test2 = "* [Shared Mutability (Cell, RefCell)](./shared-mutability.md)";
        let res2 = "Shared Mutability (Cell, RefCell)";
        assert_eq!(res2, get_deck_title(test2));
    }

    #[test]
    fn test_focus_regions() {
        let test = "# Summary

[Start Here](./start_here.md)

# Rust Fundamentals

* [Overview](./overview.md)

# Applied Rust

Using Rust on Windows/macOS/Linux. Requires [Rust Fundamentals](#rust-fundamentals).

* [Methods and Traits](./methods-traits.md)

# Advanced Rust

Topics that go beyond [Applied Rust](#applied-rust).

* [Advanced Strings](./advanced-strings.md)

# No-Std Rust

Rust for the Linux Kernel and other no-std environments with an pre-existing C API. Requires [Applied Rust](#applied-rust).
";
        let res = vec![
            vec![
                "# Rust Fundamentals".to_owned(),
                "* [Overview](./overview.md)".to_owned(),
            ],
            vec![
                "# Applied Rust".to_owned(),
                "* [Methods and Traits](./methods-traits.md)".to_owned(),
            ],
            vec![
                "# Advanced Rust".to_owned(),
                "* [Advanced Strings](./advanced-strings.md)".to_owned(),
            ],
        ];
        assert_eq!(focus_regions(test), res);
    }
    #[test]
    fn test_extract_slides() {
        let test = "# Rust Fundamentals
* [Rust I/O Traits](./io.md)
* [Generics](./generics.md)
# No-Std Rust";
        let header = String::from("Rust Fundamentals");
        let slide_titles = vec![String::from("Rust I/O Traits"), String::from("Generics")];
        let res = SlidesSection {
            header,
            deck_titles: slide_titles,
        };
        let region = focus_regions(test);
        assert_eq!(list_of_strings_to_slide_section(&region[0]), res);
        assert!(true);
    }
    #[test]
    fn cheatsheet_tester_full_mock() {
        let lang = "cpp";
        let cheatsheet_text = "# Cpp Cheatsheet
# Rust Fundamentals
## Overview
## Installation
## Basic Types
## Control Flow
## Compound Types
## Ownership and Borrowing
## Error Handling
## Collections
## Iterators
## Imports and Modules
## Good Design Practices

# Applied Rust
## Methods and Traits
## Rust I/O Traits
## Generics
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

";
        let slide_sections: Vec<_> = vec![
            SlidesSection {
                header: "# Rust Fundamentals".into(),
                deck_titles: vec![
                    "Overview".into(),
                    "Installation".into(),
                    "Basic Types".into(),
                    "Control Flow".into(),
                    "Compound Types".into(),
                    "Ownership and Borrowing".into(),
                    "Error Handling".into(),
                    "Collections".into(),
                    "Iterators".into(),
                    "Imports and Modules".into(),
                    "Good Design Practices".into(),
                ],
            },
            SlidesSection {
                header: "# Applied Rust".into(),
                deck_titles: vec![
                    "Methods and Traits".into(),
                    "Rust I/O Traits".into(),
                    "Generics".into(),
                    "Lifetimes".into(),
                    "Cargo Workspaces".into(),
                    "Heap Allocation (Box and Rc)".into(),
                    "Shared Mutability (Cell, RefCell, OnceCell)".into(),
                    "Thread Safety (Send/Sync, Arc, Mutex)".into(),
                    "Closures and the Fn/FnOnce/FnMut traits".into(),
                    "Spawning Threads and Scoped Threads".into(),
                ],
            },
            SlidesSection {
                header: "# Advanced Rust".into(),
                deck_titles: vec![
                    "Advanced Strings".into(),
                    "Building Robust Programs with Kani".into(),
                    "Debugging Rust".into(),
                    "Deconstructing Send, Arc, and Mutex".into(),
                    "Dependency Management with Cargo".into(),
                    "Deref Coercions".into(),
                    "Design Patterns".into(),
                    "Documentation".into(),
                    "Drop, Panic and Abort".into(),
                    "Dynamic Dispatch".into(),
                    "Macros".into(),
                    "Property Testing".into(),
                    "Rust Projects Build Time".into(),
                    "Send and Sync".into(),
                    "Serde".into(),
                    "Testing".into(),
                    "The stdlib".into(),
                    "Using Cargo".into(),
                    "Using Types to encode State".into(),
                ],
            },
            SlidesSection {
                header: "# Rust and Web Assembly".into(),
                deck_titles: vec!["WASM".into()],
            },
        ];

        let _ = cheatsheet_tester(lang, slide_sections, cheatsheet_text);
    }

    #[test]
    #[should_panic]
    fn test_no_opening_title() {
        let test_str = "# notCpp Cheatsheet

        # Rust Fundamentals
        ## Overview
        ## Basic Types
        ## Installation";
        let dummy_vec: Vec<_> = Vec::new();
        let lang = "cpp";

        let _ = cheatsheet_tester(lang, dummy_vec, test_str);
    }

    #[test]
    #[should_panic]
    fn test_wrong_first_header() {
        let test_str = "# Cpp Cheatsheet

        # CLEARLY WRONG TITLE SHOULD PANIC
        ## Overview
        ## Basic Types
        ## Installation";
        let dummy_vec: Vec<_> = Vec::new();
        let lang = "cpp";

        let _ = cheatsheet_tester(lang, dummy_vec, test_str);
    }
}
