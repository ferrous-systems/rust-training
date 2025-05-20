# Generics

---

Generics are fundamental for Rust.

## Generic Structs

Structs can have type parameters.

```rust
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

```rust
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

```rust
fn print_stuff<X>(value: X) {
    // What can you do with `value` here?
}
```

Note:

Default bounds are `Sized`, so finding the size of the type is one thing that you can do. You can also take a reference or a pointer to the value.

## Generic Implementations

```rust
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

<pre><code data-trim data-noescape><span class="er b">error[E0599]</span><b>: no method named `magnitude` found for struct `Vector&lt;{integer}&gt;` in the current scope</b>
<span class="eb b">  --&gt; </span>src/main.rs:23:23
<span class="eb b">   |</span>
<span class="eb b">2  |</span> struct Vector&lt;T&gt; {
<span class="eb b">   |</span> <span class="eb b">----------------</span> <span class="eb b">method `magnitude` not found for this struct</span>
<span class="eb b"> ...</span>
<span class="eb b">23 |</span>     println!(&quot;{}&quot;, v2.magnitude());
<span class="eb b">   |</span>                       <span class="er b">^^^^^^^^^</span> <span class="er b">method not found in `Vector&lt;{integer}&gt;`</span>
<span class="eb b">   |</span>
<span class="eb b">   = </span><b>note</b>: the method was found for
           - `Vector&lt;f32&gt;`
<b>For more information about this error, try `rustc --explain E0599`.</b></code></pre>

## Adding Bounds

* Generics aren't much use without bounds.
* A bound says which traits must be implemented on any type used for that type parameter
* You can apply the bounds on the type, or a function/method, or both.

## Adding Bounds - Example

```rust
trait HasArea {
    fn area(&self) -> f32;
}

fn print_area<T>(shape: &T) where T: HasArea {
    let area = shape.area();
    println!("Area = {area:?}");
}

struct UnitSquare;

impl HasArea for UnitSquare {
    fn area(&self) -> f32 {
        1.0
    }
}

fn main() {
    let u = UnitSquare;
    print_area(&u);
}
```

## Adding Bounds - Alt. Example

```rust
trait HasArea {
    fn area(&self) -> f32;
}

fn print_area<T: HasArea>(shape: &T) {
    let area = shape.area();
    println!("Area = {area:?}");
}

struct UnitSquare;

impl HasArea for UnitSquare {
    fn area(&self) -> f32 {
        1.0
    }
}

fn main() {
    let u = UnitSquare;
    print_area(&u);
}
```

Note:

This is exactly equivalent to the previous example, but shorter. However, if you
end up with a large set of bounds, they are easier to format when at the end of
the line.

## General Rule

* If you can, try and avoid adding bounds to `structs`.
* Simpler to only add them to the methods.

## Multiple Bounds

You can specify multiple bounds.

```rust
trait HasArea {
    fn area(&self) -> f32;
}

fn print_area<T: std::fmt::Debug + HasArea>(shape: &T) {
    println!("Shape {:?} has area {}", shape, shape.area());
}

#[derive(Debug)]
struct UnitSquare;

impl HasArea for UnitSquare {
    fn area(&self) -> f32 { 1.0 }
}

fn main() {
    let u = UnitSquare;
    print_area(&u);
}
```

## impl Trait

* The `impl Trait` syntax in argument position was just *syntactic sugar*.
* (It does something special in the return position though)

```rust
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

## Generic over Constants

In Rust 1.51, we gained the ability to be generic over *constant values* too.

```rust
struct Polygon<const SIDES: u8> {
    colour: u32
}

impl<const SIDES: u8> Polygon<SIDES> {
    fn new(colour: u32) -> Polygon<SIDES> { Polygon { colour } }
    fn print(&self) { println!("{} sides, colour=0x{:06x}", SIDES, self.colour); }
}

fn main() {
    let triangle: Polygon<3> = Polygon::new(0x00FF00);
    triangle.print();
}
```

Note:

`SIDES` is a property of the type, and doesn't occupy any memory within any
values of that type at run-time - the constant is pasted in wherever it is used.

## Generic Traits

Traits themselves can have type parameters too!

```rust
trait HasArea<T> {
    fn area(&self) -> T;
}
 
// Here we only accept a shape where the `U` in `HasArea<Y>` is printable
fn print_area<T, U>(shape: &T) where T: HasArea<U>, U: std::fmt::Debug {
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

## Special Bounds

* Some bounds apply automatically
* Special syntax to *turn them off*

```rust
fn print_debug<T: std::fmt::Debug + ?Sized>(value: &T) {
    println!("value is {:?}", value);
}
```

Note:

This bound says "It must implement std::fmt::Debug, but I don't care if it has a size known at compile-time".

Things that don't have sizes known at compile time (but which may or may not implement std::fmt::Debug) include:

* String Slices
* Closures
