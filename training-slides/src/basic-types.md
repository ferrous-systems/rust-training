# Basic types

## ints

Rust comes with all standard int types, with and without sign

-   `i8`, `u8`
-   `i16`, `u16`
-   `i32`, `u32`
-   `i64`, `u64`
-   `i128`, `u128`

## Syntactic clarity in specifying numbers

```rust,ignore
    123_456   // underscore as separator
    0x12      // prefix 0x to indicate hex value
    0o23      // prefix 0o to indicate octal value
    0b0001    // prefix 0b to indicate binary value
    b'a'      // A single byte character
```

## Architecture-dependent numbers

Rust comes with two architecture-dependent number types:

-   `isize`, `usize`

## Casts

Casts between number are possible, **also shortening casts**:

```rust,editable
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

Boolean in Rust is represented by either of two values: `true` or `false`

## Character

`char` is a [Unicode Scalar
Value](https://www.unicode.org/glossary/#unicode_scalar_value) being
represented as a "single character"

-   A literal in single quotes: `'r'`
-   Four (4) bytes in size
-   Can save more than ASCII, like glyphs, emoji, accented characters
    etc.

<!-- -->

```rust,editable
    fn main() {
        // U+0072 LATIN SMALL LETTER R
        let ascii_char = 'r';
        // U+03BC GREEK SMALL LETTER MU
        let special_char = 'μ';
        // U+0154 LATIN CAPITAL LETTER R WITH ACUTE
        let accented_char = 'Ŕ';
      // U+1F60E Symbol, other
        let emoji_char = '😎';
    }
```

```rust,ignore,does_not_compile,editable
    fn main() {
        // U+1F468 U+200D U+1F469 U+200D U+1F467 U+200D U+1F467
        let seven_chars_emoji = '👨‍👩‍👧‍👧'; // Error: char must be one codepoint long
    }
```

## Arrays

Fixed-size arrays have the following notation:

```rust
    fn main() {
        let arr: [i32; 4] = [1,2,3,4];
    }
```

## Arrays of dynamic size

Arrays of dynamic size in Rust are represented as slices.

Slices carry a pointer to the array and a length. Slices cannot be
resized.

```rust
    fn main() {
        let arr: &[i32] = &[1,2,3,4];
    }
```
