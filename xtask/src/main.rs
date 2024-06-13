//#![deny(warnings)]

mod tasks;

use std::env;

// Code adapted from the xtask workflow in rust-exercises
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // first arg is the name of the executable; skip it
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|arg| &arg[..]).collect::<Vec<_>>();

    let langs = vec!["python", "go", "cpp", "swift", "java", "julia", "c"];

    if !langs.contains(&args[1]) {
        panic!("{} is not a valid language name. \nExpected one of: python, go, cpp, swift, java, julia, c", &args[1]);
    }

    match &args[..] {
        ["make-cheatsheet", lang] => tasks::make_cheatsheet(lang),
        ["test-cheatsheet", lang] => tasks::test_cheatsheet(lang),
        _ => {
            eprintln!(
                "cargo xtask

USAGE:
    cargo xtask [COMMAND]

COMMANDS:
    make-cheatsheet [LANG]      make LANG cheatsheet by scraping slides names in `SUMMARY.md`
    test-cheatsheet [LANG]      test LANG's cheatsheet (all `SUMMARY.md` items are in sheet and viceversa)

LANG:
    Valid values are `python, go, cpp, swift, java, julia, c`
",
            );

            Ok(())
        }
    }
}
