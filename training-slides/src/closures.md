# Closures

## Rust's Function Traits

* `trait FnOnce<Args>`
* `trait FnMut<Args>: FnOnce<Args>`
* `trait Fn<Args>: FnMut<Args>`

Note:

* Instances of FnOnce can only be called once.
* Instances of FnMut can be called repeatedly and may mutate state.
* Instances of Fn can be called repeatedly without mutating state.
* `Fn` (a trait) and `fn` (a function pointer) are different!

## These traits are implemented by:

* Function Pointers
* Closures

## Function Pointers

```rust
fn add_one(x: usize) -> usize {
    x + 1
}

fn main() {
    let ptr: fn(usize) -> usize = add_one;
    println!("ptr(5) = {}", ptr(5));
}
```

## Closures

* Defined with `|<args>|`
* Most basic kind, are just function pointers

```rust
fn main() {
    let clos: fn(usize) -> usize = |x| x + 5;
    println!("clos(5) = {}", clos(5));
}
```

## Capturing

* Closures can capture their environment.
* Now it's an anonymous `struct`, not a `fn`
* It implements `Fn`

```rust
fn main() {
    let increase_by = 1;
    let clos = |x| x + increase_by;
    println!("clos(5) = {}", clos(5));
}
```

## Capturing Mutably

* Closures can capture their environment by mutable reference
* Now it implements `FnMut`

```rust
fn main() {
    let mut total = 0;
    let mut update = |x| total += x;
    update(5);
    update(5);
    println!("total: {}", total);
}
```

Note:

The closure is dropped before the `println!`, making `total` accessible again (the &mut ref stored in the closure is now gone).
If you try and call `update()` after the `println!` you get a compile error.

## Capturing by transferring ownership

```rust
fn main() {
    let items = vec![1, 2, 3, 4];
    let update = move || {
        for item in items {
            println!("item is {}", item);
        }
    };
    update();
    // println!("items is {:?}", items);
}
```

## But why?

* But why is this useful?
* It makes iterators really powerful!

```rust []
fn main() {
    let items = [1, 2, 3, 4, 5, 6];
    let n = 2;
    for even_number in items.iter().filter(|x| (**x % n) == 0) {
        println!("{} is even", even_number);
    }
}
```

## Cleaning up

It's also very powerful if you have something you need to clean up.

1. You do some set-up
2. You want do some work (defined by the caller)
3. You want to clean up after.

```rust []
fn setup_teardown<F, T>(f: F) -> T where F: FnOnce(&mut Vec<u32>) -> T {
    let mut state = Vec::new();
    println!("> Setting up state");
    let t = f(&mut state);
    println!("< State contains {:?}", state);
    t
}
```

## Cleaning up

```rust []
fn setup_teardown<F, T>(f: F) -> T where F: FnOnce(&mut Vec<u32>) -> T {
    let mut state = Vec::new();
    println!("> Setting up state");
    let t = f(&mut state);
    println!("< State contains {:?}", state);
    t
}

fn main() {
    setup_teardown(|s| s.push(1));
    setup_teardown(|s| {
        s.push(1);
        s.push(2);
        s.push(3);
    });
}
```

Note:

In release mode, all this code just gets inlined.
