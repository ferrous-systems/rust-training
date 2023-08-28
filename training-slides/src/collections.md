# Collections

## Using Arrays

In Rust, arrays (`[T; N]`) have a fixed size.

```rust []
fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("array = {:?}", array);
}
```

<br>

```mermaid
flowchart LR
    array("[1, 2, 3, 4, 5]")
```

## Building the array at runtime.

How do you know how many 'slots' you've used?

```rust []
fn main() {
    let mut array = [0u8; 10];
    for idx in 0..5 {
        array[idx] = idx as u8;
    }
    println!("array = {:?}", array);
}
```

<br>

```mermaid
flowchart LR
    array("[0, 1, 2, 3, 4, 0, 0, 0, 0, 0]")
```

## Slices

A view into *some other data*, plus a value to indicate *how many items*.

Written as `&[T]` (or `&mut [T]`).

```rust [1-8|6]
// 🚌🛑 2
fn main() {
    let mut array = [0u8; 10];
    for idx in 0..5 {
        array[idx] = idx as u8;
    }
    let data = &array[0..5];
    println!("data = {:?}", data);
}
```

<br>

```mermaid
flowchart LR
    array("[0, 1, 2, 3, 4, 0, 0, 0, 0, 0]")
    data["&[u8] { ptr, len: 5 }"]
    data -.-> array
```

Note:
Slices are *unsized* types and can only be access via a reference. This reference is a 'fat reference' because instead of just containing a pointer to the start of the data, it also contains a length value.

## Vectors

If you don't how how much space you will need, a `Vec` is a growable, heap-allocated, array-like type, that you can index and also treat as a slice.

```rust []
fn process_data(input: &[u32]) {
    let mut vector = Vec::new();
    for value in input {
        vector.push(value * 2);
    }
    println!("vector = {:?}, first = {}", vector, vector[0]);
}

fn main() { process_data(&[1, 2, 3]); }
```

<br>

```mermaid
flowchart LR
    data("[2, 4, 6, 0]")
    vector["Vec { ptr, cap: 4, len: 3 }"]
    vector --> data
    style data fill:#ccf
```

Note:

The dark blue block of data is heap allocated.

## There's a macro short-cut too...

```rust
fn main() {
    let mut vector = vec![1, 2, 3, 4];
}
```

<br>

Check out the [docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)!

## Features of Vec

* Growable (will re-allocate if needed)
* Can borrow it as a `&[T]` slice
* Can access any element (`vector[i]`) quickly
* Can push/pop from the back easily

## Downsides of Vec

* Not great for insertion
* Everything must be of the same type
* Indices are always `usize`

## String Slices

The basic string types in Rust are all UTF-8.

A *String Slice* (`&str`) is an immutable view on to some valid UTF-8 bytes

```rust
fn main() {
    let bytes = [0xC2, 0xA3, 0x39, 0x39, 0x21];
    let s = std::str::from_utf8(&bytes).unwrap();
    println!("{}", s);
}
```

<br>

```mermaid
flowchart LR
    data("[0xC2, 0xA3, 0x39, 0x39, 0x21]")
    s["&str { ptr, len: 5 }"]
    s -.-> data
```

Note:

A string slice is tied to the lifetime of the data that it refers to.

## String Literals

* String Literals produce a string slice "with static lifetime"
* Points at some bytes that live in read-only memory with your code

```rust []
fn main() {
    let s: &'static str = "Hello!";
    println!("s = {}", s);
}
```

<br>

```mermaid
flowchart LR
    data1("[0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21")
    str1["&str { ptr, len: 6 }"]
    str1 -.-> data1

    data2("[0x73, 0x20, 0x3d, 0x20")
    str2["&str { ptr, len: 4 }"]
    str2 -.-> data2

    style data1 fill:#cfc
    style data2 fill:#cfc
```

Note:

The lifetime annotation of `'static` just means the string slice lives forever
and never gets destroyed. We wrote out the type in full so you can see it - you
can emit it on variable declarations.

The second green object is the literal we gave to println - `s = `

## Strings ([docs](https://doc.rust-lang.org/std/string/struct.String.html))

* A growable collection of `char`
* Actually stored as a `Vec<u8>`, with UTF-8 encoding
* You cannot acccess characters by index (only bytes)
  * But you never really want to anyway

```mermaid
flowchart LR
    data("[0xc2, 0xa3, 0x31, 0x32, 0x00]")
    string["String { ptr, cap: 5, len: 4 }"]
    string --> data
    style data fill:#ccf
```

Note:

The dark blue block of data is heap allocated.

## Making a String

```rust [1-7|2|3|4|5|6]
// 🚌🛑 6
fn main() {
    let s1 = "String literal up-conversion".to_string();
    let s2: String = "Into also works".into();
    let s3 = String::from("Or using from");
    let s4 = format!("String s1 is {:?}", s1);
    let s5 = String::new(); // empty
}
```

## Appending to a String

