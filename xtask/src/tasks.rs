use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
struct SlidesSection {
    header: String,
    slide_titles: Vec<String>,
}

fn focus_regions() -> Vec<String> {
    let stream = read_to_string("./slides/book/src/SUMMARY.md").expect("SUMMARY.md not found");
    let mut regions: Vec<String> = vec![];

    // Process `# Rust Fundamentals`, `# Advanced Rust` and stop at `# Advanced Rust`, in that order
    let headers = vec!["# Rust Fundamentals", "# Applied Rust", "# Advanced Rust"];

    // Yes it's a linear seach but it's relocation oblivious and we're iterating over ~100 lines at a time. Boo hoo.
    for header in headers.into_iter() {
        let region = stream
            .lines()
            // Find the slide section we are intereted int
            .skip_while(|l| *l != header)
            // Tricky: accumulate into a string until we find the sentinel string "\n\n#", which is a new slide section
            .take_while(|l| *l != "\n\n# ")
            .collect::<String>();
        regions.push(region);
    }

    assert!(regions.len() == 3);

    regions
}

fn extract_slides(chunk: &str) -> SlidesSection {
    let header = chunk
        .strip_suffix('\n')
        .unwrap()
        .strip_prefix("# ")
        .unwrap()
        .into();

    let slide_titles = chunk
        .lines()
        .into_iter()
        .filter(|l| is_valid_slide_line(*l))
        .map(get_slide_name)
        .collect::<Vec<String>>();

    SlidesSection {
        header,
        slide_titles,
    }
}

#[test]
fn test_extract_slides() {
    let test = r"# Applied Rust

Using Rust on Windows/macOS/Linux. Requires [Rust Fundamentals](#rust-fundamentals).

* [Rust I/O Traits](./io.md)
* [Generics](./generics.md)
";
    let header = String::from("Applied Rust");
    let slide_titles = vec![String::from("Rust I/O Traits"), String::from("Generics")];
    let res = SlidesSection {
        header,
        slide_titles,
    };
    assert_eq!(extract_slides(test), res);
}

fn get_slide_name(line: &str) -> String {
    // SAFETY
    // This file should be a well formed mdbook entries
    let top = line.rfind(']').unwrap();
    let bot = line.find('[').unwrap();
    String::from(&line[bot + 1..top])
}

#[test]
fn test_get_slide_name() {
    let test = "* [Methods and Traits](./methods-traits.md)";
    let res = "Methods and Traits";
    assert_eq!(res, get_slide_name(test));

    let test2 = "* [Shared Mutability (Cell, RefCell)](./shared-mutability.md)";
    let res2 = "Shared Mutability (Cell, RefCell)";
    assert_eq!(res2, get_slide_name(test2));
}

fn is_valid_slide_line(line: &str) -> bool {
    if line.starts_with('#') || line.is_empty() || !line.starts_with('*') || !line.ends_with(".md)")
    {
        false
    } else {
        true
    }
}

#[test]
fn test_valid_slide_lines() {
    let test1 = "# Applied Rust";
    let test2 = "";
    let test3 =
        "Using Rust on Windows/macOS/Linux. Requires [Rust Fundamentals](#rust-fundamentals).";
    let test4 = "* [Methods and Traits](./methods-traits.md)";

    assert!(!is_valid_slide_line(test1));
    assert!(!is_valid_slide_line(test2));
    assert!(!is_valid_slide_line(test3));
    assert!(is_valid_slide_line(test4));
}

pub fn make_cheatsheet(lang: &str) -> color_eyre::Result<()> {
    println!("make_cheatsheet for {lang}");
    Ok(())
}

pub fn test_cheatsheet(lang: &str) -> color_eyre::Result<()> {
    println!("make_cheatsheet for {lang}");
    Ok(())
}
