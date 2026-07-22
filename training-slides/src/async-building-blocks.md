# Async Building Blocks

## Async

* Built from important building blocks, which are partially baked into the language
* Futures, Tasks, Executors, Streams, and more

## Differences between async & sync

* sync programming often has imperative behaviour
* async programming is an abstraction that allows developers to define pausing points where
  the execution can be paused and later resumed

Note:

- Imperative: Statements in synchronous code are just executed step-by-step.

## Async language support

* `async` functions can define yield points where the execution can be paused, represented by
  `await`ion points.
* Built into the language: The compiler generates the state machines required to do this
  automatically.

## An async Rust function

```rust [], ignore
use tokio::{fs::File, io::AsyncReadExt};

async fn read_from_disk(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
```

Note:

- function must be `async` so `await` can be used inside it.
- The code between two `await`ion points has a regular synchronous flow.

## (sketch) Desugaring the return type

```rust [], ignore
use std::future::Future;

use tokio::{fs::File, io::AsyncReadExt};

fn read_from_disk<'a>(path: &'a str)
   -> impl Future<Output = std::io::Result<String>> + 'a
{
    async move {
        let mut file = File::open(path).await?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).await?;
        Ok(buffer)
    }
}
```

Note:

- Helpful mental model: A `async` function is a regular synchronous functions that returns a `Future`.

## What are Futures

Futures represent a datastructure that - at some point in the future - give us the value that we
are waiting for. The Future may be:

* delayed
* immediate
* infinite

## Futures are operations

Futures are complete operations that can be awaited for.

Examples:

* `read`: Read (up to) a number of bytes
* `read_to_end`: Read a complete input stream
* `connect`: Connect a socket

Note:

- Alternative wording: A future can be resolved or polled to completion.

## Futures are poll-based

They can be checked if they are _done_, and are usually mapped to readiness based APIs:

- On full operating systems, mechanisms like `epoll` are used
- On embedded systems, interrupts are used to advance or complete operations

## .await registers interest in completion

```rust [], ignore
use tokio::{fs::File, io::AsyncReadExt};

async fn read_from_disk(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
```

Note:

- Reminder: `await` are the yield/pause points where the execution might be paused and later
  resumed

## Futures are cold

```rust [], ignore
fn main() {
    let read_from_disk_future = read_from_disk();
}
```

## Futures need to be executed

```rust [], ignore
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let read_from_disk_future = read_from_disk("Cargo.toml");

    let result = async {
        let task = tokio::task::spawn(read_from_disk_future);
        task.await
    }
    .await;

    println!("{:?}", result);
}

async fn read_from_disk(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).await?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
```

## Tasks

* A task connects a future to the executor
* _The task is the concurrent unit_!
* A task is similar to a thread, but is user-space scheduled

## Futures all the way down: Combining Futures

```rust [], ignore
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let read_from_disk_future = read_from_disk("Cargo.toml");

    let timeout = Duration::from_millis(1000);
    let timeout_read = tokio::time::timeout(timeout, read_from_disk_future);

    let result = async {
        let task = tokio::task::spawn(timeout_read);
        task.await
    }
    .await;

    println!("{:?}", result);
}
```

## Ownership/Borrowing Memory in concurrent systems

* Ownership works just like expected - it flows in and out of tasks/futures
* Borrows work over `.await` points
  * This means: All owned memory in a Future _must remain at the same place_
* Sharing between tasks is often done using `Rc/Arc`

## Categories of Executors

* Single-threaded
  * Generally better latency, no synchronisation requirements
  * Highly susceptible to accidental blockades
  * Harmed by accidental pre-emption
* Multi-threaded
  * Generally better resource use, synchronisation requirements
  * Harmed by accidental pre-emption
* Deblocking
  * Actively monitor for blocked execution threads and will spin up new ones

## Reference Counting

* Reference counting on single-threaded executors can be done using `Rc`
* Reference counting on multi-threaded executors can be done using `Arc`

## Streams

* Streams are async iterators
* They represent _potentially infinite arrivals_
* They cannot be executed, but operations on them are futures

## Classic Stream operations

* iteration
* merging
* filtering

## Async iteration

```rust [], ignore
while let Some(item) = stream.next().await {
    //...
}
```
