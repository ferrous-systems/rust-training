# Deconstructing `thread::scope`

## `thread::scope` Example

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = data.split_at(data.len() / 2);
    let (mut left_sum, mut right_sum) = (0, 0);
    std::thread::scope(|s| {
        s.spawn(|| {
            left_sum = left.iter().sum();
        });
        s.spawn(|| {
            right_sum = right.iter().sum();
        });
    });
    println!("Total: {}", left_sum + right_sum);
}
```

## `thread::scope`

```rust ignore
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
{
    // ...
}
```

we will call `f` a **Spawner** closure

## `Scope`

```rust ignore
pub struct Scope<'scope, 'env: 'scope> {
    data: Arc<ScopeData>,
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}
```

## `Scope::spawn`

```rust ignore
impl<'scope, 'env> Scope<'scope, 'env> {
    pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        // ...
    }
}
```

we'll call `f` a **Thread** closure

## Closures

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = data.split_at(data.len() / 2);
    let (mut left_sum, mut right_sum) = (0, 0);
    std::thread::scope(|s| { // <- Spawner closure
        s.spawn(|| { // <- Thread closure
            left_sum = left.iter().sum();
        });
        s.spawn(|| { // <- Thread closure
            right_sum = right.iter().sum();
        });
    });
    println!("Total: {}", left_sum + right_sum);
}
```

## Meet lifetime annotations

* `F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T`
  * `'scope` is a *Higher-Rank Trait Bound* - it describes *all possible* lifetimes that closure can observe
* `fn scope<'env, F, T>(f: F) -> T`
  * the function observes *some* lifetime `'env`
* `struct Scope<'scope, 'env: 'scope>`
  * lifetime `'env` outlives `'scope`

## Relationship 1

All possible `'scope` lifetimes for the **Spawner** closure *cannot outlive* `'env` lifetime that `scope` function observes.

Upvars with references in **Spawner** closure *cannot outlive* data referenced by `'env` annotation

## Thread closure

```text
F: FnOnce() -> T + Send + 'scope,
T: Send + 'scope
```

* Data passed to and from the child thread
  * should be `Send`
  * if has references to surrounding data, they should stay valid *at least* for the whole duration of `'scope`
    * `&'scope Scope` in **Spawner** signature allows nested calls to `spawn`!

## Relationship 2

By the time `'scope` is over all calls to `Scope::spawn` are over and all **Thread** closures are completed.

## Practical implications

* **Spawner** closure can finish earlier that **Thread** closures.
  * **Thread** closures can't use upvars from **Spawner** without moving them
* both **Spawner** and all **Thread** closures are completed before the call to `scope` function returns
  * both can take upvars from the code before `scope()` call without moving

## Relationships in action

```rust ignore
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = data.split_at(data.len() / 2);
    let mut left_sum = 0; // Ok
    std::thread::scope(|s| {
        s.spawn(|| {
            left_sum = left.iter().sum();
        });
        // ERROR: closure may outlive the current function
        let mut right_sum = 0;
        s.spawn(|| {
            right_sum = right.iter().sum();
        });
    });
    // println!("Total: {}", left_sum + right_sum);
}
```

## Using `join`

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (left, right) = data.split_at(data.len() / 2);
    let mut left_sum = 0;
    let right_sum = std::thread::scope(|s| {
        s.spawn(|| {
            left_sum = left.iter().sum();
        });

        let handle = s.spawn(|| {
            right.iter().sum()
        });
        let right_sum: i32 = handle.join().unwrap();
        right_sum
    });
    println!("Total: {}", left_sum + right_sum);
}
```

## How `scope` function waits for all threads to finish?

```rust ignore
pub fn scope<'env, F, T>(f: F) -> T {
    let scope = Scope {
        data: Arc::new(ScopeData {
            num_running_threads: AtomicUsize::new(0),
            main_thread: current(),
            a_thread_panicked: AtomicBool::new(false),
        }),
        // ...
    };

    let result = catch_unwind(AssertUnwindSafe(|| f(&scope)));

    // Wait until all the threads are finished.
    while scope.data.num_running_threads.load(Ordering::Acquire) != 0 {
        park();
    }
    // ...
}
```

## (1) Who calls `unpark()`?

```rust ignore
impl ScopeData {
    pub(super) fn decrement_num_running_threads(&self, panic: bool) {
        // ...
        // fetch_sub returns the previous value
        if self.num_running_threads.fetch_sub(1, Ordering::Release) == 1 {
            self.main_thread.unpark();
        }
    }
}
```

## (2) Who calls `unpark()`?

```rust ignore
impl<'scope, T> Drop for Packet<'scope, T> {
    fn drop(&mut self) {
        // ...
        if let Some(scope) = &self.scope {
            scope.decrement_num_running_threads(/* ... */);
        }
    }
}
```

`Packet` is a mechanism to pass panics and results from a thread closure to the parent thread.

## `thread::scope()`

* function completion is controlled by a single atomic counter in an `Arc`
* local read-only data can be safely shared across threads
* locking is only needed for safe mutable access
* access rules checked at compile time by the type system

`thread::spawn` - safe concurrency
`thread::scope` - safe *ergonomic* concurrency

## When to use what?

1. `thread::scope` is your *default* choice
2. `thread::spawn` for threads that run forever (background jobs, resource management, etc.)
3. `thread::spawn` for short-living threads that don't have a definite join point (fire and forget)

## TCP server with `thread::spawn`

```rust ignore
fn handle_client(stream: net::TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    // ...
}

fn main() -> Result<(), io::Error> {
    // Need an Arc to control resource availability at runtime
    let log = Arc::new(Mutex::new(vec![]));
    let listener = net::TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        // `Arc`s need explicit cloning
        let thread_log = log.clone();
        thread::spawn(move || {
            let _ = handle_client(stream, &thread_log);
        });
    }
    Ok(())
}
```

## TCP server with `thread::scope`

```rust ignore
fn handle_client(stream: net::TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    // ...
}

fn main() -> Result<(), io::Error> {
    // The compiler can deduce the availability at compile time
    // No need for runtime reference counting
    let log = Mutex::new(vec![]);
    let listener = net::TcpListener::bind("0.0.0.0:7878")?;

    thread::scope(|s| {
        for stream in listener.incoming() {
            let stream = stream?;
            // resources are shareable as is
            s.spawn(|| {
                let _ = handle_client(stream, &log);
            });
        }
        Ok(())
    })
}
```

## Further Research

* returned value propagation from child threads to the parent thread
* panic propagation
* covariance `scope: PhantomData<&'scope mut &'scope ()>`
