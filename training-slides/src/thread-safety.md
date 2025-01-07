# Thread Safety (Send/Sync, Arc, Mutex)

## Rust is thread-safe

But what does that mean?

## An Example in C (or C++)

```c
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

void *thread_function(void *p_arg) {
    int* p = (int*) p_arg;
    for(int i = 0; i < 1000000; i++) {
        *p += 1;
    }
    return NULL;
}

int main() {
    int value = 0;
    pthread_t thread1, thread2;
    pthread_create(&thread1, NULL, thread_function, &value);
    pthread_create(&thread2, NULL, thread_function, &value);
    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);
    printf("value = %d\n", value);
    exit(0);
}
```

## What does that produce...

`1000000 * 2 = 2000000`, right?

```console
$ ./a.out
value = 1059863
```

But there were no compiler errors!

(See <https://godbolt.org/z/41x1dG6oY>)

## Let's try Rust

```rust ignore
fn thread_function(arg: &mut i32) {
    for _ in 0..1_000_000 {
        *arg += 1;
    }
}

fn main() {
    let mut value = 0;
    std::thread::scope(|s| {
        s.spawn(|| thread_function(&mut value));
        s.spawn(|| thread_function(&mut value));
    });
    println!("value = {value}");
}
```

## Oh!

<pre><code data-trim data-noescape><font color="#FF0000"><b>error[E0499]</b></span><b>: cannot borrow `value` as mutable more than once at a time</b>
  <font color="#5C5CFF"><b>--&gt; </b></span>src/main.rs:11:17
   <font color="#5C5CFF"><b>|</b></span>
<font color="#5C5CFF"><b>9</b></span>  <font color="#5C5CFF"><b>|</b></span>     std::thread::scope(|s| {
   <font color="#5C5CFF"><b>|</b></span>                         <font color="#5C5CFF"><b>-</b></span> <font color="#5C5CFF"><b>has type `&amp;&apos;1 Scope&lt;&apos;1, &apos;_&gt;`</b></span>
<font color="#5C5CFF"><b>10</b></span> <font color="#5C5CFF"><b>|</b></span>         s.spawn(|| thread_function(&amp;mut value));
   <font color="#5C5CFF"><b>|</b></span>         <font color="#5C5CFF"><b>---------------------------------------</b></span>
   <font color="#5C5CFF"><b>|</b></span>         <font color="#5C5CFF"><b>|</b></span>       <font color="#5C5CFF"><b>|</b></span>                       <font color="#5C5CFF"><b>|</b></span>
   <font color="#5C5CFF"><b>|</b></span>         <font color="#5C5CFF"><b>|</b></span>       <font color="#5C5CFF"><b>|</b></span>                       <font color="#5C5CFF"><b>first borrow occurs due to use of `value` in closure</b></span>
   <font color="#5C5CFF"><b>|</b></span>         <font color="#5C5CFF"><b>|</b></span>       <font color="#5C5CFF"><b>first mutable borrow occurs here</b></span>
   <font color="#5C5CFF"><b>|</b></span>         <font color="#5C5CFF"><b>argument requires that `value` is borrowed for `&apos;1`</b></span>
<font color="#5C5CFF"><b>11</b></span> <font color="#5C5CFF"><b>|</b></span>         s.spawn(|| thread_function(&amp;mut value));
   <font color="#5C5CFF"><b>|</b></span>                 <font color="#FF0000"><b>^^</b></span>                      <font color="#5C5CFF"><b>-----</b></span> <font color="#5C5CFF"><b>second borrow occurs due to use of `value` in closure</b></span>
   <font color="#5C5CFF"><b>|</b></span>                 <font color="#FF0000"><b>|</b></span>
   <font color="#5C5CFF"><b>|</b></span>                 <font color="#FF0000"><b>second mutable borrow occurs here</b></span>
<b>For more information about this error, try `rustc --explain E0499`.</b>
</code></pre>

It's our old friend/enemy shared mutability!

## How about a `RefCell`...

```rust ignore
fn thread_function(arg: &std::cell::RefCell<i32>) {
    for _ in 0..1_000_000 {
        let mut p = arg.borrow_mut();
        *p += 1;
    }
}

fn main() {
    let mut value = std::cell::RefCell::new(0);
    std::thread::scope(|s| {
        s.spawn(|| thread_function(&value));
        s.spawn(|| thread_function(&value));
    });
    println!("value = {}", value.borrow());
}
```

## Oh come on...

<pre><code data-trim data-noescape><font color="#FF0000"><b>error[E0277]</b></span><b>: `RefCell&lt;i32&gt;` cannot be shared between threads safely</b>
   <font color="#5C5CFF"><b>--&gt; </b></span>src/main.rs:11:17
    <font color="#5C5CFF"><b>|</b></span>
<font color="#5C5CFF"><b>11</b></span>  <font color="#5C5CFF"><b>|</b></span>         s.spawn(|| thread_function(&amp;value));
    <font color="#5C5CFF"><b>|</b></span>           <font color="#5C5CFF"><b>-----</b></span> <font color="#FF0000"><b>^^^^^^^^^^^^^^^^^^^^^^^^^^</b></span> <font color="#FF0000"><b>`RefCell&lt;i32&gt;` cannot be shared between threads safely</b></span>
    <font color="#5C5CFF"><b>|</b></span>           <font color="#5C5CFF"><b>|</b></span>
    <font color="#5C5CFF"><b>|</b></span>           <font color="#5C5CFF"><b>required by a bound introduced by this call</b></span>
    <font color="#5C5CFF"><b>|</b></span>
    <font color="#5C5CFF"><b>= </b></span><b>help</b>: the trait `Sync` is not implemented for `RefCell&lt;i32&gt;`, which is required by `{closure@src/main.rs:11:17: 11:19}: Send`
    <font color="#5C5CFF"><b>= </b></span><b>note</b>: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    <font color="#5C5CFF"><b>= </b></span><b>note</b>: required for `&amp;RefCell&lt;i32&gt;` to implement `Send`
<font color="#00FF00"><b>note</b></span>: required because it&apos;s used within this closure
   <font color="#5C5CFF"><b>--&gt; </b></span>src/main.rs:11:17
    <font color="#5C5CFF"><b>|</b></span>
<font color="#5C5CFF"><b>11</b></span>  <font color="#5C5CFF"><b>|</b></span>         s.spawn(|| thread_function(&amp;value));
    <font color="#5C5CFF"><b>|</b></span>                 <font color="#00FF00"><b>^^</b></span>
<font color="#00FF00"><b>note</b></span>: required by a bound in `Scope::&lt;&apos;scope, &apos;env&gt;::spawn`
   <font color="#5C5CFF"><b>--&gt; </b></span>/home/mrg/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/scoped.rs:196:28
    <font color="#5C5CFF"><b>|</b></span>
<font color="#5C5CFF"><b>194</b></span> <font color="#5C5CFF"><b>|</b></span>     pub fn spawn&lt;F, T&gt;(&amp;&apos;scope self, f: F) -&gt; ScopedJoinHandle&lt;&apos;scope, T&gt;
    <font color="#5C5CFF"><b>|</b></span>            <font color="#5C5CFF"><b>-----</b></span> <font color="#5C5CFF"><b>required by a bound in this associated function</b></span>
<font color="#5C5CFF"><b>195</b></span> <font color="#5C5CFF"><b>|</b></span>     where
<font color="#5C5CFF"><b>196</b></span> <font color="#5C5CFF"><b>|</b></span>         F: FnOnce() -&gt; T + Send + &apos;scope,
    <font color="#5C5CFF"><b>|</b></span>                            <font color="#00FF00"><b>^^^^</b></span> <font color="#00FF00"><b>required by this bound in `Scope::&lt;&apos;scope, &apos;env&gt;::spawn`</b></span>
<b>For more information about this error, try `rustc --explain E0277`.</b>
</code></pre>

## What is Send?

* It is a marker trait with no methods
* We use it to mark types which are safe to *send between threads*

```rust ignore
pub unsafe auto trait Send { }
```

## What is Sync?

* It is a marker trait with no methods
* We use it to mark types where it is safe to *send their references between threads*
* A type `T` is `Sync` if and only if `&T` is `Send`

```rust ignore
pub unsafe auto trait Sync { }
```

## Is there a `Sync` version of `RefCell`?

Yes, several - and the error message suggested one: [`std::sync::RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html).

There's also the slightly simpler [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html).

## Using a `Mutex`

```rust
fn thread_function(arg: &std::sync::Mutex<i32>) {
    for _ in 0..1_000_000 {
        let mut p = arg.lock().unwrap();
        *p += 1;
    }
}

fn main() {
    let value = std::sync::Mutex::new(0);
    std::thread::scope(|s| {
        s.spawn(|| thread_function(&value));
        s.spawn(|| thread_function(&value));
    });
    println!("value = {}", value.lock().unwrap());
}
```

## Why the `unwrap`?

* The `Mutex` is locked on `lock()`
* It is unlocked when the value returned from `lock()` is *dropped*
* What if you `panic!` whilst holding the lock?
* -> The next `lock()` will return `Err(...)`
* You can basically ignore it (the panic is a bigger issue...)

## What about `Rc<T>`?

That's not thread-safe either. Use `std::sync::Arc<T>`.

```rust
fn thread_function(arg: &std::sync::Mutex<i32>) {
    for _ in 0..1_000_000 {
        let mut p = arg.lock().unwrap();
        *p += 1;
    }
}

fn main() {
    let value = std::sync::Arc::new(std::sync::Mutex::new(0));
    let t1 = std::thread::spawn({
        let value = value.clone();
        move || thread_function(&value)
    });
    let t2 = std::thread::spawn({
        let value = value.clone();
        move || thread_function(&value)
    });
    let _ = t1.join();
    let _ = t2.join();
    println!("value = {}", value.lock().unwrap());
}
```

## Atomic Values

* Locking things is fairly ... heavyweight
* Are there integers which just *work* when used across threads?
* ... which just support shared mutability?
* Yes: See <https://doc.rust-lang.org/std/sync/atomic>

## Methods on Atomics

* We have `AtomicBool`, `AtomicPtr`, and 10 sizes of Atomic integer
* `load()` and `store()`
* `fetch_add()` and `fetch_sub()`
* `compare_exchange()`
* etc

Note:

* `load` and `store` work as expected
* `fetch_add` will add a value to the atomic, and return its old value
* `fetch_sub` will subtract a value from the atomic, and return its old value
* `compare_exchange` will swap an atomic for some new value, provided it is currently equal to some given existing value
* All these functions require an `Ordering`, which explains whether you are only concerned about *this* value, or other operations in memory which should happen *before* or *after* this atomic access; e.g. when taking a lock.

## An Example

We *highly* recommend ["Rust Atomics and Locks" by Mara Bos](https://marabos.nl/atomics) for further details.

```rust
use std::sync::atomic::{Ordering, AtomicI32};

fn thread_function(arg: &AtomicI32) {
    for _ in 0..1_000_000 {
        arg.fetch_add(1, Ordering::Relaxed);
    }
}

fn main() {
    let value = AtomicI32::new(0);
    std::thread::scope(|s| {
        s.spawn(|| thread_function(&value));
        s.spawn(|| thread_function(&value));
    });
    println!("value = {}", value.load(Ordering::Relaxed));
}
```
