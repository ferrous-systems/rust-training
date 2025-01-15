<!-- markdownlint-disable MD031 MD033 MD037 -->
# Deconstructing Send, Arc, and Mutex

## `thread::spawn` Function

```rust ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    // ...
}
```

## Quick Primer on Rust Closures

* 3 categories of data
  * data the closure *closes over* / *captures*: **Upvars**
    * convenient compiler terminology
    * not represented by closure type signature
  * parameters
  * returned value

```rust ignore
let upper_threshold = 20;
let outliers: Vec<_> = data.iter().copied().filter(|n| -> bool {
    // `n` is a parameter, `upper_threshold` is an *upvar*
    n >= upper_threshold
}).collect();
```

## Spawn closure type

* `F: FnOnce() -> T`
  * closure doesn't accept any parameters
  * closure can *consume upvars* ("FnOnce")
* `F: Send + 'static`
  * applies to *upvars*
* `T: Send + 'static`
  * applies to returned value

## `T: 'static`

Two options allowed:

* the type doesn't have any references inside ("Owned data")
  * `struct User { name: String }`
* the references inside the type are `'static`
  * `struct Db { connection_string: &'static str }`

## Why `F: 'static` and `T: 'static`?

* applies to data passed from parent thread to child thread or vice-versa
* prevents passing references to local variables
  * one thread can finish before the other and such references may become invalid
  * `+ 'static` avoids this by ensuring any references point to data that has the static lifetime (i.e. that lives forever)

## `T: Send`

`pub unsafe auto trait Send { }`

* `auto` means all types get this trait automatically
  * opt-out instead of opt-in
* various types in standard library implement `Send` or `!Send`
* `unsafe` means you have to put `unsafe` keyword in front of `impl` when implementing `Send` or `!Send`
  * precautionary measure

## Why would one implement `Send` or `!Send`

* Rust pointers (`*const T`, `*mut T`, `NonNull<T>`) are `!Send`
  * Use-case: what if the pointer comes from FFI library that assumes that all functions using this pointer are called from the same thread?
* `Arc` has a `NonNull<..>` inside and becomes `!Send` automatically
  * to override this behavior `Arc` explicitly implements `Send`

## `Send` in `thread::spawn` Function

`F: Send` and `T: Send` means that all data traveling from the parent thread to child thread has to be marked as `Send`

* Rust compiler has no inherent knowledge of threads, but the use of marker traits and lifetime annotations let the type / borrow checker prevent data race errors

## Sharing data between threads

## Example: Message Log for TCP Echo Server

```rust ignore
use std::{
    io::{self, BufRead as _, Write as _},
    net, thread,
};

fn handle_client(stream: net::TcpStream) -> Result<(), io::Error> {
    let mut writer = io::BufWriter::new(&stream);
    let reader = io::BufReader::new(&stream);
    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "{}", line)?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = net::TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| {
            let _ = handle_client(stream);
        });
    }
    Ok(())
}
```

## Task

* create a log of lengths of all lines coming from all streams
* `let mut log = Vec::<usize>::new();`
* `log.push(line.len());`

## "Dream" API

```rust ignore
fn handle_client(stream: net::TcpStream, log: &mut Vec<usize>) -> Result<(), io::Error> {
    // ...
    for line in ... {
        log.push(line.len());
        // ...
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut log = vec![];

    for stream in listener.incoming() {
        // ...
        thread::spawn(|| {
            let _ = handle_client(stream, &mut log);
        });
    }
    Ok(())
}
```

## Errors

