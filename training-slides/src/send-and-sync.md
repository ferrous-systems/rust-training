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

<pre><code data-trim data-noescape><span class="er b">error[E0277]</span><b>: `Rc&lt;bool&gt;` cannot be sent between threads safely</b>
<span class="eb b">    --&gt; </span>src/main.rs:7:19
<span class="eb b">     |</span>
<span class="eb b">7    |</span>       thread::spawn(move || {
<span class="eb b">     |</span>       <span class="eb b">-------------</span> <span class="er b">^</span><span class="eb b">------</span>
<span class="eb b">     |</span>       <span class="eb b">|</span>             <span class="er b">|</span>
<span class="eb b">     |</span>  <span class="er b">_____</span><span class="eb b">|</span><span class="er b">_____________</span><span class="eb b">within this `{closure@src/main.rs:7:19: 7:26}`</span>
<span class="eb b">     |</span> <span class="er b">|</span>     <span class="eb b">|</span>
<span class="eb b">     |</span> <span class="er b">|</span>     <span class="eb b">required by a bound introduced by this call</span>
<span class="eb b">8    |</span> <span class="er b">|</span>         println!(&quot;{:?}&quot;, value);
<span class="eb b">9    |</span> <span class="er b">|</span>     }).join().unwrap();
<span class="eb b">     |</span> <span class="er b">|_____^</span> <span class="er b">`Rc&lt;bool&gt;` cannot be sent between threads safely</span>
<span class="eb b">     |</span>
<span class="eb b">     = </span><b>help</b>: within `{closure@src/main.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc&lt;bool&gt;`, which is required by `{closure@src/main.rs:7:19: 7:26}: Send`
<span class="eg">note</span>: required because it&apos;s used within this closure
<span class="eb b">    --&gt; </span>src/main.rs:7:19
<span class="eb b">     |</span>
<span class="eb b">7    |</span>     thread::spawn(move || {
<span class="eb b">     |</span>                   <span class="eg">^^^^^^^</span>
<span class="eg">note</span>: required by a bound in `spawn`
<span class="eb b">    --&gt; </span>/home/mrg/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:675:8
<span class="eb b">     |</span>
<span class="eb b">672  |</span> pub fn spawn&lt;F, T&gt;(f: F) -&gt; JoinHandle&lt;T&gt;
<span class="eb b">     |</span>        <span class="eb b">-----</span> <span class="eb b">required by a bound in this function</span>
<span class="eb b">   ...</span>
<span class="eb b">675  |</span>     F: Send + &apos;static,
<span class="eb b">     |</span>        <span class="eg">^^^^</span> <span class="eg">required by this bound in `spawn`</span>
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
