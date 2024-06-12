use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

#[derive(Debug, Eq, PartialEq)]
struct SlidesSection {
    header: String,
    slide_titles: Vec<String>,
}

// fn is_valid_slide_line(line: &str) -> bool {
//     !line.starts_with('#') && !line.is_empty() && line.starts_with('*') && line.ends_with(".md)")
// }

// #[test]
// fn test_valid_slide_lines() {
//     let test1 = "# Applied Rust";
//     let test2 = "";
//     let test3 =
//         "Using Rust on Windows/macOS/Linux. Requires [Rust Fundamentals](#rust-fundamentals).";
//     let test4 = "* [Methods and Traits](./methods-traits.md)";

//     assert!(!is_valid_slide_line(test1));
//     assert!(!is_valid_slide_line(test2));
//     assert!(!is_valid_slide_line(test3));
//     assert!(is_valid_slide_line(test4));
// }

const INITIAL_HEADER: &str = "# Rust Fundamentals";
const LAST_HEADER: &str = "# No-Std Rust";

fn get_slide_name(line: &str) -> String {
    // SAFETY
    // This line should be a well formed mdbook entries: `* [TEXT](./foo.md)`
    let top = line
        .rfind(']')
        .expect("the markdown file entry did not have a ']'");
    let bot = line
        .find('[')
        .expect("the markdown file entry did not have a '['");
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

fn focus_regions(text: &str) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current_section: Vec<String> = Vec::new();

    if !text.contains(INITIAL_HEADER) {
        panic!("Your INITIAL_HEADER is not part of the input. Check your `SUMMARY.md` for {INITIAL_HEADER}");
    }
    if !text.contains(LAST_HEADER) {
        panic!("YOUR LAST_HEADER is not part of the text input. CHECK your `SUMMARY.md` for {LAST_HEADER}");
    }

    let first_header = text.find(INITIAL_HEADER).unwrap();
    let last_header = text.rfind(LAST_HEADER).unwrap();

    let text = &text[first_header..last_header];

    for line in text.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty()
            || (!trimmed_line.starts_with('*') && !trimmed_line.starts_with('#'))
        {
            continue;
        }

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

fn extract_slides(chunk: Vec<String>) -> SlidesSection {
    assert!(chunk.len() > 2);
    // # Rust Fundamentals
    //   ^ 3rd character in title
    let header = String::from(&chunk[0][2..]);

    let slide_titles = chunk[1..]
        .iter()
        .map(|l| get_slide_name(l))
        .collect::<Vec<String>>();

    SlidesSection {
        header,
        slide_titles,
    }
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
        slide_titles,
    };
    let region = focus_regions(test);
    assert_eq!(extract_slides(region[0].clone()), res);
    assert!(true);
}

pub fn make_cheatsheet(lang: &str) -> Result<(), eyre::Report>{
    // Collect headers
    let text = read_to_string("./training-slides/src/SUMMARY.md").expect("SUMMARY.md not found");
    let slide_texts = focus_regions(&text);
    let slide_sections: Vec<SlidesSection> = slide_texts
        .iter()
        .map(|l| extract_slides(l.clone()))
        .collect();

    // Check to see if a file exists
    let file_str = format!("./training-slides/src/{lang}-cheatsheet.md");
    let new_file = Path::new(&file_str);

    // If so, just check if headers any headers are missing
    // otherwise, create the new file, then write new file into summary.md
    eprintln!("file is {file_str}");
    match File::create_new(new_file) {
        Ok(mut f) => {
            eprintln!("File {file_str} just created");
            let result_text = write_cheatsheet(slide_sections);
            let _ = f.write_all(&result_text.as_bytes());
            //.error!("Could not write slide headers to file");
            eprintln!("Cheatsheat for {lang} written at {file_str}");
        }
        Err(_) => {
            eprintln!("File {lang}-cheatsheet.md already exists - checking it's in sync");
        }
    }
    Ok(())
}

pub fn test_cheatsheet(lang: &str) -> Result<(), eyre::Report> {
    println!("make_cheatsheet for {lang}");
    let text = read_to_string("./training-slides/src/SUMMARY.md").expect("SUMMARY.md not found");
    let slide_texts = focus_regions(&text);
    let slide_sections: Vec<SlidesSection> = slide_texts
        .iter()
        .map(|l| extract_slides(l.clone()))
        .collect();
    println!("test-cheatsheet {lang} {slide_sections:?}");
    Ok(())
}

fn write_cheatsheet(slide_sections: Vec<SlidesSection>) -> String {
    let mut res = String::new();
    for slide in slide_sections.iter() {
        let mut section_str_buf = format!("# {}\n", slide.header);
        for entry in slide.slide_titles.iter() {
            let slide_title = format!("## {entry}\n");
            section_str_buf.push_str(&slide_title);
        }
        section_str_buf.push_str("\n");
        res.push_str(&section_str_buf);
    }
    res
}
