//! Process an mdbook, removing line highlight hints from code blocks
//!
//! Changes:
//!
//! ````text
//! ```rust [1|2-3|4]
//! fn example() {}
//! ```
//! ````
//!
//! To:
//!
//! ````text
//! ```rust
//! fn example() {}
//! ```
//! ````

use mdbook_preprocessor::errors::Result as MdbookResult;
use mdbook_preprocessor::parse_input;

/// mdbook code block attributes we want to let through
static ATTRS: [&str; 11] = [
    "editable",
    "noplayground",
    "mdbook-runnable",
    "ignore",
    "should_panic",
    "no_run",
    "compile_fail",
    "edition2015",
    "edition2018",
    "edition2021",
    "edition2024",
];

fn main() -> MdbookResult<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return Ok(());
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    let (_ctx, mut book) = parse_input(std::io::stdin())?;
    book.for_each_mut(|item| match item {
        mdbook_preprocessor::book::BookItem::Chapter(chapter) => {
            let mut lines: Vec<String> = chapter.content.lines().map(|s| s.to_string()).collect();
            for line in lines.iter_mut() {
                if line.starts_with("````rust") {
                    let mut newline = String::from("````rust");
                    for attr in ATTRS {
                        if line.contains(attr) {
                            newline.push(',');
                            newline.push_str(attr);
                        }
                    }
                    *line = newline;
                }
            }
            chapter.content = lines.join("\n");
        }
        _ => {}
    });
    serde_json::to_writer(std::io::stdout(), &book)?;

    Ok(())
}
