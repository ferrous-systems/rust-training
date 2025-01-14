# Lifetimes

## Rust Ownership

* Every piece of memory in Rust program has exactly one owner at the time
* Ownership changes ("moves")
  * `fn takes_ownership(data: Data)`
  * `fn producer() -> Data`
  * `let people = [paul, john, emma];`

## Producing owned data

```rust ignore
fn producer() -> String {
    String::new()
}
```

## Producing references?

```rust ignore
fn producer() -> &str {
    // ???
}
```

* `&str` "looks" at some string data. Where can this data come from?

## Local Data

Does this work?

```rust ignore
fn producer() -> &str {
    let s = String::new();
    &s
}
```

## Local Data

No, we can't return a reference to local data...

<pre><code data-trim data-noescape><span class="er b">error[E0515]</span><b>: cannot return reference to local variable `s`</b>
 <span class="eb b">--&gt; </span>src/lib.rs:3:5
  <span class="eb b">|</span>
<span class="eb b">3</span> <span class="eb b">|</span>     &amp;s
  <span class="eb b">|</span>     <span class="er b">^^</span> <span class="er b">returns a reference to data owned by the current function</span>
</code></pre>

## Local Data

You will also see:

<pre><code data-trim data-noescape><span class="er b">error[E0106]</span><b>: missing lifetime specifier</b>
 <span class="eb b">--&gt; </span>src/lib.rs:1:18
  <span class="eb b">|</span>
