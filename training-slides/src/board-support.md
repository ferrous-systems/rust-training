# Board Support Crates

## Using a 'normal' PC

* Did you tell your PC it had a mouse plugged in?
* Did you tell it what I/O address the video card was located at?
* No! It auto-discovers all of these things.
  * USB, PCI-Express, SATA all have "plug-and-play"

## Using an Embedded System

* There is (almost always) no plug-and-play
* Your MCU can put different functions (UART, SPI, etc) on different pins
* The choice of which function goes on which pin was decided by the PCB designer
* You now have to tell the software how the PCB was laid out
  * i.e UART0 TX is on Port 0, Pin 13

## A Board Support Crate

* You can wrap this up into a *Board Support Crate*
* Especially useful if you are using a widely available dev-kit
  * e.g. the nRF52840-DK, or the STM32 Discovery
* Still useful if the board design is an in-house one-off
* Create the drivers and does the pin assignments for you
* Helps make your application portable across different boards

## Using a Board Support Crate

[Link](../../example-code/nrf52/bsp_demo/)

```rust [] ignore
#[entry]
fn main() -> ! {
    let mut nrf52 = Board::take().unwrap();
    loop {
        writeln!(nrf52.cdc, "On!").unwrap();
        nrf52.leds.led_2.enable();
        writeln!(nrf52.cdc, "Off!").unwrap();
        nrf52.leds.led_2.disable();
    }
}
```

Note:

We don't have to configure the LED pins as outputs. We don't have to configure
the UART pins. The Board Support Crate did it all for us.

## Making a Board Support Crate

```rust [] ignore
pub struct Board {
    /// The nRF52's pins which are not otherwise occupied on the nRF52840-DK
    pub pins: Pins,
    /// The nRF52840-DK UART which is wired to the virtual USB CDC port
    pub cdc: Uarte<nrf52::UARTE0>,
    /// The LEDs on the nRF52840-DK board
    pub leds: Leds,
    ...
    /// nRF52 peripheral: PWM0
    pub PWM0: nrf52::PWM0,
    ...
}

impl Board {
  fn take() -> Option<Self> { todo!() }
  fn new(cp: CorePeripherals, p: Peripherals) -> Self { todo!() }
}
```

Note:

Because constructing the `Board` struct consumed *all* the peripherals from the
PAC, it's important to re-export the ones the BSC didn't use so that
applications can construct their own drivers using them,.

## More things to consider

* Does the MCU start-up on a slow internal oscillator?
* Are there jumpers to control routing on the board?
* SD Cards: should you pick a driver, or let them choose?
* Radios: same question!
