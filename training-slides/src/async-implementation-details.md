# Async Implementation Details

## Main components of Async/Await

* Tasks
* Wakers
* Executors
* Pin

## Tasks

* Every async function that awaits creates tasks
* Effectively "subtasks" of the function
* Tasks describe dependencies

## Tasks example

```rust, ignore
async fn learn_and_sing() {
    let song = learn_song().await;  // 1
    sing_song(song).await;          // 2
    sing_song(song).await;          // 3
}
```

## Tasks implementation

* Functions become a state machine
* State machines can restart once progress can be made
* Similar to generators

## Wakers

* Polling all the tasks is a busy wait
* Wakers allow us to register a way to wake

## Pin

* All types by default can move
* But what if you want to prevent it
* Pin: underlying memory can not move

## Pin Example

```rust, ignore
#[tokio::main]
async fn main() {
    let mut stream = async_stream();
    let sleep = time::sleep(Duration::from_secs(10));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            maybe_v = stream.next() => {
                if maybe_v.is_none() { break }
                println!("got = {:?}", maybe_v);
            }
            _ = &mut sleep => {
                println!("timeout: 10 secs elapsed");
                break;
            }
        }
    }
}
```

## Executors

* Start with the top-level Futures and drive to completions
* Calls `wake()` when a task can make progress
