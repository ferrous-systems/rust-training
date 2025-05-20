# Safety, Performance and Productivity

## 1) Safety

## Rust is *memory-safe*

* Every value has one owner
* You can create either:
  * One exclusive, mutable, reference
  * Multiple shared, immutable, references
  * Never both!
* These rules are checked at compile time
  * Or at run-time if you choose
* Rust applies bounds checks to array and slice accesses
  * Where possible (e.g. the indices are constant) those checks are optimized out

## Index Example

```rust
fn process(items: &mut [i32]) {
    items[10] = 6;
}
```

---

If `items` isn't long enough, this raises a run-time panic instead of corrupting
memory.

## Iter Example

```rust ignore []
/// Adds 0x00 padding for every 0xCC found
fn process(data: &mut Vec<u8>) {
    for item in data.iter_mut() {
        if *item == 0xCC {
            data.push(0);
        }
    }
}
```

---

Rust won't let you modify the `Vec<u8>` whilst you iterate through it - this
breaks the rules around *exclusive borrows*.

Note:

This is trivial to do in C++ and causes silent corruption.

## Iter Example (fixed)

```rust
/// Adds 0x00 padding for every 0xCC found
fn process(data: &mut Vec<u8>) {
    let padding_byte_count = data.iter().filter(|&&x| x == 0xCC).count();
    for _ in 0..padding_byte_count {
        data.push(0);
    }
}
```

## Rust is *thread-safe*

* Types must be marked as safe for:
  * Transferring ownership between threads, and/or
  * Transferring a reference between threads
* You cannot create race-hazards!

## APIs can reason about thread-safety

* Rust *channels* require types to be marked as thread-safe
* Passing values when starting a spawned thread - same checks
* The ref-counting allocation type `Rc<T>` __is not__ thread-safe
* The __atomic__-ref-counting allocation type `Arc<T>` __is__ (but is slightly slower)
* Make the wrong choice? Compiler stops you!

## Thread Example

```rust ignore
fn main() {
    let mut total = 0;
    for _ in 0..10 {
        std::thread::spawn(|| {
            total += 1;
        });
    }
    println!("{total}");
}
```

Note:

* Failure 1 - threads can live forever, but they are trying to borrow a variable
  on the stack of the main function
* Failure 2 - multiple threads trying to take mutable (exclusive) access to a
  variable

## Thread Example (Fixed)

```rust
use std::sync::atomic::{AtomicU32, Ordering};
fn main() {
    let total = AtomicU32::new(0);
    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| total.fetch_add(1, Ordering::Relaxed));
        }
    });
    println!("{}", total.load(Ordering::Relaxed));
}
```

## There's an escape hatch

* Where the compiler cannot verify the rules are upheld, you can tell it you've
  done the checks manually
* We create `unsafe { }` blocks and `unsafe fn` functions
* Lets you access raw pointers (e.g. for memory-mapped I/O)
* When you audit/review the code, you pay close attention to these parts!

## 2) Performance

## A Comparison

Let's use Python to calculate the sum of the cubes of the first 100 million integers.

```python
import datetime
start = datetime.datetime.now()
cube_sum = sum(
    map(
        lambda x: x * x * x,
        range(0, 100_000_000)
    )
)
print(f"Took {datetime.datetime.now() - start}")
print(f"cube_sum = {cube_sum}")
```

```text
>>> run()
Took 0:00:09.076986
24999999500000002500000000000000
```

## In Rust?

```rust
fn main() {
    let start = std::time::Instant::now();
    let sum: u128 = (0..100_000_000u32)
        .into_iter()
        .map(|n| {
            let n = u128::from(n);
            n * n * n
        })
        .sum();
    println!("Took {:?}", start.elapsed());
    println!("sum = {sum}");
}
```

```console
$ cargo run --release
   Compiling process v0.1.0 (/Users/jonathan/process)
    Finished release [optimized] target(s) in 0.34s
Took 45ns
sum = 24999999500000002500000000000000
```

