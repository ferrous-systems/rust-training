# Writing Drivers

---

* Writing to all those registers is tedious
  * You have to get the values right, and the order right
* Can we wrap it up into a nicer, easier-to-use object?

## Typical driver interface

```rust [] ignore
let p = pac::Peripherals.take().unwrap();
let mut uarte0 = hal::uarte::Uarte::new(
    // Our singleton representing exclusive access to
    // the peripheral IP block
    p.UARTE0,
    // Some other settings we might need
    115200,
    hal::uarte::Parity::None,
    hal::uarte::Handshaking::None,
);
// Using the `uarte0` object:
uarte0.write_all(b"Hey, I'm using a UART!").unwrap();
```

## The Hardware Abstraction Layer

* Contains all the drivers for a chip
* Often common/shared across chip families
  * e.g. nRF52 HAL for 52832, 52840, etc
* Usually community developed
* Often quite different between MCU vendors
  * Different teams came up with different designs!

## Kinds of driver

* PLL / Clock Configuration
* Reset / Power Control of Peripherals
* GPIO pins
* UART
* SPI
* I²C
* ADC
* Timer/Counters
* and more!

## Handling GPIO pins with code

```rust [] ignore
// Get the singletons
let p = pac::Peripherals.take().unwrap();
// Make a driver for GPIO port P0
let pins = hal::gpio::p0::Parts::new(p.P0);
// Get Pin 13 on port P0 and make it an output
let mut led_pin = pins.p0_13.into_push_pull_output(Level::High);
// Now set the output low
led_pin.set_low();
```

This differs widely across MCUs (ST, Nordic, Espressif, Atmel, etc). Some MCUs
(e.g. Nordic) let you put any function on any pin, and some are much more
restrictive!

## Correctness by design

* HALs want to make it hard to do the wrong thing
* Is a UART driver any use, if you haven't configured at least one TX pin and one RX pin?
* Should the UART driver check you've done that?

## Giving the pins to the driver

```rust [] ignore
// 'degrade()' converts a P0_08 type into a generic Pin type.
let pins =  hal::uarte::Pins {
    rxd: pins.p0_08.degrade().into_floating_input(),
    txd: pins.p0_06.degrade().into_push_pull_output(Level::High),
    cts: None,
    rts: None,
};

let uarte = hal::uarte::Uarte::new(
    periph.UARTE1, pins, Parity::EXCLUDED, Baudrate::BAUD115200
);
```

This is example is for the nRF52, as used in some of our examples.