```rust [1-7|2|3|4|5-6]
// 🚌🛑 5
fn main() {
    let mut start = "Mary had a ".to_string();
    start.push_str("little");
    let rhyme = start + " lamb";
    println!("rhyme = {}", rhyme);
    // println!("start = {}", start);
}
```

## Joining pieces of String

```rust [1-5|2|3|4]
// 🚌🛑 4
fn main() {
    let pieces = ["Mary", "had", "a", "little", "lamb"];
    let rhyme = pieces.join(" ");
    println!("Rhyme = {}", rhyme);
}
```

## VecDeque ([docs](https://doc.rust-lang.org/std/collections/struct.VecDeque.html))

A ring-buffer, also known as a Double-Ended Queue:

```rust []
use std::collections::VecDeque;
fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    println!("first: {:?}", queue.pop_front());
    println!("second: {:?}", queue.pop_front());
    println!("third: {:?}", queue.pop_front());
}
```

## Features of VecDeque

* Growable (will re-allocate if needed)
* Can access any element (`queue[i]`) quickly
* Can push/pop from the front or back easily

## Downsides of Vec

* Cannot borrow it as a single `&[T]` slice without moving items around
* Not great for insertion in the middle
* Everything must be of the same type
* Indicies are always `usize`

## HashMap ([docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html))

If you want to store *Values* against *Keys*, Rust has `HashMap<K, V>`.

Note that the keys must be all the same type, and the values must be all the same type.

```rust
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("Triangle", 3);
    map.insert("Square", 4);
    println!("Triangles have {:?} sides", map.get("Triangle"));
    println!("Triangles have {:?} sides", map["Triangle"]);
    println!("map {:?}", map);
}
```

Note:
The index operation will panic if the key is not found, just like with slices and arrays if the index is out of bounds. Get returns an `Option`.

If you run it a few times, the result will change because it is un-ordered.

## The Entry API

What if you want to *update an existing value* __OR__ *add a new value if it's not there yet*?

`HashMap` has the *Entry API*:

```rust ignore
enum Entry<K, V> {
    Occupied(...),
    Vacant(...),
}

fn entry(&mut self, key: K) -> Entry<K, V> {
    ...
}
```

## Entry API Example

```rust []
use std::collections::HashMap;

fn award_points(name: &'static str, map: &mut HashMap<String, u64>) {
    map.entry(name.to_string())
        .and_modify(|v| *v += 1)
        .or_insert(1);
}

fn main() {
    let mut map = HashMap::new();
    award_points("Sam", &mut map);
    award_points("Bob", &mut map);
    award_points("Sam", &mut map);
    println!("{:?}", map);
}
```

## Features of HashMap

* Growable (will re-allocate if needed)
* Can access any element (`map[i]`) quickly
* Great at insertion
* Can choose the *Key* and *Value* types independently

## Downsides of HashMap

* Cannot borrow it as a single `&[T]` slice
* Everything must be of the same type
* Unordered

## BTreeMap ([docs](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html))

Like a `HashMap`, but kept in-order.

```rust
use std::collections::BTreeMap;
fn main() {
    let mut map = BTreeMap::new();
    map.insert("Triangle", 3);
    map.insert("Square", 4);
    println!("Triangles have {:?} sides", map.get("Triangle"));
    println!("Triangles have {:?} sides", map["Triangle"]);
    println!("map {:?}", map);
}
```

## Features of BTreeMap

* Growable (will re-allocate if needed)
* Can access any element (`map[i]`) quickly
* Great at insertion
* Can choose the *Key* and *Value* types independently
* Ordered

## Downsides of BTreeMap

* Cannot borrow it as a single `&[T]` slice
* Everything must be of the same type
* Slower than a `HashMap`

## Sets

We also have [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) and [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).

Just sets the `V` type parameter to `()`!

## A Summary

| Type         | Owns | Grow |  Index  | Slice | Cheap Insert |
| :----------- | :--: | :--: | :-----: | :---: | :----------: |
| Array        |  ✅  |  ❌  | `usize` |  ✅   |      ❌      |
| Slice        |  ❌  |  ❌  | `usize` |  ✅   |      ❌      |
| Vec          |  ✅  |  ✅  | `usize` |  ✅   |      ↩       |
| String Slice |  ❌  |  ❌  |   🤔   |  ✅   |      ❌      |
| String       |  ✅  |  ✅  |   🤔   |  ✅   |      ↩       |
| VecDeque     |  ✅  |  ✅  | `usize` |  🤔  |    ↪ / ↩     |
| HashMap      |  ✅  |  ✅  |   `T`   |  ❌   |      ✅      |
| BTreeMap     |  ✅  |  ✅  |   `T`   |  ❌   |      ✅      |

Note:

The 🤔 for indexing string slices and Strings is because the index is a byte
offset and the system will panic if you try and chop a UTF-8 encoded character
in half.

The 🤔 for indexing VecDeque is because you might have to get the contents in
two pieces (i.e. as two disjoint slices) due to wrap-around.

Tecnically you *can* insert into the middle of a Vec or a String, but we're
talking about 'cheap' insertions that don't involve moving too much stuff
around.
