# Methods and Traits

# Methods

## Methods

* Methods in Rust, are functions in an `impl` block
* They take `self` (or similar) as the first argument (the *method receiver*)
* They can be called with the *method call operator*

## Example

```rust []
struct Square(f64);

impl Square {
    fn area(&self) -> f64 { self.0 * self.0 }
    fn double(&mut self) { self.0 *= 2.0; }
    fn destroy(self) -> f64 { self.0 }
}

fn main() {
    let mut sq = Square(5.0);
   
    sq.double();  // Square::double(&mut sq)
    println!("area is {}", sq.area()); // Square::area(&sq)
    sq.destroy(); // Square::destroy(sq)
}
```

Note:

You can always use the full function-call syntax. That is what the method call operator will be converted into during compilation.

For motivation for something that takes `self`, imagine an embedded device with a `Uart` object that owns two `Pin` objects - one for the Tx pin and one for the Rx pin. Whilst the `Uart` object exists, those pins are in UART mode. But if you destroy the `Uart`, you want to get the pins back so you can re-use them for something else (e.g. as GPIO pins). Equally you could destroy some `HTTPRequest` object and recover the `TCPStream` contained within, so you could use it for WebSocket traffic instead of HTTP traffic.

## Method Receivers

* `&self` means `self: &Self`
* `&mut self` means `self: &mut Self`
* `self` means `self: Self`
* `Self` means whatever type this `impl` block is for

## Method Receivers

* Other, fancier, *method receivers* [are available](https://doc.rust-lang.org/reference/items/associated-items.html)!

```rust [] ignore
struct Square(f64);

impl Square {
    fn by_value(self: Self) {}
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}
    fn via_projection(self: <Example as Trait>::Output) {}
}
```

Notes:

This slide is only intended to show that there's lots of complexity behind the curtain, and we're ignoring almost all of it in this course. Come back for Advanced Rust if you want to know more!

## Associated Functions

* You can also just declare functions with no *method receiver*.
* You call these with normal *function call* syntax.
* Typically we provide a function called `new`

```rust []
pub struct Square(f64);

impl Square {
    pub fn new(width: f64) -> Square {
        Square(width)
    }
}

fn main() {
    // Just an associated function - nothing special about `new`
    let sq = Square::new(5.0);
}
```

Note:

Question - can anyone just call `Square(5.0)` instead of `Square::new(5.0)`? Even from another module?

## Associated Constants

`impl` blocks can also have `const` values:

```rust []
pub struct Square(f64);

impl Square {
    const NUMBER_OF_SIDES: u8 = 4;

    pub fn perimeter(&self) -> f64 {
        self.0 * f64::from(Self::NUMBER_OF_SIDES)
    }
}
```

# Traits

## Traits

* A trait is a list of methods and functions that a type must have.
* A trait can provide *default* implementations if desired.

```rust []
trait HasArea {
    /// Get the area, in m².
    fn area_m2(&self) -> f64;

    /// Get the area, in acres.
    fn area_acres(&self) -> f64 {
        self.area_m2() / 4046.86
    }
}
```

## An example

```rust []
trait HasArea {
    fn area_m2(&self) -> f64;
}

struct Square(f64);

impl HasArea for Square {
    fn area_m2(&self) -> f64 {
        self.0 * self.0
    }
}

fn main() {
    let sq = Square(5.0);
    println!("{}", sq.area_m2());
}
```

## Associated Types

A trait can also have some *associated types*, which are type aliases chosen when
the trait is *implemented*.

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct MyRange { start: u32, len: u32 }

impl Iterator for MyRange {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
```

## Rules for Implementing

You can only *implement* a *Trait* for a *Type* if:

* The *Type* was declared in this module, or
* The *Trait* was declared in this module

You can't implement someone else's trait on someone else's type!

Note:

If this was allowed, how would anyone know about it?

## Rules for Using

You can only *use* the trait methods provided by a *Trait* on a *Type* if:

* The trait is in scope
* (e.g. you add `use Trait;` in that module)

## Traits

* The standard library provides lots of traits, such as:
  * [std::cmp::PartialEq] and [std::cmp::Eq]
  * [std::fmt::Debug] and [std::fmt::Display]
  * [std::iter::IntoIterator] and [std::iter::Iterator]
  * [std::convert::From] and [std::convert::Into]

[std::cmp::PartialEq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[std::cmp::Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[std::fmt::Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[std::fmt::Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[std::iter::IntoIterator]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
[std::iter::Iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[std::convert::From]: https://doc.rust-lang.org/std/convert/trait.From.html
[std::convert::Into]: https://doc.rust-lang.org/std/convert/trait.Into.html

Note:

We walk the attendees through each of these examples. They are only listed in pairs for the pleasing symmetry - nothing in Rust says they have to come in pairs.

## Sneaky Workarounds

If a trait method uses `&mut self` and you really want it to work on some `&SomeType` reference, you can:

```rust [] ignore
impl SomeTrait for &SomeType {
    // ...
}
```

The I/O traits do this.

## Using Traits Statically

* One way to use traits is by using `impl Trait` as a type.
* This is static-typing, and a new function is generated for every actual type passed.
  * Known as *[monomorphisation](https://en.wikipedia.org/wiki/Monomorphization)*
* You can also `impl Trait` in the return position.

## Using Traits Statically: Example

```rust []
trait HasArea {
    fn area_m2(&self) -> f64;
}

struct AreaCalculator {
    area_m2: f64
}

impl AreaCalculator {
    // Multiple symbols may be generated by this function
    fn add(&mut self, shape: impl HasArea) {
        self.area_m2 += shape.area_m2();
    }

    fn total(&self) -> impl std::fmt::Display {
        self.area_m2
    }
}
```

Note:

The total function says "I will give you a value you can display (with `println`), but I am not telling you what it is". You can look up "RPIT" (return position impl trait) for the history of this feature. APIT (argument position impl trait) is probably the less useful of the two.

## Using Traits Dynamically

* Rust also supports *trait references*
* The types are given at run-time through a *vtable*
* The reference is now a *wide pointer*

## Using Traits Dynamically: Example

```rust []
trait HasArea {
    fn area_m2(&self) -> f64;
}

struct AreaCalculator {
    area_m2: f64
}

impl AreaCalculator {
    // Only one symbol is generated by this function. The reference contains
    // a pointer to the table, *and* a pointer to a function table.
    fn add(&mut self, shape: &dyn HasArea) {
        self.area_m2 += shape.area_m2();
    }

    fn total(&self) -> &dyn std::fmt::Display {
        &self.area_m2
    }
}
```

Note:

In earlier editions, it was just `&Trait`, [but it was changed to `&dyn Trait`](https://rust-lang.github.io/rfcs/2113-dyn-trait-syntax.html)

## Which is better?

*Monomorphisation*? Or *Polymorphism*?

## Requiring other Traits

* Traits can also *require* other traits to also be implemented

```rust []
trait Printable: std::fmt::Debug { 
    fn print(&self) {
        println!("I am {:?}", self);
    }
}
```

## Special Traits

* Some traits have no functions (`Copy`, `Send`, `Sync`, etc)
  * But code can require that the trait is implemented
  * More in this in generics!
* Traits can be marked `unsafe`
  * Must use the `unsafe` keyword to implement
  * They're telling you to read the instructions!
