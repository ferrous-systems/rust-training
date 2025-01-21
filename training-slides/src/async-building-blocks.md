# Async Building Blocks

## Async

* Built from important "building blocks"
* Futures, Tasks, Executors, Streams, and more

## Differences between async & sync

* sync programming often has imperative behaviour
* async programming is about constructing a process at runtime and then executing it
* this process is called the "futures tree"

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

## (sketch) Desugaring return type

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

## What are Futures

Futures represent a datastructure that - at some point in the future - give us the value that we are waiting for. The Future may be:

* delayed
* immediate
* infinite

## Futures are operations

Futures are complete operations that can be awaited for.

Examples:

* `read`: Read (up to) a number of bytes
* `read_to_end`: Read a complete input stream
* `connect`: Connect a socket

## Futures are poll-based

They can be checked if they are _done_, and are usually mapped to readiness based APIs like `epoll`.

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
** Generally better latency, no synchronisation requirements
** Highly susceptible to accidental blockades
** Harmed by accidental pre-emption
* Multi-threaded
** Generally better resource use, synchronisation requirements
** Harmed by accidental pre-emption
* Deblocking
** Actively monitor for blocked execution threads and will spin up new ones

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
