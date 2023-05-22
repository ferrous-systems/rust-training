# Implementation and Traits

---

Rust offers the possibility to bind functions to types.

## Warning

This sometimes looks like object-oriented programming, but it is not.

In particular, run-time polymorphism, messages, classes, subtypes, and method overload are missing.

## Simple implementations: associated function

```rust []
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let my_point = Point::new(1, 2);
    println!("My point being: {:?}", my_point);
}
```

## Remark

`new` here is purely convention.

## A Python analogy

```rust []
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn from_pair(pair: (i32, i32)) -> Point {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }

    fn into_pair(self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn inspect(&self) {
        println!("Current point value: {:?}", self);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn x(&self) -> &i32 {
        &self.x
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn y(&self) -> &i32 {
        &self.y
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }
}

fn main() {
    let mut my_point = Point::new(1, 2);
    my_point.inspect();
    my_point.move_to(2, 3);
    my_point.inspect();

    let x = my_point.x_mut();
    *x = 5;

    my_point.inspect();
}
```

Note:

If you're familiar with Python's `self`, Rust's use of it is very very similar

## Borrowing and Ownership of `self`

It is like normal ownership and borrowing, but at the beginning somewhat unfamiliar.

-   Borrowing through one function *simultaneously borrows self*.
-   This is especially applicable for mutable borrows!
-   `self` without `&` takes ownership to the value from the calling context.

## Borrowing and Ownership of `self`

| Owned | Borrowed | Mutably borrowed |
| ----- | -------- | ---------------- |
| self  | &self    | &mut self        |

## Interesting Differences to Common OO

-   Values can be replaced when calling `&mut` functions
-   Values, for example iterators and builders, can have methods that consume `self` and are thus invalidated.
-   This solves the problem of invalidating iterators!

## Side note

-   Implementations can occur multiple times. This is useful when multiple constraints are needed.

## Traits

Traits are Rust's particular way of abstracting over types.

---

We've already met a trait: `Debug`.

---

Traits define functions types must implement. They can then be used generically.

---

```rust []
struct Point {
    x: i32,
    y: i32
}

trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

impl Distance for Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", p1.distance(&p2));
}
```

## Self

`Self` is a special type: it is the type currently being implemented.

## Generic Traits

Traits can have type parameters.

---

```rust []
struct Point {
    x: i32,
    y: i32
}

trait Distance<OtherShape> {
    fn distance(&self, other: &OtherShape) -> f64;
}

impl Distance<Point> for Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", p1.distance(&p2));
}
```

---

Working with generic traits is very common.

## Inference of Traits

Type inference of traits is very advanced, but sometimes, undecidable situations can occur. In this case, the compiler needs help deciding.

There are multiple techniques.

## Full qualified function calls

```rust []
struct Point {
    x: i32,
    y: i32
}

trait Distance<OtherShape> {
    fn distance(&self, other: &OtherShape) -> f64;
}

impl Distance<Point> for Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", <Point as Distance<Point>>::distance(&p1, &p2));
}
```

Any reachable function in Rust can be addressed with this syntax.

## Associated Types

Associated types are generic parameters, but they are ignored during inference.

---

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

trait MyAddition<Other> {
    type Output;

    fn add(&self, other: &Other) -> Self::Output;
}

impl MyAddition<Point> for Point {
    type Output = Point;

    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    println!("{:?}", p1.add(&p2))
}
```

## `impl Trait`

`impl Trait` is used when the type of a value does not need to be named.

```rust []
fn main() {
    let v = vec![1,2,3];
    let i = make_iter(&v);
}

fn make_iter<'a>(v: &'a Vec<u8>) -> impl Iterator<Item=u8> + 'a {
    v.iter().map(|v| (*v)*2)
}
```


---

```rust []
fn main() {
    let v = vec![1,2,3];
    let i = v.iter();
    let i2 = double(i);
}

fn double<'a>(i: impl Iterator<Item=&'a u8> + 'a) -> impl Iterator<Item=u8> + 'a {
    i.map(|v| (*v)*2)
}
```
---

Limitations:

* No `impl Trait` in trait methods

---

```rust ignore []
trait Foo {}

trait Bar {
    /// This is not currenty supported in Rust
    fn fooify(&self) -> impl Foo;
}
```
