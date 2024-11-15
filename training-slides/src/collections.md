# Collections

## Using Arrays

Arrays (`[T; N]`) have a fixed size.

```rust []
fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("array = {:?}", array);
}
```

<br>

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "array:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=4.75, fixedsize=true];
    array [label="1 | 2 | 3 | 4 | 5", color=blue, fillcolor=lightblue, style=filled];

    { rank=same; "array:"; array }
}
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

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "array:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=4.75, fixedsize=true];
    array [label="0 | 1 | 2 | 3 | 4 | 0 | 0 | 0 | 0 | 0", color=blue, fillcolor=lightblue, style=filled];

    { rank=same; "array:"; array }
}
```

## Slices

A view into *some other data*. Written as `&[T]` (or `&mut [T]`).

```rust [1-8|6]
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

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "data:" -> "array:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    array [label="<f0> 0 | 1 | 2 | 3 | 4 | 0 | 0 | 0 | 0 | 0", color=blue, fillcolor=lightblue, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    data [label="<p0> ptr | len = 5", color=blue, fillcolor=lightblue, style=filled];
    data:p0 -> array:f0;

    { rank=same; "data:"; data }
    { rank=same; "array:"; array }
}
```

Note:
Slices are *unsized* types and can only be access via a reference. This reference is a 'fat reference' because instead of just containing a pointer to the start of the data, it also contains a length value.

## Vectors

`Vec` is a growable, heap-allocated, array-like type.

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

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "vector:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    _inner [label="<f0> 2 | 4 | 6 | 0", color=blue, fillcolor=lightgreen, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    vector [label="<p0> ptr | len = 3 | cap = 4", color=blue, fillcolor=lightblue, style=filled];
    vector:p0 -> _inner:f0;

    { rank=same; "vector:"; vector }
}
```

Note:

The green block of data is heap allocated.

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

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "s:" -> "bytes:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    bytes [label="<f0> 0xC2 | 0xA3 | 0x39 | 0x39 | 0x21", color=blue, fillcolor=lightblue, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    s [label="<p0> ptr | len = 5", color=blue, fillcolor=lightblue, style=filled];
    s:p0 -> bytes:f0;

    { rank=same; "s:"; s }
    { rank=same; "bytes:"; bytes }
}
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

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "s:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    bytes [label="<f0> 0x48 | 0x65 | 0x6c | 0x6c | 0x6f | 0x21", color=blue, fillcolor=lightgray, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    s [label="<p0> ptr | len = 5", color=blue, fillcolor=lightblue, style=filled];
    s:p0 -> bytes:f0;

    { rank=same; "s:"; s }
}
```

Note:

The lifetime annotation of `'static` just means the string slice lives forever
and never gets destroyed. We wrote out the type in full so you can see it - you
can emit it on variable declarations.

There's a second string literal in this program. Can you spot it?

(It's the format string in the call to `println!`)

## Strings ([docs](https://doc.rust-lang.org/std/string/struct.String.html))

* A growable collection of `char`
* Actually stored as a `Vec<u8>`, with UTF-8 encoding
* You cannot access characters by index (only bytes)
  * But you never really want to anyway

```rust
fn main() {
    let string = String::from("Hello!");
}
```

<br>

```dot process
digraph {
    node [shape=plaintext, fontcolor=black, fontsize=18];
    "string:" [color=white];

    node [shape=record, fontcolor=black, fontsize=14, width=3];
    _inner [label="<f0> 0x48 | 0x65 | 0x6c | 0x6c | 0x6f | 0x21", color=blue, fillcolor=lightgreen, style=filled];
    node [shape=record, fontcolor=black, fontsize=14, width=2];
    string [label="<p0> ptr | len = 6 | cap = 6", color=blue, fillcolor=lightblue, style=filled];
    string:p0 -> _inner:f0;

    { rank=same; "string:"; string }
}
```

Note:

The green block of data is heap allocated.

## Making a String

```rust [1-7|2|3|4|5|6]
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

## Downsides of VecDeque

* Cannot borrow it as a single `&[T]` slice without moving items around
* Not great for insertion in the middle
* Everything must be of the same type
* Indices are always `usize`

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
 
fn update_connection(map: &mut HashMap<i32, u64>, id: i32) {
    map.entry(id)
        .and_modify(|v| *v = *v + 1)
        .or_insert(1);
}
 
fn main() {
    let mut map = HashMap::new();
    update_connection(&mut map, 100);
    update_connection(&mut map, 200);
    update_connection(&mut map, 100);
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

---

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

Technically you *can* insert into the middle of a Vec or a String, but we're
talking about 'cheap' insertions that don't involve moving too much stuff
around.