<pre><code data-trim data-noescape><span class="er b">error[E0373]</span><span class="b">: closure may outlive the current function, but it borrows `log`, which is owned by the current function</span>
<span class="eb b">  --&gt; </span>src/main.rs:26:23
<span class="eb b">   |</span>
<span class="eb b">26 |</span>         thread::spawn(|| {
<span class="eb b">   |</span>                       <span class="er b">^^ may outlive borrowed value `log`</span>
<span class="eb b">27 |</span>             let _ = handle_client(stream.unwrap(), &amp;mut log);
<span class="eb b">   |</span>                                                         <span class="eb b">--- `log` is borrowed here</span>
<span class="eb b">   |</span>
<span class="eb b">  --&gt; </span>src/main.rs:26:23
<span class="eb b">   |</span>
<span class="eb b">26 |</span>         thread::spawn(|| {
<span class="eb b">   |</span>                       <span class="er b">^^ may outlive borrowed value `log`</span>
<span class="eb b">27 |</span>             let _ = handle_client(stream.unwrap(), &amp;mut log);
<span class="eb b">   |</span>                                                         <span class="eb b">--- `log` is borrowed here</span>
<span class="eb b">   |</span>
<span class="eg b">note</span>: function requires argument type to outlive `&apos;static`
</code></pre>

## Lifetime problem

Problem:

* local data may be cleaned up prematurely

Solution:

* move the decision when to clean the data from *compile-time* to *run-time*
  * use reference-counting

## Attempt 1: `Rc`

* `let mut log = Rc::new(vec![]);`
* `let mut thread_log = log.clone()` now doesn't clone the data, but simply increases the reference count
  * both variables now have *owned* type, and satisfy `F: 'static` requirement

<pre><code data-trim data-noescape><span class="er b">error[E0277]</span><b>: `Rc<Vec<usize>>` cannot be sent between threads safely</b>
</code></pre>


## `Rc` in Rust Standard Library

* uses `usize` for reference counting
* explicitly marked as `!Send`

```rust ignore
pub struct Rc<T> {
    ptr: NonNull<RcBox<T>>,
}

impl<T> !Send for Rc<T> {}

struct RcBox<T> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}
```

## `Arc` in Rust Standard Library

* uses `AtomicUsize` for reference counting
* explicitly marked as `Send`

```rust ignore
pub struct Arc<T> {
    ptr: NonNull<ArcInner<T>>,
}

impl<T> Send for Arc<T> {}

struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,
    weak: atomic::AtomicUsize,
    data: T,
}
```

## `Rc` vs `Arc`

* `Arc` uses `AtomicUsize` for reference counting
  * slower
  * safe to increment / decrement from multiple threads
* With the help of marker trait `Send` and trait bounds on `thread::spawn`, the compiler *forces* you to use the correct type

## `Arc` / `Rc` "transparency"

```rust ignore
let mut log = Arc::new(Vec::new());
// how does this code work?
log.len();
// and why doesn't this work?
log.push(1);
```

## `Deref` and `DerefMut` traits

```rust ignore
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

## `Deref` coercions

* `Deref` can convert a `&self` reference to a reference of another type
  * conversion function call can be inserted by the compiler for you automatically
  * in most cases the conversion is a no-op or a fixed pointer offset
  * deref functions can be inlined
* `Target` is an associated type
  * can't `deref()` into multiple different types
* `DerefMut: Deref` allows the `DerefMut` trait to reuse the same `Target` type
  * read-only and read-write references coerce to the references of the same type

## `Arc` / `Rc` "transparency" with `Deref`

```rust ignore
let mut log = Arc::new(Vec::new());
// Arc<T> implements `Deref` from `&Arc<T> into `&T`
log.len();
// the same as
Vec::len(<Arc<_> as Deref>::deref(&log));

// Arc<T> DOES NOT implement `DerefMut`
// log.push(1);

// the line above would have expanded to:
// Vec::push(<Arc<_> as DerefMut>::deref_mut(&mut log), 1);
```

## `Arc` and mutability

* lack of `impl DerefMut for Arc` prevents accidental creation of multiple `&mut` to underlying data
* the solution is to move mutability decision to runtime

```rust ignore
let log = Arc::new(Mutex::new(Vec::new()));
```
<p>&nbsp<!-- run-button placeholder --></p>

* `Arc` guarantees *availability* of data in memory
  * prevents memory form being cleaned up prematurely
* `Mutex` guarantees *exclusivity of mutable access*
  * provides *only one* `&mut` to underlying data simultaneously

## `Mutex` in Action

* `log` is passed as `&` and is `deref`-ed from `Arc` by the compiler
* `mut`ability is localized to a local `guard` variable
  * `Mutex::lock` method takes `&self`
* `MutexGuard` implements `Deref` *and* `DerefMut`!
* `'_` lifetime annotation is needed only because guard struct has a `&Mutex` inside

```rust ignore
fn handle_client(..., log: &Mutex<Vec<usize>>) -> ... {
    for line in ... {
        let mut guard: MutexGuard<'_, Vec<usize>> = log.lock().unwrap();
        guard.push(line.len());
        // line above expands to:
        // Vec::push(<MutexGuard<'_, _> as DerefMut>::deref_mut(&mut guard), line.len());
        writeln!(writer, "{}", line)?;
        writer.flush()?;
    }
}
```

## `Mutex` locking and unlocking

* we `lock` the mutex for exclusive access to underlying data at runtime
* old C APIs used a pair of functions to lock and unlock the mutex
* `MutexGuard` does unlocking automatically when is dropped
  * time between guard creation and drop is called *critical section*

## Lock Poisoning

* `MutexGuard` in its `Drop` implementation checks if it is being dropped normally or during a `panic` unwind
  * in later case sets a poison flag on the mutex
* calling `lock().unwrap()` on a poisoned Mutex causes `panic`
  * if the mutex is *"popular"* poisoning can cause many application threads to panic, too.
* `PoisonError` doesn't provide information about the panic that caused the poisoning

## Critical Section "Hygiene"

* keep it short to reduce the window when mutex is locked
* avoid calling functions that can panic
* using a named variable for Mutex guard helps avoiding unexpected temporary lifetime behavior

## Critical Section Example

```rust ignore
fn handle_client(..., log: &Mutex<Vec<usize>>) -> ... {
    for line in ... {
        {
            let mut guard: MutexGuard<'_, Vec<usize>> = log.lock().unwrap();
            guard.push(line.len());
        } // critical section ends here, before all the IO
        writeln!(writer, "{}", line)?;
        writer.flush()?;
    }
}
```
<p>&nbsp<!-- run-button placeholder --></p>

* `drop(guard)` also works, but extra block nicely highlights the critical section

## Lessons Learned

* careful use of traits and trait boundaries lets the compiler detect problematic multi-threading code at compile time
* `Arc` and `Mutex` let the program ensure data availability and exclusive mutability at runtime where the compiler can't predict the behavior of the program
* `Deref` coercions make concurrency primitives virtually invisible and transparent to use
* **Make invalid state unrepresentable**

## Full Example

```rust ignore
use std::{
    io::{self, BufRead as _, Write as _},
    net,
    sync::{Arc, Mutex},
    thread,
};

fn handle_client(stream: net::TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    let mut writer = io::BufWriter::new(&stream);
    let reader = io::BufReader::new(&stream);
    for line in reader.lines() {
        let line = line?;
        {
            let mut guard = log.lock().unwrap();
            guard.push(line.len());
        }
        writeln!(writer, "{}", line)?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let log = Arc::new(Mutex::new(vec![]));
    let listener = net::TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        let thread_log = log.clone();
        thread::spawn(move || {
            let _ = handle_client(stream, &thread_log);
        });
    }
    Ok(())
}
```
