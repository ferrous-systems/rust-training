# Send & Sync

---

There are two special traits in Rust for concurrency semantics.

-   `Send` marks a structure safe to *send* between threads.
-   `Sync` marks a structure safe to *share* between threads.
    -   (`&T` is `Send`)

---

These traits are what Rust uses to prevent data races.

They are *automatically derived* for all types if appropriate.

## Automatically Derived

```rust
use std::thread;

#[derive(Debug)]
struct Thing;

// Can send between threads!
fn main() {
    let thing = Thing;

    thread::spawn(move || {
        println!("{:?}", thing);
    }).join().unwrap();
}
```

---

There are some notable types which are not `Send` or `Sync`.

Such as `Rc`, raw pointers, and `UnsafeCell`.

## Example: `Rc`

```rust ignore
use std::rc::Rc;
use std::thread;

// Does not work!
fn main() {
    let value = Rc::new(true);
    thread::spawn(move || {
        println!("{:?}", value);
    }).join().unwrap();
}
```

## Example: `Rc`

<pre><code data-trim data-noescape><span style="color:#FF0000"><b>error[E0277]</b></span><b>: `Rc&lt;bool&gt;` cannot be sent between threads safely</b>
   <span style="color:#5C5CFF"><b>--&gt; </b></span>src/main.rs:7:19
    <span style="color:#5C5CFF"><b>|</b></span>
<span style="color:#5C5CFF"><b>7</b></span>   <span style="color:#5C5CFF"><b>|</b></span>       thread::spawn(move || {
    <span style="color:#5C5CFF"><b>|</b></span>       <span style="color:#5C5CFF"><b>-------------</b></span> <span style="color:#FF0000"><b>^</b></span><span style="color:#5C5CFF"><b>------</b></span>
    <span style="color:#5C5CFF"><b>|</b></span>       <span style="color:#5C5CFF"><b>|</b></span>             <span style="color:#FF0000"><b>|</b></span>
    <span style="color:#5C5CFF"><b>|</b></span>  <span style="color:#FF0000"><b>_____</b></span><span style="color:#5C5CFF"><b>|</b></span><span style="color:#FF0000"><b>_____________</b></span><span style="color:#5C5CFF"><b>within this `{closure@src/main.rs:7:19: 7:26}`</b></span>
    <span style="color:#5C5CFF"><b>|</b></span> <span style="color:#FF0000"><b>|</b></span>     <span style="color:#5C5CFF"><b>|</b></span>
    <span style="color:#5C5CFF"><b>|</b></span> <span style="color:#FF0000"><b>|</b></span>     <span style="color:#5C5CFF"><b>required by a bound introduced by this call</b></span>
<span style="color:#5C5CFF"><b>8</b></span>   <span style="color:#5C5CFF"><b>|</b></span> <span style="color:#FF0000"><b>|</b></span>         println!(&quot;{:?}&quot;, value);
<span style="color:#5C5CFF"><b>9</b></span>   <span style="color:#5C5CFF"><b>|</b></span> <span style="color:#FF0000"><b>|</b></span>     }).join().unwrap();
    <span style="color:#5C5CFF"><b>|</b></span> <span style="color:#FF0000"><b>|_____^</b></span> <span style="color:#FF0000"><b>`Rc&lt;bool&gt;` cannot be sent between threads safely</b></span>
    <span style="color:#5C5CFF"><b>|</b></span>
    <span style="color:#5C5CFF"><b>= </b></span><b>help</b>: within `{closure@src/main.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc&lt;bool&gt;`, which is required by `{closure@src/main.rs:7:19: 7:26}: Send`
<span style="color:#00FF00"><b>note</b></span>: required because it&apos;s used within this closure
   <span style="color:#5C5CFF"><b>--&gt; </b></span>src/main.rs:7:19
    <span style="color:#5C5CFF"><b>|</b></span>
<span style="color:#5C5CFF"><b>7</b></span>   <span style="color:#5C5CFF"><b>|</b></span>     thread::spawn(move || {
    <span style="color:#5C5CFF"><b>|</b></span>                   <span style="color:#00FF00"><b>^^^^^^^</b></span>
<span style="color:#00FF00"><b>note</b></span>: required by a bound in `spawn`
   <span style="color:#5C5CFF"><b>--&gt; </b></span>/home/mrg/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:675:8
    <span style="color:#5C5CFF"><b>|</b></span>
<span style="color:#5C5CFF"><b>672</b></span> <span style="color:#5C5CFF"><b>|</b></span> pub fn spawn&lt;F, T&gt;(f: F) -&gt; JoinHandle&lt;T&gt;
    <span style="color:#5C5CFF"><b>|</b></span>        <span style="color:#5C5CFF"><b>-----</b></span> <span style="color:#5C5CFF"><b>required by a bound in this function</b></span>
<span style="color:#5C5CFF"><b>...</b></span>
<span style="color:#5C5CFF"><b>675</b></span> <span style="color:#5C5CFF"><b>|</b></span>     F: Send + &apos;static,
    <span style="color:#5C5CFF"><b>|</b></span>        <span style="color:#00FF00"><b>^^^^</b></span> <span style="color:#00FF00"><b>required by this bound in `spawn`</b></span>
<b>For more information about this error, try `rustc --explain E0277`.</b>
</code></pre>

## Implementing

It's possible to add the implementation of `Send` and `Sync` to a type.

```rust
struct Thing(*mut String);

unsafe impl Send for Thing {}
unsafe impl Sync for Thing {}
```

In these cases, the task of thread safety is left to the implementor.

## Relationships

If a type implements both `Sync` and `Copy` then it can also implement `Send`.

## Relationships

A type `&T` can implement `Send` if the type `T` also implements `Sync`.

```rust ignore
unsafe impl<'a, T: Sync + ?Sized> Send for &'a T {}
```

## Relationships

A type `&mut T` can implement `Send` if the type `T` also implements `Send`.

```rust ignore
unsafe impl<'a, T: Send + ?Sized> Send for &'a mut T {}
```

## Consequences

What are the consequences of having `Send` and `Sync`?

## Consequences

Carrying this information at the type system level allows driving data race bugs down to a *compile time* level.

Preventing this error class from reaching production systems.

`Send` and `Sync` are independent of the choice of concurrency (async, threaded, etc.).