## OK, but it's cheating

```rust
fn main() {
    let start = std::time::Instant::now();
    let sum: u128 = (0..100_000_000u32)
        .into_iter()
        .map(|n| {
            let n = u128::from(n);
            std::hint::black_box(n * n * n)
        })
        .sum();
    println!("Took {:?}", start.elapsed());
    println!("sum = {sum}");
}
```

```console
$ cargo run --release
   Compiling process v0.1.0 (/Users/jonathan/process)
    Finished release [optimized] target(s) in 0.34s
Took 68.014583ms
sum = 24999999500000002500000000000000
```

## Let's use all our CPU cores...

```rust ignore []
// Import the rayon library
use rayon::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    // Swap `into_iter` for `into_par_iter`
    let sum: u128 = (0..100_000_000u32)
        .into_par_iter()
        .map(|n| {
            let n = u128::from(n);
            std::hint::black_box(n * n * n)
        })
        .sum();
    println!("Took {:?}", start.elapsed());
    println!("sum = {sum}");
}
```

## Let's use all our CPU cores...

```console
$ cargo add rayon
    Updating crates.io index
      Adding rayon v1.6.1 to dependencies.
$ cargo run --release
...
   Compiling rayon v1.6.1
   Compiling process v0.1.0 (/Users/jonathan/process)
    Finished release [optimized] target(s) in 2.38s
     Running `target/release/process`
Took 9.928125ms
sum = 24999999500000002500000000000000
```

## Sure, but C can do this too, right?

```console
$ clang -o ./target/main src/main.c -O3 -mcpu=native -std=c17 && ./target/main
sum 0x13b8b5ae675d38cb7260b704000
Took 70.3 milliseconds
```

## And was getting that performance ... enjoyable?

```c []
#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>
#include <time.h>

int main(int argc, char** argv) {
    uint64_t start = clock_gettime_nsec_np(CLOCK_MONOTONIC);
    __uint128_t x = 0;
    for(uint32_t idx = 0; idx < 100000000; idx++) {
        __uint128_t i = (__uint128_t) idx;
        volatile __uint128_t result = i * i * i;
        x += result;
    }
    uint64_t end = clock_gettime_nsec_np(CLOCK_MONOTONIC);
    printf("sum 0x%08llx%08llx\n", (unsigned long long) (x >> 64), (unsigned long long) x);
    printf("Took %.3g milliseconds\n", ((double) (end - start)) / (1000.0 * 1000.0) );
    return 0;
}
```

## 3) Productivity

## libstd

<div class="columns">
<div>

* Filesystem access and Path handling
* Heap allocation, with optional reference-counting
* Threads, with Mutexes, Condition Variables, and Channels
* Strings, and a powerful value formatting system
* Growable arrays, hash-tables, B-Trees

</div>
<div>

* First-class Unicode text support
* Networking support (IPv4/IPv6, TCP/UDP, etc)
* I/O traits for working with files, strings, sockets, etc
* Time handling: Duration and Instant
* Environment Variables and CLI arguments

</div>
</div>

## Much less time chasing down weird bugs

* *If it compiles, it'll probably work right*
* No data races across threads
* No double frees, buffer overflows

## Async Programming

* Third-party libraries (e.g. *tokio*) give you all that but with an asynchronous API
* Great if your code spends a lot of time *waiting* (for the disk, for the network)

## Tools like `rust-analyzer` have powerful auto-completion

* Filling in functions to meet a trait definition
* Covering all the arms in a match expression
* Importing modules or qualifying a given type

## Built in testing

* The test-runner compiles and runs:
  * All your unit tests
  * All your integration tests
  * All the code examples in your docs!
* It also compiles all your examples

## It's completely cross-platform

* Windows, Linux and macOS devs all working with the *same tools*
* You can build stand-alone binaries that are [trivial to deploy](https://github.com/axodotdev/cargo-dist/releases)
