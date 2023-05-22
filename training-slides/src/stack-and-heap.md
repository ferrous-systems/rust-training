# Stack and Heap

---

Rust defaults to allocation on the stack

## Stack Allocation

```rust []
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 1, y: 1};
}
```

## Box

Heap allocation is represented by the type `Box`.

```rust []
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 1, y: 1};
    let point_on_heap = Box::new(point);
}
```

## Ownership and Borrowing

`Box` is owned, but you can borrow the contained values.

```rust []
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point { x: 1, y: 1};
    let point_on_heap = Box::new(point);
    print_point(&point_on_heap);
}

fn print_point(p: &Point) {
    println!("{:?}", p);
}
```

## Other heap allocations

Other types also allocate on the heap, most notably `Vec` and `String`.

## Placement in

It is currently *not* possible to allocate values at a self-chosen location. The missing feature is called "placement in".

[Detailed discussion here](https://internals.rust-lang.org/t/lang-team-minutes-feature-status-report-placement-in-and-box/4646)

---

In most cases, LLVM already optimizes the stack allocation and the subsequent move to the heap to a direct heap allocation.