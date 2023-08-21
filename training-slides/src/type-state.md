# Using Types to encode State

## Systems have state

The system state is the product of all the things in the system that can be varied.

State can often be sub-divided into smaller units - some independent, some connected.

## Examples?

A GPIO pin on a microcontroller. It typically has:

* An output driver, that allows it to drive current out of the pin (or not)
* An input buffer, that allows the CPU to read the state of the pin
* An output level (high or low)

## Functionality can depend on state

Is this program correct?

```rust [] ignore
let p = GpioPin::new(7);
if p.is_low() {
    println!("Button is pressed");
}
```

Note:

* What if the pin defaults to "output mode"?
* What does it mean to read the level of a pin in output mode?

## Ignoring the problem

You don't *have* to solve this problem.

See, Arduino, which happily uses `int` for GPIO pin IDs, not values of custom
types.

## But we can do better?

We've got a type system with traits and a powerful static analysis engine...

```rust [] ignore
let p = OutputPin::new(7);
if p.is_low() {
    println!("Button is pressed");
}
```

```text
1 | struct OutputPin {}
  | ---------------- method `is_low` not found for this struct
...
9 |     if p.is_low() {
  |          ^^^^^^ method not found in `OutputPin`
```

## How would you change state?

With a method that takes ownership:

```rust [] ignore
impl OutputPin {
    fn into_input(self) -> InputPin {
        poke_hardware_registers();
        InputPin { self.pin_id }        
    }
}

impl InputPin {
    fn into_output(self) -> OutputPin {
        poke_hardware_registers();
        OutputPin { self.pin_id }        
    }
}
```

Note:

The function call `poke_hardware_registers()` is a placeholder for whatever work
you need to do on that microcontroller to change the state of that pin.

## Non-Zero Sized Types

This type consumes 1 byte of RAM (maybe 4 bytes, with alignment). Is that
strictly required?

```rust
struct OutputPin {
    pin_id: u8
}
```

## Zero Sized Types

This type is of zero size. But any method call on it has access to the pin number,
through the type system.

```rust
struct OutputPin<const PIN: u8> { _inner: () }

impl<const PIN: u8> OutputPin<PIN> {
    fn print_id(&self) {
        println!("I am pin {}", PIN);
    }
}

fn main() {
    let p: OutputPin<5> = OutputPin { _inner: () };
    p.print_id();
    println!("size is {}", std::mem::size_of_val(&p));
}
```

Note:

The `_inner` field is not pub, and therefore ensures values of this type can't
be constructed outside the module it was defined in. This forces people to use
the `new` functions you provide!

## Generic Pin Modes?

```rust []
pub trait PinMode {}

pub struct Output {}
impl PinMode for Output {}

pub struct Input {}
impl PinMode for Input {}

pub struct Pin<MODE> where MODE: PinMode { mode: MODE }

impl Pin<Output> {
    pub fn set_high(&self) { }
    pub fn set_low(&self) { }
}

impl Pin<Input> {
    pub fn is_high(&self) -> bool { todo!() }
    pub fn is_low(&self) -> bool { todo!() }
}
```

## Preventing mis-use.

Who can `impl PinMode for Type`? Turns out anyone can...

```rust [] ignore
use my_driver_crate::{Pin, PinMode};

struct OnFire {}
impl PinMode for OnFire {}

let pin: Pin<OnFire> = ...;
```

## Sealing traits

```rust []
mod private { pub trait Sealed {} }
pub trait PinMode: private::Sealed {}

pub struct Output {}
impl PinMode for Output {}
impl private::Sealed for Output {}

pub struct Input {}
impl PinMode for Input {}
impl private::Sealed for Input {}
```

Note:

The 'private' module is not `pub`, but the trait within it is `pub`. This means
you cannot implement the `PinMode` trait yourself unless you can also 'see' a
path to the `private::Sealed` trait - which is only visible within this
module.

It's a trick to ensure only this module can implement the trait, but anyone else
can see the trait and which types implement it.
