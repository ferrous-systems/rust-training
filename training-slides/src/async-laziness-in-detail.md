# Async/Await: Laziness in detail (Rust vs other languages)


## Example: JavaScript

* Eager execution model
* Other languages behave similarly (C#, Python, Swift, Kotlin, etc.)
* JavaScript calls its futures "Promises"

[.columns.is-vcentered]
## Rust Futures vs JavaScript Promises

```rust [], ignore
async fn subtask() {
    println!("> > subtask"); // <4>
}

async fn task() {
    println!("> Before subtask"); // <3>
    subtask().await;
    println!("> After subtask"); // <5>
}

fn main() {
    futures::executor::block_on(async {
        println!("before future"); // <1>
        let future = task();
        println!("future is created"); // <2>
        future.await;
        println!("future is awaited"); // <6>
    });
}
```

```js [], ignore

```

[.columns.is-vcentered]
## Eager Execution (JavaScript and many other languages)

[.column]
[source,javascript]
----
include::./futures-vs-promises.js[]
----

```dot process
sequenceDiagram
  participant main
  participant task
  participant subtask
  Note right of main: console.log("before promise")
  main->>task: let promise = task()
  Note right of task: console.log("> before subtask")
  task->>subtask: subtask()
  Note right of subtask: console.log("> > subtask")
  subtask-->>task: a promise is prepared
  task-->>main: a promise is prepared
  Note right of main: console.log("promise is created")
  main->>task: await
  task->>subtask: await
  subtask-->>task: return
  Note right of task: console.log("> after subtask")
  task-->>main: return
  Note right of main: console.log("promise is awaited")
```

[.columns.is-vcentered]
## Eager Execution (JavaScript and many other languages)

[.column]
[source,javascript]
----
include::./futures-vs-promises.js[]
----

++++
<div class="column">
++++
Output: JavaScript
----
before promise

> before subtask

> > subtask

promise is created

> after subtask

promise is awaited
----
++++
</div>
++++

## Eager Execution: Takeaways

* as soon as async function is called it starts executing
* runs till the first `await` point
* inner async functions run their code, too, and stop at `await`
* an async function with no `await` inside will execute its full body eagerly

[.columns.is-vcentered]
## Lazy Execution: Rust Futures

[.column]
[source,rust]
----
include::./futures_vs_promises.rs[]
----

[.column]
[mermaid, format=svg]
....
sequenceDiagram
  participant main
  participant task
  participant subtask
  Note right of main: println!("before future")
  Note right of main: let future = task()
  Note right of main: println!("future is created")
  main->>task: await
  Note right of task: println!("> Before subtask")
  Note right of task: subtask()
  task->>subtask: await
  Note right of subtask: println!("> > subtask")
  subtask-->>task: return
  Note right of task: println!("> After subtask")
  task-->>main: return
  Note right of main: println!("future is awaited")
....

[.columns.is-vcentered]
## Lazy Execution: Rust Futures

[.column]
[source,rust]
----
include::./futures_vs_promises.rs[]
----

++++
<div class="column">
++++
Output: Rust
----
before future

future is created

> Before subtask

> > subtask

> After subtask

future is awaited
----
++++
</div>
++++

[.columns.is-vcentered]
## Rust Futures vs JavaScript Promises

[.column]
[source,rust]
----
include::./futures_vs_promises.rs[]
----

++++
<div class="column">
++++
Output: Rust
----
before future

future is created

> Before subtask

> > subtask

> After subtask

future is awaited
----
Output: JavaScript
----
before promise

> before subtask

> > subtask

promise is created

> after subtask

promise is awaited
----
++++
</div>
++++

[.column]
[source,javascript]
----
include::./futures-vs-promises.js[]
----

## Lazy Future Execution: Takeaways

* no code is being run until a future is `await` ed
* `await` triggers the whole chain to execute
* no "fire now, `await` later" workflow
