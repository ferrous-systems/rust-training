# Dynamic Dispatch

---

Sometimes, we want to take the decision of which implementation to use at runtime instead of letting the compiler monomorphize the code.

There's two approaches.

## Dispatch through Enums

If the number of possible choices is limited, an Enum can be used:

```rust []
enum Operation {
    Get,
    Set(String),
    Count
}

fn execute(op: Operation) {
    match op {
        Operation::Get => { }
        Operation::Set(s) => { }
        Operation::Count => { }
    }
}
```

## Alternative Form

```rust []
enum Operation {
    Get,
    Set(String),
    Count
}

impl Operation {
    fn execute(&self) {
        match &self {
            &Operation::Get => { }
            &Operation::Set(s) => { }
            &Operation::Count => { }
        }
    }
}
```

## Recommendation

Try to minimize repeated matches on the Enum, if not strictly necessary.

## Trait Objects

References or raw pointers on traits, also boxes, describe so-called *trait objects*.

Trait objects are a pair of pointers to a virtual function table and the data.

## Limitations

- You can only use one trait per object
- This trait must fulfill certain conditions

## Rules for object-safe traits (abbreviated)

- Object-safe traits are *not* allowed to require `Self: Sized`
- All methods are object-safe
    * They have no type parameters
    * They don't use `Self`

## Trait Objects and Closures

Closure traits fulfill object safety rules.

```rust []
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}
```

## Further properties

- As trait objects know about their exact type at runtime, they support downcasts through the `Any` trait.

```rust []
use std::fmt::Debug;
use std::any::Any;

// Logger function for any type that implements Debug.
fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;
    match value_any.downcast_ref::<String>() {
        Some(string) => {
            println!("String ({}): {}", string.len(), string);
        }
        None => {
            println!("Not a String: {:?}", value);
        }
    }
}

fn main() {
    log(&"Some message".to_string());
    log(&1)
}
```
