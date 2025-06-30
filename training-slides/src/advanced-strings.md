# Advanced Strings

---

There are several different kinds of strings in Rust.

Most common are `String` and `&str`.

## `String`

-   *Owns* the data it stores, and can be mutated freely
-   The bytes it points at exist on the *heap*
-   Does not implement `Copy`, but implements `Clone`

<br>
<br>

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "string:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    _inner [label="<f0> 0x48 | 0x65 | 0x6c | 0x6c | 0x6f | 0x21 | 0xFF | 0xFF", color=blue, fillcolor=green3, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    string [label="<p0> ptr | len = 6 | cap = 8", color=blue, fillcolor=lightblue, style=filled];
    string:p0 -> _inner:f0;

    { rank=same; "string:"; string }
}
```

## `&str`

-   A "string slice reference" (or just "string slice")
-   Usually only seen as a borrowed value
-   The bytes it points at may be anywhere: heap, stack, or in read-only memory

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "str:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    bytes [label="<f0> 0xC2 | 0xA3 | 0x39 | 0x39 | 0x21", color=blue, fillcolor=lightblue, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    str [label="<p0> ptr | len = 5", color=blue, fillcolor=lightblue, style=filled];
    str:p0 -> bytes:f0;

    { rank=same; "str:"; str }
}
```

## Creation

```rust
fn main() {
    // &'static str
    let this = "Hello";
    // String
    let that: String = String::from("Hello");
    // &str
    let other = that.as_str();
}
```

## When to Use What?

-   `String` is the *easiest* to use when starting out. Refine later.
-   `String` owns its data, so works well as a field of a `struct` or `enum`.
-   `&str` is typically used in function arguments.

## `Deref` Coercion

Just because multiple types exist doesn't mean they can't work in harmony.

```rust
fn main() {
    let part_one = String::from("Hello ");
    let part_two = String::from("there ");
    let whole = part_one + &part_two + "world!";
    println!("{}", whole);
}
```

This is because `String` s implement `Deref<Target=str>` .

## Exotic String types

-   `OsStr` and `OsString` may show up when working with file systems or system calls.

-   `CStr` and `CString` may show up when working with FFI.

The differences between `[Os|C]Str` and `[Os|C]String` are generally the same as the normal types.

## `OsString` & `OsStr`

These types represent *platform native* strings. This is necessary because Unix and Windows strings have different characteristics.

## Behind the `OsString` Scenes

-   Unix strings are often arbitrary non-zero 8-bit sequences, usually interpreted as UTF-8.
-   Windows strings are often arbitrary non-zero 16-bit sequences, usually interpreted as UTF-16.
-   Rust strings are always valid UTF-8, and may contain `NUL` bytes.

`OsString` and `OsStr` bridge this gap and allow for conversion to and from `String` and `str`.

Note:

In particular, UNIX file paths are not required to be valid UTF-8 and you might encounter such paths when looking at someone's disk.

Windows file paths are also not required to be valid UTF-16 (i.e. might contain invalid surrogate pairs) and you might encounter such paths when looking at someone's disk.

## `CString` & `CStr`

These types represent valid C compatible strings.

They are predominantly used when doing FFI with external code.

It is strongly recommended you read *all* of the [documentation](https://doc.rust-lang.org/std/ffi/index.html) on these types before using them.

## Common String Tasks

Splitting:

```rust
fn main() {
    let words = "Cow says moo";
    let each: Vec<_> = words.split(" ").collect();
    println!("{:?}", each);
}
```

## Common String Tasks

Concatenation:

```rust
fn main() {
    let animal = String::from("Cow");
    let sound = String::from("moo");
    let words = [&animal, " says ", &sound].concat();
    println!("{:?}", words);
}
```

## Common String Tasks

Replacing:

```rust
fn main() {
    let words = "Cow says moo";
    let replaced = words.replace("moo", "roar");
    println!("{}", replaced);
}
```

## Accepting `String` or `str`

It's possible to accept either rather painlessly:

```rust
fn accept_either<S>(thing: S) -> String
where S: AsRef<str> {
    String::from("foo") + thing.as_ref()
}

fn main() {
    println!("{}", accept_either("blah"));
    println!("{}", accept_either(String::from("blah")));
}
```

## Raw String Literals

-   Starts with `r` followed by zero or more `#` followed by `"`
-   Ends with `"` followed by the same number of `#`
-   Can span multiple lines, leading spaces become part of the line
-   Escape sequences are not processed

```rust
fn main () {
    let json = r##"
{
    "name": "Rust Analyzer",
    "brandColor": "#5bbad5"
}
"##;
    assert_eq!(r"\n", "\\n");
}
```

## Byte String Literals

* not really strings
* used to declare static byte slices (have a `&[u8]` type)

```rust
fn main() {
    let byte_string: &[u8] = b"allows ASCII and \xF0\x9F\x98\x80 only";
    println!("Can Debug fmt but not Display fmt: {:?}", byte_string);
    if let Ok(string) = std::str::from_utf8(byte_string) {
        println!("Now can Display '{}'", string);
    }
}
```
