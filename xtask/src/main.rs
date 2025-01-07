#![deny(warnings)]

mod tasks;

use std::env;

static LANG_LIST: [&'static str; 7] = ["python", "go", "cpp", "swift", "java", "julia", "c"];
static HELP_TEXT: &'static str = "cargo xtask

USAGE:
    cargo xtask [COMMAND]

COMMANDS:
    make-cheatsheet [LANG]      make LANG cheatsheet by scraping slides names in `SUMMARY.md`
    test-cheatsheet [LANG]      test LANG's cheatsheet (all `SUMMARY.md` items are in sheet)

LANG:

    We support $$LANG_LIST$$
";

// Code adapted from the xtask workflow in rust-exercises
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // First arg is the name of the executable; skip it
    let unprocessed_args = env::args().skip(1).collect::<Vec<_>>();
    let args: Vec<&str> = unprocessed_args
        .iter()
        .map(|arg| &arg[..])
        .collect::<Vec<&str>>();

    let printed_help_text = HELP_TEXT.replace("$$LANG_LIST$$", &join_str(&LANG_LIST));

    // Check they gave the right number of args
    if args.len() != 2 {
        panic!("Incorrect number of arguments.\n\n{printed_help_text}");
    }

    // We replace $$LANG_LISTS$$ with the pretty-printed langs
    let langs = join_str(&LANG_LIST);

    // Check if they gave a language we support
    if !LANG_LIST.contains(&args[1]) {
        let panic_text = format!(
            "{} {}\n{}\n",
            args[1], "is not a valid language name. \n\nExpected one of:", langs
        );
        panic!("{panic_text}\n===========\n{printed_help_text}");
    }

    match &args[..] {
        ["make-cheatsheet", lang] => tasks::make_cheatsheet(lang),
        ["test-cheatsheet", lang] => tasks::test_cheatsheet(lang),
        _ => {
            Ok(())
        }
    }
}

// Helper function for pretty printing the lang list string
fn join_str(input: &[&str]) -> String {
    let mut output = String::new();
    let mut iter = input.iter().peekable();
    while let Some(lang) = iter.next() {
        if iter.peek().is_none() {
            output += " or ";
            output += lang;
        } else {
            output += lang;
            output += ", ";
        }
    }
    output
}
