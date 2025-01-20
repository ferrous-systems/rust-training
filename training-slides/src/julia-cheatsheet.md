# Rust Fundamentals

## Overview

### Why Learning Rust for Julia people in particular is **special**:

* We already have many features that aren't a new sale: safety *and* performance through a fast GC, standard memory semantics that have mechanical sympathy coupled with a powerful JIT, a modern package manager, a unit testing framework, metaprogramming, and a documentation system.
* When reaching for Rust is justified (resource constrained environments like embedded, cloud infrastructure or data bases, etc.) Julians have to go straight for FFI, erasing many of the boons of Safe Rust and dealing very quickly with unsafe code. This is topic is **not** the promising experience for beginners in the language.

----

* Julians are used to thinking about memory footprint and layout in perf-sensitive code, as well as subtyping relations - two lynchpins for understanding the ownership and borrowsing system where the role of subtyping and variance are very commonly ommitted. This topic can therefore be explained much clearer and earlier in the curriculum.
* Materials for Rust beginners normally cater for C++ expertise (which skip error handling as philosophy) or Python/Go/Javascripters and spend too much importance on memory layouts (which Julians may know from designing faster algorithms)
* Julia has a rich generics vocabulary and a JIT that mirror Rust's trait -> monomorphize approach.

## Installation
## Basic Types

