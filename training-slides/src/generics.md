# Generics

---

Generics are fundamental for Rust.

## Generic Structs

Structs can have type parameters.

```rust []
struct Point<Precision> {
    x: Precision,
    y: Precision,
}

fn main() {
    let point = Point { x: 1_u32, y: 2 };
    let point: Point<i32> = Point { x: 1, y: 2 };
}
```

Note:

The part `<Precision>` introduces a *type parameter* called `Precision`. Often people just use `T` but you don't have to!

## Type Inference

* Inside a function, Rust can look at the types and infer the types of variables and type parameters.
* Rust will only look at other signatures, never other bodies.
* If the function signature differs from the body, the body is wrong.

## Generic Enums

Enums can have type parameters.

```rust []
enum Either<T, X> {
    Left(T),
    Right(X),
}

fn main() {
    let alternative: Either<i32, f64> = Either::Left(123);
}
```

Note:

What happens if I leave out the `<i32, f64>` specifier? What would type parameter `X` be set to?

## Generic Functions

Functions can have type parameters.

```rust []
fn print_stuff<X>(value: X) {
    // What can you do with `value` here?
}
```

Note:

Default bounds are `Sized`, so finding the size of the type is one thing that you can do. You can also take a reference or a pointer to the value.

## Generic Implementations

```rust []
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    fn new(x: T, y: T) -> Vector<T> {
        Vector { x, y }
    }
}

impl Vector<f32> {
    fn magnitude(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

fn main() {
    let v1 = Vector::new(1.0, 1.0);
    println!("{}", v1.magnitude());
    let v2 = Vector::new(1, 1);
    // println!("{}", v2.magnitude());
}
```

Note:

Can I call `my_vector.magnitude()` if T is ... a String? A Person? A TCPStream?

Are there some trait bounds we could place on `T` such that `T + T -> T` and `T * T -> T` and `T::sqrt()` were all available?

## The error:

```text
error[E0599]: no method named `magnitude` found for struct `Vector<{integer}>` in the current scope
  --> src/main.rs:22:23
   |
1  | struct Vector<T> {
   | ---------------- method `magnitude` not found for this struct
...
22 |     println!("{}", v2.magnitude());
   |                       ^^^^^^^^^ method not found in `Vector<{integer}>`
   |
   = note - the method was found for
           - `Vector<f32>`

For more information about this error, try `rustc --explain E0599`.
```

## Generic Traits

Traits can have type parameters.

```rust []
trait HasArea<T> {
    fn area(&self) -> T;
}
 
// Here we only accept a shape where the `T` in `HasArea<T>` is `f64`
fn print_area(shape: &dyn HasArea<f64>) {
    let area = shape.area();
    println!("Area = {area:0.6}");
}

struct UnitSquare;

impl HasArea<f64> for UnitSquare {
    fn area(&self) -> f64 {
        1.0
    }
}

impl HasArea<u32> for UnitSquare {
    fn area(&self) -> u32 {
        1
    }
}

fn main() {
    let u = UnitSquare;
    print_area(&u);
}
```

## Adding Bounds

* Generics aren't much use without bounds.
* You can apply the bounds on the type, or a function, or both.

```rust []
trait HasArea<T> {
    fn area(&self) -> T;
}

fn print_area<T>(shape: &dyn HasArea<T>) where T: std::fmt::Debug {
    let area = shape.area();
    println!("Area = {area:?}");
}

struct UnitSquare;

impl HasArea<f64> for UnitSquare {
    fn area(&self) -> f64 {
        1.0
    }
}

fn main() {
    let u = UnitSquare;
    print_area(&u);
}
```

## Adding Bounds

The bounds can also go here:

```rust []
trait HasArea<T> {
    fn area(&self) -> T;
}

fn print_area<T: std::fmt::Debug>(shape: &dyn HasArea<T>) {
    let area = shape.area();
    println!("Area = {area:?}");
}
```

Note:

This is exactly equivalent to the previous example, but shorter.

## General Rule

* If you can, try and avoid adding bounds to `structs`.
* Simpler to only add them to the methods.

## Multiple Bounds

You can specify multiple bounds.

```rust []
trait HasArea<T> {
    fn area(&self) -> T;
}

fn print_areas<T: std::fmt::Debug + std::cmp::PartialEq>(
    shape1: &dyn HasArea<T>,
    shape2: &dyn HasArea<T>,
) {
    let area1 = shape1.area();
    let area2 = shape2.area();
    if area1 == area2 {
        println!("Both areas are {area1:?}");
    } else {
        println!("{area1:?}, {area2:?}");
    }
}

struct UnitSquare;

impl HasArea<f64> for UnitSquare {
    fn area(&self) -> f64 {
        1.0
    }
}

fn main() {
    let u1 = UnitSquare;
    let u2 = UnitSquare;
    print_areas(&u1, &u2);
}
```

Note:

Try removing the `std::cmp::PartialEq` bound and see what it says about using the `==` operator on type `T`.

## impl Trait

* The `impl Trait` syntax in argument position was just *syntactic sugar*.
* (It does something special in the return position though)

```rust []
trait HasArea {
    fn area_m2(&self) -> f64;
}

struct AreaCalculator {
    area_m2: f64
}

impl AreaCalculator {
    // Same: fn add(&mut self, shape: impl HasArea) {
    fn add<T: HasArea>(&mut self, shape: T) {
        self.area_m2 += shape.area_m2();
    }
}
```

Note:

Some types that cannot be written out, like the closure, can be expressed as return types using `impl`. e.g. `fn score(y: i32) -> impl Fn(i32) -> i32`.

## Caution

* Using Generics is *Hard Mode Rust*
* Don't reach for it in the first instance...
  * Try and just use concrete types?

## Special Bounds

* Some bounds apply automatically
* Special syntax to *turn them off*

```rust []
fn print_debug<T: std::fmt::Debug + ?Sized>(value: &T) {
    println!("value is {:?}", value);
}
```

Note:

This bound says "It must implement std::fmt::Debug, but I don't care if it has a size known at compile-time".

Things that don't have sizes known at compile time (but which may or may not implement std::fmt::Debug) include:

* String Slices
* Closures

