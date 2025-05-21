# The Embedded HAL and its implementations

## These things are different

* STM32F030 I²C Driver
* nRF52840 I²C Driver
* But I want to write a library which is generic!
  * e.g. a Sensor Driver

## How does Rust allow generic behaviour?

* Generics!
* `where T: SomeTrait`

## Traits

An example:

```rust
pub trait I2c {
    type Error;

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error>;
}
```

## My Library

```rust ignore []
struct Co2Sensor<T> {
    i2c_bus: T,
    ...
}

impl<T> Co2Sensor<T> where T: I2c {
    fn new(i2c_bus: T) -> Co2Sensor<T> { ... }
    fn read_sensor(&mut self) -> Result<f32, Error> { ... }
}
```

Note how `Co2Sensor` *owns* the value whose type implements the `I2c` trait.

## My Application

```rust ignore []
let i2c = stm32f0xx_hal::i2c::i2c1(...);
let sensor = sensor_lib::Co2Sensor::new(i2c);
let Ok(reading) = sensor.read_sensor() else {
    // did you unplug it?
};
```

## My Application (2)

```rust ignore []
let i2c = nrf52840_hal::twim::Twim::new(...);
let sensor = sensor_lib::Co2Sensor::new(i2c);
let Ok(reading) = sensor.read_sensor() else {
    // did you unplug it?
};

```

## How do we agree on the traits?

* The Rust Embedded Working Group has developed some traits
* They are called the *Embedded HAL*
* See <https://docs.rs/embedded-hal>
* All HAL implementations should implement these traits

## Blocking vs Non-blocking

* Should a trait API stall your CPU until the data is ready?
* Or should it return early, saying "not yet ready"
  * So you can go and do something else in the mean time?
  * Or sleep?
* Or should it be an `async fn`

## Blocking vs Non-blocking

* https://crates.io/crates/embedded-hal
* https://crates.io/crates/embedded-hal-nb
* https://crates.io/crates/embedded-hal-async

## Trade-offs

* Some MCUs have more features than others
* The trait design has an inherent trade-off
  * Flexibility/Performance vs Portability
