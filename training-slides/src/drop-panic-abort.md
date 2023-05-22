# Drop, panic, and abort

---

What happens in detail when values drop?

## Drop-Order

Rust generally guarantees drop order ([RFC1857](https://github.com/rust-lang/rfcs/issues/1857))

## Drop-Order

- Values are dropped at the end of their scope
- The order is *the reverse introduction order*
- Unbound values drop immediately
- Structure fields are dropped *first to last*

## Destructors

Sometimes, certain actions must be taken before deallocation.

For this, the `Drop` trait can be implemented.

---

```rust [] ignore
struct LevelDB {
    handle: *mut leveldb_database_t
}

impl Drop for LevelDB {
    fn drop(&mut self) {
        unsafe { leveldb_close(self.handle) };
    }
}
```

## Warning!

Destructors cannot return errors.

## Also possible

Explicit destruction of a value through a consuming function. This cannot be statically enforced currently.

Implementing a `Drop`-bomb (a failing destructor) can make sure this error is caught early.

## Panics

Rust also has another error mechanism: `panic!`

```rust [] should_panic
fn main() {
    panicking_function();
}

fn panicking_function() {
    panic!("gosh, don't call me!");
}
```

---

In case of a panic, the following happens:

* The current thread immediately halts
* The stack is unwound
* All affected values are dropped and their destructors run

---

Panics are implementation-wise similar to C++-Exceptions, but should only be used for fatal errors. They cannot be (normally) caught.

The affected thread dies.

## Catching Panics

Panicking across FFI-boundaries is undefined behaviour.
In these cases, panics _must_ be caught.
For cases like this, there are [std::panic::catch-unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) and [std::panic::resume-unwind](https://doc.rust-lang.org/std/panic/fn.resume_unwind.html).

## Hooks

[std::panic::set_hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html) allows setting a global handler that is run *before* the unwinding happens.

---

In general, `Result` is always the right way to propagate errors if they are to be handled.

## Abort

In some environments, unwinding on `panic!` is not very meaningful. For those cases, `rustc` and `cargo` have a switch that immediately aborts the program on panic.

The panic hook is executed.

## Double-panics

Panicking while a panic is being handled - for example in a destructor - invokes undefined behaviour. For that reason, the program will immediately abort.
