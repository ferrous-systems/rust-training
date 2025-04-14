# Async Component Interaction

## Blocking

Blocking is an overloaded term

* Blocking API: an API that _might_ force pre-emption
* Blocked Task: A task that runs for too long

## Dealing with blocking in practice

* Blocking APIs are generally faster
* Determining if a task really blocks is hard

It's hard to determine for a full program if _all instances of a task are staying under a certain max execution time_.

## `spawn_blocking`

* `spawn_blocking` is usually the solution for dealing with slightly longer tasks

```rust [], ignore
task::spawn_blocking(async {
    std::thread::sleep(Duration::from_secs(1000));
});
```

## Solution

* Separation of async and sync parts for benchmarking
* Runtime monitoring, mostly through tracing.

## Component interaction with channels

* Channels allow communication between tasks
* This allows weak binding between components
* All channels work through Ownership

## Threading vs. async

* Threading can be a lot faster in high-throughput situations
* Threading deschedules automatically if threads run out of their timeslice
* Async makes it much cheaper to hold slow and sleepy connections
* Async is very good in reactive models

## Models

* Full async
* async at the edge
* Multiple reactors

## Example

```rust [], ignore
let (s, r) = mpsc::channel(32);

assert_eq!(s.send("Hello").await, Ok(()));
assert_eq!(r.recv().await, Ok("Hello"));
```

## Classes of channels

* Bounded
* Unbounded
* Single Producer, Single Consumer (SPSC)
* Multiple Producers, Single Consumer (MPSC)
* Multiple Producers, Multiple Consumers (MPMC)
* One-Shot

## Strategy

* Pick a default one, preferably MPMC.
* Be liberal in using others when needed.

## Synchronisation and Locking: Warning

* Avoid std::sync types - they preempt
* There's a `async_std::sync` module with API equivalents

## Synchronisation and Locking

* Pick types based on your usage pattern
* e.g. RWLocks if Writes are common and reads rare
* Mutex for other situations
* Fairness comes into play here

## Channels as synchronisation methods

* Channels act as a natural synchronisation method, as they are read from 1 by 1.

## Fairness and starvation

* Fairness describes the property of a combinator to make sure that every side is equally served. If one is not, it may starve.
