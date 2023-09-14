# The Embedded HAL and its implementations

## These things are different

* STM32F030 UART Driver
* nRF52840 UART Driver
* But I want to write a library which is generic!
  * e.g. an AT Command Parser

## How does Rust allow generic behaviour?

* Generics!
* `where T: SomeTrait`

## Traits

An example:

```rust
trait GenericSerial {
    type Error;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;
}
```

## My Library

```rust [] ignore
struct AtCommandParser<T> {
    uart: T,
    ...
}

impl<T> AtCommandParser<T> where T: GenericSerial {
    fn new(uart: T) -> AtCommandParser<T> { ... }
    fn get_command(&mut self) -> Result<Option<AtCommand>, Error> { ... }
}
```

Note how `AtCommandParser` *owns* the object which meets the `GenericSerial` trait.

## My Application

```rust [] ignore
let uart = stm32_hal::Uart::new(...);
let at_parser = at_library::AtCommandParser::new(uart);
while let Some(cmd) = at_parser.get_command().unwrap() {
    ...
}
```

## My Application (2)

```rust [] ignore
let uart = nrf52_hal::Uart::new(...);
let at_parser = at_library::AtCommandParser::new(uart);
while let Some(cmd) = at_parser.get_command().unwrap() {
    ...
}
```

## How do we agree on the traits?

* The Rust Embedded Working Group has developed some traits
* They are called the *Embedded HAL*
* See <https://docs.rs/embedded-hal>
* All HAL implementations should implement these traits

## Blocking vs Non-blocking

* Should a trait API stall your CPU until the data is ready?
* Or should it return early, saying "not yet ready"
  * So you can go an do something else in the mean time?
  * Or sleep?
* `embedded_hal::blocking::serial::Write`, vs
* `embedded_hal::serial::Write`

## Trade-offs

* Some MCUs have more features than others
* The trait design has an inherent trade-off
  * Flexibility/Performance vs Portability
