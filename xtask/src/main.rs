#![deny(warnings)]

mod tasks;

use std::env;

static HELP_TEXT: &'static str = "cargo xtask

USAGE:
    cargo xtask [COMMAND]

COMMANDS:
    make-cheatsheet [LANG]      make LANG cheatsheet by scraping slides names in `SUMMARY.md`
    test-cheatsheet [LANG]      test LANG's cheatsheet (all `SUMMARY.md` items are in sheet and viceversa)

LANG:
    Valid values are `python, go, cpp, swift, java, julia, c`
";

// Code adapted from the xtask workflow in rust-exercises
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // first arg is the name of the executable; skip it
    let unprocessed_args = env::args().skip(1).collect::<Vec<_>>();
    let args: Vec<&str> = unprocessed_args.iter().map(|arg| &arg[..]).collect::<Vec<&str>>();

    let langs = ["python", "go", "cpp", "swift", "java", "julia", "c"];

    if !langs.contains(&args[1]) {
        let panic_text = format!("{} {}\n{}\n", args[1], "is not a valid language name. \n\nExpected one of:", langs.join("\n"));
        panic!("{panic_text}");
    }

    match &args[..] {
        ["make-cheatsheet", lang] => tasks::make_cheatsheet(lang),
        ["test-cheatsheet", lang] => tasks::test_cheatsheet(lang),
        _ => {
            eprintln!("{HELP_TEXT}");

            Ok(())
        }
    }
}

