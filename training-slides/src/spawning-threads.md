# Spawning Threads and Scoped Threads

## Platform Differences - Windows

* On Windows, a *Process* is just an address space, and it has one *Thread* by default.
* You can start more *Threads*

```c
HANDLE CreateThread(
  /* [in, optional]  */ LPSECURITY_ATTRIBUTES   lpThreadAttributes,
  /* [in]            */ SIZE_T                  dwStackSize,
  /* [in]            */ LPTHREAD_START_ROUTINE  lpStartAddress,  // <<-- function to run in thread
  /* [in, optional]  */ __drv_aliasesMem LPVOID lpParameter,     // <<-- context for thread function
  /* [in]            */ DWORD                   dwCreationFlags,
  /* [out, optional] */ LPDWORD                 lpThreadId
);
```

## Platform Differences - POSIX

* On POSIX, a *Process* includes one thread of execution.
* You can start more *Threads*, typically using the POSIX Threads API

```c
int pthread_create(
    pthread_t *restrict thread,
    const pthread_attr_t *restrict attr,
    void *(*start_routine)(void *),        // <<-- function to run in thread
    void *restrict arg                     // <<-- context for thread function
);     
```

## Rusty Threads

The Rust [thread API](https://doc.rust-lang.org/std/thread/) looks like this:

```rust ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

## Using spawn

* You *could* pass a function to `std::thread::spawn`.
* In almost all cases you pass a *closure*

```rust []
use std::{thread, time};

fn main() {
    let thread_handle = thread::spawn(|| {
        thread::sleep(time::Duration::from_secs(1));
        println!("I'm a thread");
    });
    
    thread_handle.join().unwrap();
}
```

## Why no context?

There's no `void* p_context` argument, because *closures* can *close-over* local variables.

```rust []
use std::thread;

fn main() {
    let number_of_loops = 5; // on main's stack
    let thread_handle = thread::spawn(move || {
        for _i in 0..number_of_loops { // captured by value, not reference
            println!("I'm a thread");
        }
    });
    
    thread_handle.join().unwrap();
}
```

Note:

Try changing this *move* closure to a regular referencing closure.

## Context lifetimes

However, the thread might live forever...

```rust []
use std::{sync::Mutex, thread};

fn main() {
    let buffer: Mutex<Vec<i32>> = Mutex::new(Vec::new());
    let thread_handle = thread::spawn(|| {
        for i in 0..5 {
            // captured by reference, does not live long enough
            // buffer.lock().unwrap().push(i);
        }
    });
    thread_handle.join().unwrap();
    let locked_buffer = buffer.lock();
    println!("{:?}", &locked_buffer);
}

```

## Making context live forever

If a thread can live forever, we need its context to live just as long.

```rust []
use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let thread_buffer = buffer.clone();
    let thread_handle = thread::spawn(move || {
        for i in 0..5 {
            thread_buffer.lock().unwrap().push(i);
        }
    });
    thread_handle.join().unwrap();
    let locked_buffer = buffer.lock().unwrap();
    println!("{:?}", &locked_buffer);
}
```

## Tidying up the handle

* In Rust, functions take *expressions*
* Blocks are expressions...

```rust ignore
let thread_buffer = buffer.clone();
let thread_handle = thread::spawn(
    move || {
        for i in 0..5 {
            thread_buffer.lock().unwrap().push(i);
        }
    }
);
```

## Tidying up the handle

* In Rust, functions take *expressions*
* Blocks are expressions...

```rust ignore
let thread_handle = thread::spawn({
    let thread_buffer = buffer.clone();
    move || {
        for i in 0..5 {
            thread_buffer.lock().unwrap().push(i);
        }
    }
});
```

Note:

This clearly limits the visual scope of the `thread_buffer` variable, to match the logical scope caused by the fact it is transferred by value into the closure.

## Scoped Threads

As of 1.63, we can say the threads will all have ended before we carry on our calling function.

```rust []
use std::{sync::Mutex, thread};

fn main() {
    let buffer = Mutex::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..5 {
                buffer.lock().unwrap().push(i);
            }
        });
    });
    let locked_buffer = buffer.lock().unwrap();
    println!("{:?}", &locked_buffer);
}
```
