# Basic Types

## Integers

Rust comes with all standard int types, with and without sign

* `i8`, `u8`
* `i16`, `u16`
* `i32`, `u32`
* `i64`, `u64`
* `i128`, `u128`

## Syntactic clarity in specifying numbers

```rust
let x = 123_456;   // underscore as separator
let x = 0x12;      // prefix 0x to indicate hex value
let x = 0o23;      // prefix 0o to indicate octal value
let x = 0b0001;    // prefix 0b to indicate binary value
let x = b'a';      // A single u8
```

## Architecture-dependent Numbers

Rust comes with two architecture-dependent number types:

* `isize`, `usize`

## Casts

Casts between number are possible, **also shortening casts**:

```rust
fn main() {
    let foo = 3_i64;
    let bar = foo as i32;
}
```

---

If the size isn’t given, or cannot be inferred, ints default to `i32`.

## Overflows

Overflows trigger a trap in Debug mode, but not in release mode. This
behaviour can be configured.

## Floats

Rust also comes with floats of all standard sizes: `f32`, `f64`

```rust
fn main() {
    let float: f64 = 1.0;
}
```

## Boolean

Boolean in Rust is represented by either of two values: `true` or
`false`

## Character

`char` is a [Unicode Scalar Value](https://www.unicode.org/glossary/#unicode_scalar_value) being represented as a "single character"

* A literal in single quotes: `'r'`
* Four (4) bytes in size
* More than just ASCII: glyphs, emoji, accented characters, etc.

## Character Literals

```rust [2-3|4-5|6-7|8-9]
fn main() {
    // U+0072 LATIN SMALL LETTER R
    let ascii_char = 'r';
    // U+03BC GREEK SMALL LETTER MU
    let special_char = 'μ';
    // U+0154 LATIN CAPITAL LETTER R WITH ACUTE
    let accented_char = 'Ŕ';
    // U+1F60E SMILING FACE WITH SUNGLASSES
    let emoji_char = '😎';
}
```

## Character Literals

```rust,ignore
fn main() {
    // U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F467
    let seven_chars_emoji = '👨‍👩‍👧‍👧'; // Error: char must be one codepoint long
}
```

## Arrays

* Arrays have multiple elements of the same type.
* They are of fixed size (it's part of the type).

```rust
fn main() {
    let arr: [i32; 4] = [1, 2, 3, 4];
}
```

## Slices

* Slices are like arrays, but with a run-time specified size.
* Slices carry a pointer to some other array, and a length.
* Slices cannot be resized but can be subsliced.

```rust [2|3]
fn main() {
    let slice: &[i32] = &[1, 2, 3, 4];
    let sub: &[i32] = &slice[0..1];
}
```

Note:

* Use `.get()` method on the slice to avoid panics instead of accessing via index.
* The range syntax include the first value but excludes the last value. Use `0..=1` to include both ends.

## String Slices

* Strings Slices (`&str`) are a special kind of `&[u8]`
* They are *guaranteed* to be a valid UTF-8 encoded Unicode string
* It is *undefined behaviour* to create one that isn't valid UTF-8
* Slicing must be done on *character boundaries*

```rust []
fn main() {
    let hello_world: &str = "Hello 😀";
    println!("Start = {}", &hello_world[0..5]);
    // println!("End = {}", &hello_world[7..]);
}
```

Note:

Use [`std::str::from_utf8`](https://doc.rust-lang.org/std/str/fn.from_utf8.html) to make an `&str` from a `&[u8]`
Let trainees know that Strings are covered over many slides in the training and that an `Advanced Strings` slides exist for completeness' sake
