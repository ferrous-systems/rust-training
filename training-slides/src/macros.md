# Macros

## What can macros do?

Macros can be used to things such as:

* Generate repetitive code
* Create *Domain-Specific Languages* (or *DSLs*)
* Write things that would otherwise be hard without Macros

## There are two kinds of macro

* Declarative
* Procedural

# Declarative Macros

## Declarative Macros

* Defined using `macro_rules!`
* Perform pattern matching and substitution
* Can do repeated actions

## Declarative Macros are:

* __Hygienic__: expansion happens in a different 'syntax context'
* __Correct__: they cannot expand to invalid code
* __Limited__: they cannot, for example, pollute their expansion site

## The `vec!` macro

```rust [1-10|2-3|4-11]
fn main() {
    // You write:
    let v = vec![1, 2, 3];
    // The compiler sees (roughly):
    let v = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };
}
```

## How does that work?

"Match zero or more expressions, and paste each into into a `temp_vec.push()` call"

```rust [1-12|1|2|3|4-10|6-8]
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

Note:

The actual macro is more complicated as it sets the `Vec` to have the correct capacity up front, to avoid re-allocation during the pushing of the values. Any new variables we introduce are given a *colour* to distinguish them from any the caller had created in the same scope.

## `println!` and friends

`println!` is a macro, because:

* Rust does not have variadic functions
* Rust wants to type-check the call

## Expanding `println!`

```rust [] ignore
fn main() {
    // You write
    println!("Hello {}, aged {}", "Sam", 40);
    // The compiler sees (roughly):
    let arguments = Arguments {
        pieces: &["Hello ", ", aged ", "\n"],
        args: &[
            Argument { value: &"Sam", formatter: string_formatter },
            Argument { value: &40, formatter: integer_formatter },
        ],
    };
    ::std::io::_print(arguments);
}
```

Note:

This is a simplified example - the real output is slightly more complicated, and is in fact handled by a *compiler built-in* so you can't even see the macro source for yourself.

## Downsides of Declarative Macros

* Can be difficult to debug
* Can be confusing to read and understand

## When Should You Use Declarative Macros?

* When there are no other good alternatives

# Procedural macros

## Procedural macros

* A procedural macro is a function that takes some code as input, and produces some code.
* It runs at *compile time*
* It is written in Rust and must therefore be *compiled* before your program is

## Three kinds of procedural macro

* Custom `#[derive]` macros
* Attribute-like macros
* Function-like macros

## Custom `#[derive]` macros

Work like the built-in Rust derives, once you've imported them:

```rust [] ignore
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Square {
    width: u32,
}

fn main() {
    let sq = Square { width: 25 };
    let json = serde_json::to_string(&sq).unwrap();
    println!("{}", json);
}
```

Often named after the traits they implement.

Note:

In the Rust Docs search results, the trait appears in blue, and the macro appears in green.

Rust can always work out whether you mean the trait or the macro, from the context.

## Attribute-like macros

* Placed above a type, function, or field
* Can have optional arguments

```rust [] ignore
#[tokio::main(worker_threads = 2)]
async fn main() {
    println!("Hello world");
}
```

## Function-like macros

Called like a function:

```rust ignore
let query = sqlx::query!("SELECT * FROM `person`");
```

## Downsides of Procedural Macros

* Can be difficult to debug
* Slows down compilation a lot
* Have to be stored in a separate crate
  * You're basically building compiler plug-ins at build time

## When Should You Use Procedural Macros?

* When it saves your users a sufficient amount of work
