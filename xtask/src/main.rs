#![deny(warnings)]

mod tasks;

use std::env;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // first arg is the name of the executable; skip it
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|arg| &arg[..]).collect::<Vec<_>>();

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
    
",
            );

            Ok(())
        }
    }
}