<span class="eb b">1</span> <span class="eb b">|</span> fn producer() -&gt; &amp;str {
  <span class="eb b">|</span>                  <span class="er b">^</span> <span class="er b">expected named lifetime parameter</span>
  <span class="eb b">|</span>
</code></pre>

## Static Data

```rust
fn producer() -> &'static str {
    "hello"
}
```

* bytes `h e l l o` are "baked" into your program
* part of *static* memory (not heap or stack)
* a slice pointing to these bytes will always be valid
* **safe** to return from `producer` function

Note:

You didn't need to specify `'static` for the static variable - there's literally no other lifetime that can work here.

How big is a `&'static str`? Do you think the length lives with the string data, or inside the str-reference itself?

(It lives with the reference - so you can take sub-slices)

## Static Data

It doesn't have to be a string literal - any reference to a static is OK.

```rust
static HELLO: [u8; 5] = [0x68, 0x65, 0x6c, 0x6c, 0x6f];

fn producer() -> &'static str {
    std::str::from_utf8(&HELLO).unwrap()
}
```

## `'static` annotation

* Rust never assumes `'static` for function returns or fields in types
* `&'static T` means this reference to `T` will never become invalid
* `T: 'static` means that "if type `T` has any references inside they should be `'static`"
  * `T` may have no references inside at all!
* string literals are always `&'static str`

---

```rust ignore
fn takes_and_returns(s: &str) -> &str {

}
```

Where can the returned <code>&str</code> come from?

<ul>
    <li class="fragment">can't be local data</li>
    <li class="fragment">is not marked as <code>'static</code></li>
    <li class="fragment"><strong>Conclusion: must come from <code>s</code>!</strong></li>
</ul>

## Multiple sources

```rust ignore
fn takes_many_and_returns(s1: &str, s2: &str) -> &str {

}
```

Where can the returned <code>&str</code> come from?

<ul>
    <li class="fragment">is not marked as <code>'static</code></li>
    <li class="fragment">should it be <code>s1</code> or <code>s2</code>?</li>
    <li class="fragment"><strong>Ambiguous. Should ask programmer for help!</strong></li>
</ul>

## Tag system

```rust ignore
fn takes_many_and_returns<'a>(s1: &str, s2: &'a str) -> &'a str {

}
```

"Returned `&str` comes from `s2`"

## `'a`

* "Lifetime annotation"
* often called "lifetime" for short, but that's a very bad term
  * every reference has a lifetime
  * annotation doesn't name a lifetime of a reference, but used to tie lifetimes of several references together
  * builds *"can't outlive"* and *"should stay valid for as long as"* relations
* arbitrary names: `'a`, `'b`, `'c`, `'whatever`

## Lifetime annotations in action

```rust ignore
fn first_three_of_each(s1: &str, s2: &str) -> (&str, &str) {
    (&s1[0..3], &s1[0..3])
}

fn main() {
    let amsterdam = format!("AMS Amsterdam");

    let (amsterdam_code, denver_code) = {
        let denver = format!("DEN Denver");
        first_three_of_each(&amsterdam, &denver)
    };

    println!("{} -> {}", amsterdam_code, denver_code);
}
```

## Annotate!

```rust ignore
fn first_three_of_each<'a, 'b>(s1: &'a str, s2: &'b str) -> (&'a str, &'b str) {
    (&s1[0..3], &s1[0..3])
}
```

## Annotations are used to validate function body

"The source you used in code doesn't match the tags"

<pre><code data-trim data-noescape><span class="er b">error</span><b>: lifetime may not live long enough</b>
 <span class="eb b">--&gt; </span>src/lib.rs:2:5
  <span class="eb b">|</span>
<span class="eb b">1</span> <span class="eb b">|</span> fn first_three_of_each&lt;&apos;a, &apos;b&gt;(s1: &amp;&apos;a str, s2: &amp;&apos;b str) -&gt; (&amp;&apos;a str, &amp;&apos;b str) {
  <span class="eb b">|</span>                        <span class="eb b">--</span>  <span class="eb b">--</span> <span class="eb b">lifetime `&apos;b` defined here</span>
  <span class="eb b">|</span>                        <span class="eb b">|</span>
  <span class="eb b">|</span>                        <span class="eb b">lifetime `&apos;a` defined here</span>
<span class="eb b">2</span> <span class="eb b">|</span>     (&amp;s1[0..3], &amp;s1[0..3])
  <span class="eb b">|</span>     <span class="er b">^^^^^^^^^^^^^^^^^^^^^^</span> <span class="er b">function was supposed to return data with lifetime `&apos;b` but it is returning data with lifetime `&apos;a`</span>
  <span class="eb b">|</span>
  <span class="eb b">= </span><b>help</b>: consider adding the following bound: `&apos;a: &apos;b`
</code></pre>

## Annotations are used to validate reference lifetimes at a call site

"Produced reference *can't outlive* the source"

<pre><code data-trim data-noescape><span class="er b">error[E0597]</span><b>: `amsterdam` does not live long enough</b>
  <span class="eb b">--&gt; </span>src/main.rs:10:29
   <span class="eb b">|</span>
<span class="eb b">6</span>  <span class="eb b">|</span>     let amsterdam = format!(&quot;AMS Amsterdam&quot;);
   <span class="eb b">|</span>         <span class="eb b">---------</span> <span class="eb b">binding `amsterdam` declared here</span>
<span class="eb b">...</span>
<span class="eb b">10</span> <span class="eb b">|</span>         first_three_of_each(&amp;amsterdam, &amp;denver)
   <span class="eb b">|</span>         <span class="eb b">--------------------</span><span class="er b">^^^^^^^^^^</span><span class="eb b">----------</span>
   <span class="eb b">|</span>         <span class="eb b">|</span>                   <span class="er b">|</span>
   <span class="eb b">|</span>         <span class="eb b">|</span>                   <span class="er b">borrowed value does not live long enough</span>
   <span class="eb b">|</span>         <span class="eb b">argument requires that `amsterdam` is borrowed for `&apos;static`</span>
<span class="eb b">...</span>
<span class="eb b">14</span> <span class="eb b">|</span> }
   <span class="eb b">|</span> <span class="eb b">-</span> <span class="eb b">`amsterdam` dropped here while still borrowed</span>
</code></pre>

## Lifetime annotations help the compiler help you!

* You give Rust hints
* Rust checks memory access for correctness

```rust
fn first_three_of_each<'a, 'b>(s1: &'a str, s2: &'b str) -> (&'a str, &'b str) {
    (&s1[0..3], &s2[0..3])
}

fn main() {
    let amsterdam = format!("AMS Amsterdam");
    let denver = format!("DEN Denver");

    let (amsterdam_code, denver_code) = {
        first_three_of_each(&amsterdam, &denver)
    };

    println!("{} -> {}", amsterdam_code, denver_code);
}
```

## What if multiple parameters can be sources?

```rust ignore
fn pick_one(s1: &'? str, s2: &'? str) -> &'? str {
    if coin_flip() {
        s1
    } else {
        s2
    }
}
```

## What if multiple parameters can be sources?

```rust ignore
fn pick_one<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if coin_flip() {
        s1
    } else {
        s2
    }
}
```

* returned reference *can't outlive* either `s1` or `s2`
* potentially more restrictive

Note:

This function body does not *force* the two inputs to live for the same amount of time. Variables live for as long as they live and we can't change that here. This just says "I'm going to use the same label for the lifetimes these two references have, so pick whichever is the shorter".

## Example

```rust []
fn coin_flip() -> bool { false }

fn pick_one<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if coin_flip() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = String::from("a");
    let b = "b";
    let result = pick_one(&a, b);
    // drop(a);
    println!("{}", result);
}
```

## Lifetime annotations for types

```rust ignore
struct Configuration {
    database_url: &str,
}
```

Where does the string data come from?

## Generic lifetime parameter

```rust ignore
struct Configuration<'a> {
    database_url: &'a str,
}
```
<p>&nbsp;<!-- spacer for "run" button --></p>

* An instance of `Configuration` *can't outlive* a string<br> that it refers to via `database_url`.
* The string *can't be dropped<br> while* an instance of `Configuration` *still* refers to it.

## Lifetimes and Generics

* Lifetime annotations act like generics from type system PoV.
* Can be used to to add bounds to types: `where T: Debug + 'a`
  * Type `T` has to be printable with `:?`.
  * If `T` has references inside, they *have to stay valid for as long as* `'a` tag requires.
* Can be used to match lifetime generics in `struct` or `enum` with the annotations used in function signatures and in turn with exact lifetimes of references.

## Complex example

```rust ignore
fn select_peer<'a>(peers: &[&'a str]) -> Option<Cow<'a, str>> {
    for p in peers {
        if is_up(p) {
            return Some(Cow::Borrowed(p))
        }
    }
    None
}

fn main() {}
```

**Compiler concludes:**

Returned value will not be allowed to outlive any reference in `peers` list

`let selected = select_peer(&peers);`

## Lifetime annotations in practice

* Like generics, annotations make function signatures verbose and difficult to read
  * they often can be glossed over when reading code
* `T: 'static` means "Owned data or static references", owned data can be very short-lived
* Using owned data in your types helps avoid borrow checker difficulties