* Rust defaults to `i32` and `f64` on numeric literals, whereas Julia uses `Int`, the `Integer` pointer width on your machine. Rust uses `usize` for that, and you will have to index arrays with that type, so `x[i as usize]` will be a bane upon your code.
* As in Julia, you'll have the convenience of defining numbers with underscores without affecting parsing, i.e. `let x = 1_000_000;` is allowed.
* There's specific suffixes for primitive numeric types like `1.0_f32` or `10_u128`.
* `BigInt`'s or `BigFloat`s are not part of the Rust stdlib.
* Your numeric code will likely be sprinkled with loads of `1.0 / (n as f64)`. It's unfortunate but unavoidable.
* Delay using generic numerics and use `i64` and `f64` until you need a really, really good reason to switch, and then `num` crate. This is because generics in Julia are invisible when done well but explicit in Rust and imply call site changes for your previously working code.
* `Char`s in Rust represent a single Unicode codepoint, which means that `'👪'` is a valid Julia `Char`, but not a Rust one. See further down for more discussions about strings. This is a key Rust ethos: "Make invalid states unrepresentable".
* Yes, indexes start at 0. Use the equal sign in `for i in 0..=10 {...}` to make an inclusive range.
* Lots of useful constants are tucked away in stdlib modules, like `std::f64::consts::PI`. Import them with `std::f64::consts::*;` at the top of your file.
* Rust has validated UTF8 strings by default.
* Just use `String` when starting out. Then [use this](https://steveklabnik.com/writing/when-should-i-use-string-vs-str/) to decide when you need to care.
* `String` is the owned variant, `&str` is the borrowed variant - it's easier to think that `&str` is  just "I'm getting an immutable string slice that I'm only reading from"
* Read the standard library, it very much pays off to know methods like `.split_n`, `.bytes()` and many more other stdlib functions

## Control Flow

* `for` loops with array access syntax `a[i] = i + 1` will bounds check by default, hampering optimizations.
* `if`s don't require parentheses, and if you didn't learn that by coding a bit in Rust by now, it means you don't have a proper `rust-analyzer` setup. See the `FAQ` at the bottom for proper dev workflow instructions.
* All branches are required to return the same type, as do `match` arms. Notice that Rust takes the function return type definition (aka the `String` in  `fn foo(...) -> String {...}` as the ground truth of what your function's returns must fulfill - this means that you can often coerce different branches with a judicious `.into()` suffix and carry on.
* The ownership system can propagate some analysis across branches, see 

```rust 
fn main() {
    let mut haystack = String::from("hello");
    for needle in haystack.chars() {
        if needle == 'l' {
            haystack.push_str(" world");
            // Comment the following line for a surprise!
            break;
        } 
    }
    println!("{}", haystack);
}
```

This would normally be a trivial iterator invalidation bug (we'd be modifying a collection as we're iterating over it), but Rust is able to figure out that if the `if` branch is taken, then the iterator is no longer needed and doesn't let the code compile. This is in contrast to the borrowchecker having to explore two different branches when one is known as dead. A new version of the borrowchecker that implements [the TreeBorrows system](https://www.ralfj.de/blog/2023/06/02/tree-borrows.html) will hopefully overcome this.

## Syntax Clashes

* `a^b` is exponentiation in Julia, XOR in Rust.
* An `array` in Rust which must be stack allocated and cannot change it's size, as it is part of its type, e.g. `[f32; 4]`, and only exists by default in the 1D case. A `vector` is a different type `Vec<f32>` and it is heap allocated.
* A `slice` in Julia look slike this `x[1:10]` and copies the array values by default. A slice in Rust is actually a type where a container's length is carried by a reference: `&[f32]`. Note that the compiler must know the size somehow - `[f32; 4]` is known as size 4 at compile time, `&[f32]` knows that the `&` carries the type at runtime and `Vec<32>` is heap allocated, and that is communicated via a `Box` type.
* `println!`, and any other function that ends with a `!` is a macro in Rust; mutation is more explicit in the type system with the `mut` keyword on a binding basis.
* [Cheatsheet on confusing terms](https://ferrous-systems.com/blog/cheatsheet-for-confusing-rust-terms/) - `Clone` vs `Copy` and `Debug` vs `Display`
* `move` - if you have any knowledge of C++'s move semantics, forget them! The keyword in Rust has to do with transferring ownership in Rust, not an optimization for removing containers.
* `;` is necessary for terminating a Rust expression, whereas in Julia it stops printing to the REPL. In Rust, the last expression in a function also does an implicit return, and branches that return don't need a `return x;`, just a `x` will do.
* The turbo fish `::<>` operator looks ugly as all hell but... that's actually [what it look slike](https://en.wikipedia.org/wiki/Turbot) it's useful to disambiguate callers' return type, like `my_vec.iter().map(f).collect::<i32>()`, where Rust can now know that you cant a `Vec<i32>` in the end.
* Writing `@test 0.1 + 0.2 ≈ 0.3` in Rust is done by using `assert_abs_diff_eq!(0.1, 0.2, epsilon = f64::EPSILON * 10.0;);` inside a test function.

## Compound Types

* Enums will be your bread and butter in Rust. With `match` and traits, they are as close to a unifying design principle in Rust as multiple dispatch is to Julia. Learning to model your problems around enums will be a boon in the long run.
* Don't forget the `..p` syntax for initializing a struct:
```rust
let p2 = Point {x: 0, ..p1}; // will copy over remaining fields from `..p1`
```
It's very useful for longer "builder patterns".
* Tuple structs like `Pixel(i8, i8, i8)` - let you hitch on to the type system and expand the `newtype` idiom and friends. They don't have a direct analog in Julia, but can be defined inline as part of enums:
```rust
enum House {
    NumberOfPets(i8),
    Address(String),
    //...
}
```

* TODO add link: Also let you get around Orphan Rules / type piracy, and tooling around newtypes lets you extend other's code
* Recursive types recquire `Box<T>` - You can't define types in Rust without communicating their size at compile time or opting out (by boxing them somehow, the easiest case is with `Box<T>`).

## Pattern matching
* It happened to all of us who didn't come from ML style languages - you'll start writing "C-style" Rust until you master the succinctness offered by idiomatic pattern matching. Take some good notes and read the exmaples [in the Rust By Example](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_enum.html) guide.
* Remember, Rust is also an expression based language, which means you can match on tuples (`match (x % 3, x % 5) {...}`) and destructure them in the same line: `let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else { ... }` will only enter the inner scope if both `pop.()`s were succesful.
* The following constructs are basic but welcome syntax sugar once you start wrangling matches:
    * `let else` - pattern match on a binding, and handle the remaining cases. 
```rust
if let Shape::Circle(radius) = shape {
    // radius is a valid binding here if it pattern matched on Shape::Circle(___) 
}
```
**Note:** The syntax here is challenging when starting if you think of it as normal "left to right code" and not as an attempt to get a binding like `let x = 3+3;` - the equal sign binds weakest, so we know to resolve the expressions on the right before knowing that `x` is a valid binding in the remaining scope. If you think of the following cases as similar to those parsing rules, you will save yourself some headaches.
    * `let if` - pattern match on a binding, and ignore the remaining cases.
    * `while let` - try pattern matching on a binding, and if succesful, enter the loop body, otherwise exit. These are fundamental for async code, as `for` loops don't work with `async`/`await`.
    * `match!` - if you only need to know if a match happened (a boolean), this macro is your friend.
You'll get a lot of benefit from revisiting your code or getting peers to review your code - it's not unheard of to de-nest your Rust code by 1 or 2 levels with judicious (and clearer) idiomatic pattern matching.
* Option (absence of a value), Result(Ok or Error) -> aka how to not need `nullptr`

## Ownership and Borrowing


Historical note:
Rust didn't "invent" the ownership system ex nihilo.

* There's only 3 things: `T`, `&T`, `&mut T`
* Ownership system and where it came from - like multiple dispatch there was an adhoc, informally spec'd... same for ownership system.
* Most of your functions should take &T, not T
* Operators are secretly funcitons, and they take references, may be created behind your back (yes, even `+=` or `==`)
* avoid indexing!
* TODO Quiz

## Error Handling


Error Handling was such a central design philosophy in Rust that it's worth knowing the context because Julia's focus didn't prioritize handling errors. I will know talk for a few paragraphs to set the stage for a simple example in simple C that I would have like to have had when I was starting Rust.

In the old C code bases, different failure modes for a program (or errors) had to be managed. We have studies to [support the fact that](https://sled.rs/errors) bad error handling leads to catastrophy: 

> almost all (92%) of the catastrophic system failures
> are the result of incorrect handling of non-fatal errors
> explicitly signaled in software.

In the world of embedded systems, systems programming or critical systems, this state of affairs is unacceptable. Imagine that we have to parse an incoming message of the format `PUBLISH your_string_here\n'. Several corner cases arise if we want to extract said string:
1. We could have no ending newline
2. We could have more than 1 ending newline
3. We could have a missing space
and so on.

A C codebase would only have access to structs and primitive types, so they resorted to the use of integer macros to flag failures:
```c
#define NO_ENDING_NEWLINE 1
#define TOO_MANY_NEWLINES 2
#define MISSING_SPACE 3

int parse_message(char* buf) {
    if check_ending_newline(buf) {
        return NO_ENDING_NEWLINE;
    }
    if single_ending_newline(buf) {
        return TOO_MANY_NEWLINES;
    }
    if no_space_separates_data(buf) {
        return MISSING_SPACE;
    }
    handle_message(buf);
}
```

Which has all sorts of sharp ends:
    * You are returning an `int` and then doing a lot of additional bit manipulation to pull out the behaviour. This becomes tedious and error-prone. This also means that you can inadvertently promote the returned int and misuse your own API silently.
    * If you ever discover a new corner case (say, presence of non-ASCII characters), you're responsible for updating at least 3 different places: a new `#define` for the new error condition, new control flow `parse_message` to handle this additional case, and, worst of all, every other call site across your codebase.

... just to name a few.

Compare this with the Rust approach:
```rust
enum ParseError {
    NoEndingNewLine,
    TooManyNewLines,
    MissingSpace,
}

fn parse_message(buf: &str) -> Result<String, ParseError> {
    has_ending_newline(buf)?;
    only_single_newline(buf)?;
    contains_separating_space(buf)?;
    let data: String = extract_data(buf);
    Ok(data)
}
```
Notice:
* we know that we cannot modify `buf` since it is using a shared referencd `&str`. This function therefore is *guaranteed* by the Rust type system not to allow mutation inside it's body of `buf`.
* Should we (or a tired, unfortunate coworker on another continent) extend the `ParseError` enum, then our callers will *have* to handle those new variants of corner cases. When refactoring, changes to critical data structures are all caught by the compiler and then refactoring, usually, becomes a mechanical ordeal of applying the same fix.

Most Rust tutorials on error handling would be glad to finish the lesson here with the "big ball of mud" enum that soaks up all the corner cases. This is not a good practice for scaling your error handling: you will lose local contexts for handling those errors once callee's have to deal with `Result`s and you make no distinction between immediate, must handle errors and errors that can be ignored. [This blog has an excellent](https://sled.rs/errors) writeup about how the Rust community keeps falling for this style due to the syntactical ease of `?` (just as people in Julia tend to overdose on dispatching everything, instead of keeping its use judicious.

A more mature version of the code would look like

```rust
//fn handle_message(buf: &str) -> Result<Result(), CompareAndSwapError>, Error>
let result = handle_message(buf)?;

if let Err(error) = result {
    // handle expected issue
}
```
which lets us nest `Result`s, peel them with `?`, separate local from global concerns errors, and match on exhaustive patterns in specific places. To wit:

> "Use try ? for propagating errors. Use exhaustive pattern matching on concerns you need to handle. Do not implement conversions from local concerns into global enums, or your local concerns will find themselves in inappropriate places over time. Using separate types will lock them out of where they don’t belong."

Our takeaway is thus:
* The C story of error handling require integer manipulation and constant error checking, where the programmer had to hold a ton of invariants in their head about what any part of the codebase could interact with any other.
* Rust's type system is ergonomic enough that facilities like `match` and friends lets us offload thinking about those invariants to the compiler and worry about more interesting things.
* **Errors will be made explicit and up front by Rust - it will not let you keep coding with unhandled errors.**

This last line is the key - Rust is not the language to let you "get away with it for now". You get a `todo!()` or an `unimplemented!()` macro at best.

* Lean on the Early return operator, which is `?` ! if you have a Result, try to use ? (you only care about Ok case anyways most of the time.)
* Learning all the transforms of `Option/Err/Result` can be much nicer if you have a [handy diagram](https://www.lurklurk.org/effective-rust/transform.html) from the Effective Rust book.
* https://docs.google.com/drawings/d/1EOPs0YTONo_FygWbuJGPfikO9Myt5HwtiFUHRuE1JVM/preview

## Collections
## Iterators

* Examples:
    * reading lines in a file? double `filter_map`
* Uncomfy amount of `*x` stars. The `Iterator` trait has an associated type `Item`, and here `Item = &i32`, but the `filter` produces `&Item = &&i32`

* For debugging an iterator, you don't need to pepper in `dbg!` randomly, just use [.inspect()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect):

```rust
let sum = a.iter()
    .cloned()
    .inspect(|x| println!("about to filter: {x}"))
    .filter(|x| x % 2 == 0)
    .inspect(|x| println!("made it through filter: {x}"))
    .fold(0, |sum, i| sum + i);
```

which will print

```
6
about to filter: 1
about to filter: 4
made it through filter: 4
about to filter: 2
made it through filter: 2
about to filter: 3
```

## Imports and Modules
## Good Design Practices

## FAQ and Dev Workflow

* Q: I'm itching for a more interactive Rusty experience, and I heard there's some REPLs! What do you recommend?
    * A: As much as I'd want to, I wouldn't recommend Rust REPLs today: a) you're gonna miss out on help strings, compiler diagrams and instant feedback from `rust-analyzer` when coding, as well as stdlib API discovery via autocomplete (think having an `(0..10).iter().<TAB>)` and hovering over the methods shown. I can see very confident Rust coders whipping up demos on a notebook/REPL, but I'd encourage beginners/intermediate Rust coders away from Rust REPLs for the time being.
TODO: add link to video on rust-analyzer
* Documentation: type `std.rs/fold` into your browser to go directly to the Rust docs via a clever DNS redirect.
* `PkgTemplates.jl` -> `cargo new foo`
* `BenchmarkTools.jl` -> `criterion` for fine grained control, `divan` for easier setup.
* `using Test` -> unit tests, comes preinstalled, can be written in any file, not just inside a `test/` folder.
* `TestItemRunner.jl` -> with `rust-analyzer`: click on the `Run Test` button atop the `#[test]`
* `juliaup` -> `rustup`! We actually stole the name from them
* `Documenter.jl` -> `rustdoc`, comes preinstalled (and hence why all Rust docs tend to look the same). You could also consider `mdbook` for serving a website. You can save yourselve some clicks if you to `std.rs/foo` in your browser search.
* `LanguageServer.jl` -> `rust-analyzer`, with the VSCode extension getting the most support
* REPL snippets -> [A Rust playground link](https://play.rust-lang.org/) is the easiest way to share Rust snippets others can run. Cool note: They have the top ~100 crates preinstalled in the VMs, so you can use the `rand` crate. DevFlow: `cargo new foo` -> `examples` -> setup `divan` -> bench function / test just below it
* Julia Slack/Zulip/Discourse: Rust people tend to use [Discord]() for the larger community, a [Discourse for devs](), a [Discourse for users](), and a [Zulip for rustc development itself](). You'll likely not find as dedicated applied maths / scientific channels in any one given Rust forum as you would in Julia. If you do, let me know! I'd love to find them.
* Julia Dev Docs -> [rustc dev guide]() gets you from 0 to contributor pretty quickly - I made my first PR with that and some support on the community Discord.
* `Aqua.jl` -> The compiler itself and `clippy`, come preinstalled.
* `JuliaFormatter.jl` -> `rustfmt` comes preinstalled.
* `BinaryBuilder.jl` -> nothing yet, but maybe we can collaborate with Rust folks on that front. JLL's are usually known as a `*-sys` + installation combo. If you install `cargo install cargo-binstall`, you can add binaries without having to build them from source!!!
* `] add Foo` -> `cargo add foo`
* `Franklin.jl` -> `zola` a fast static site generator
* `@time_imports using Makie` -> `cargo build --timings`, to know which of your dependencies is taking a long while to ~~pre~~compile
* `@code_lowered/native/llvm foo(x)` -> dump your code into godbolt.com and set the `-C opt-level=3` flag.
* `This Month in Julia Newsletter` -> [This Week in Rust](https://blog.dureuill.net/articles/dont-mix-rayon-tokio/) - always a good read, includes a jobs list at the end.
* `ExprTools.jl`/`Expronicon.jl` (tools for writing macros) -> [syn](https://github.com/dtolnay/syn), [quote](https://docs.rs/quote/latest/quote/),  and `proc_macro2` for testing them. See [this blog post](https://xy2.dev/blog/simple-proc-macro/) and this [proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
* `Val{N}` -> const generics with an associated integer. 
* How do I override printing for my types, like I would with `show(io::IO, ...)`?
* `nothing` -> is called "the unit type" and is spelled `()` in Rust
* Holy Trait trick -> [Marker Traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#marker-traits)! 
* `+=` keeps failing, why? -> it's an operator. TODO `AddAssign`
* How can I setup examples for uses? -> `scrape-examples`, see `dev-flow` video

# Applied Rust
## Methods and Traits

* TODO: type piracy and Orphan rule
* Social vs systemic
* Huge blog posts on [traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)
* TODO opting into interaces with `.next()` example + rust-analyzer trick for populating them
* TODO defaults vs necessary methods

## Rust I/O Traits
## Generics

[From this great link](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md) comes this recommendation on Generic types vs Associate Types:

> The general rule-of-thumb is:
> Use associated types when there should only be a single impl of the trait per type.
> Use generic types when there can be many possible impls of the trait per type.

## Lifetimes

## Lifetimes, Subtyping, Variance

Actually, you can know **A LOT** about this system already if you know subtyping from Julia!
Almost all beginner level explanations of lifetimes I know of punt on subtyping and variance until the much later advanced courses, which is a shame, because a small bit of it can be used to explain the internals of the borrowchecker.

* [Common Rust lifetime misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)

- lifetimes are
	- named
	- regions of code
	- that a reference must be valid for
- vs
-
  ```rust
  fn example _2() {
  let foo = 69;
  let mut r;
    {
      let x = 42;
      r = &x;
  	println! ("{}", *r();
  }
  r = &foo;
  println!("{}", *r);
  ```
- liveness: a variable is live if its current value may be used later in the program
- Refs have 2 properties: when they must be valid, what they can point to
	- as in, which region of memory / which resource
- outlives: `'a: 'b` - use it implicitly all the time
	- `'a: 'b ⟺ 'a ⊆ 'b`
- Thanks to subtyping and variance, this
	-
	  ```rust
	  fn longest<'s1, 's2, 'out>(s1: &'s1 str, s2: &'s2 str) -> &'out str 
	  where
	  	's1: 'out,
	  	's2: 'out {...}
	  // this is what happens under the hood
	  
	  fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
	  ```
	- Note how if this wasn't true then this would be the most annoying code ever:
	-
	  ```rust
	  fn main() {
	    let x: &'x str = "hi";
	    let y: &'y str = "hello";
	    let z: &'z str = "hey";
	    
	    let l1: &'l1 str = longest(x, y);
	    let l2: &'l2 str = longest(l1, z);
	  }
	  ```
	- this creates a new smallest possible region `&'l1` that contains both `'x` and `'y`.
	-

## Cargo Workspaces

* The compiler will compile faster the shallower and wider your crate dependency graph is. Avoid nesting modules where possible to unlock more parallelism. This is usually a result of starting your project with workspaces. Annoyingly, there is not tool to add crates to a workspace via the CLI, but people are working on it.

## Heap Allocation (Box and Rc)
## Shared Mutability (Cell, RefCell, OnceCell)
## Thread Safety (Send/Sync, Arc, Mutex)
## Closures and the Fn/FnOnce/FnMut traits
## Spawning Threads and Scoped Threads

# Advanced Rust
## Advanced Strings
## Building Robust Programs with Kani
## Debugging Rust
## Deconstructing Send, Arc, and Mutex
## Dependency Management with Cargo
## Deref Coercions
## Design Patterns

TODO: flesh this out
* Binary vs lib 
* dbg!
* println!("{x:?}");
* use derives like `Hash`
* let mut x = ...; let x = x;
* prefer `&x` for function signatures where possible
* [This playlist](https://www.youtube.com/playlist?list=PLhjB8nmMLotIG0ik1RXjl0lfZcg9oxhMg) by Logan Smith covers a great many topics for idiomatic Rust.
* `#[static_dispatch]`
* Multiple dispatch vs Rust generics

## Documentation

* TODO `std.rs/filter_map`

## Drop, Panic and Abort
## Dynamic Dispatch
## Macros

* `rust-analyzer` lets you put your cursor on a macro and then `Ctrl+Shift+p` will let you expand it recursively. If you're using Rust Rover, it has a macro expansion stepper! These are *very* useful when debuggin macros.
* macros don't operate on symbols, but rather on syntactic elements (more specifically, fully formed ASTs). This lets Rust only produce valid Rust code from its macros.

## Property Testing
## Rust Projects Build Time
## Send and Sync
## Serde
## Testing
## The stdlib
## Using Cargo
## Using Types to encode State

# Rust and Web Assembly
## WASM

