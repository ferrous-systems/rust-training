# PACs and svd2rust

## Introduction

The *Peripheral Access Crate* crate sits near the bottom of the 'stack'. It
provides access to the memory-mapped peripherals in your MCU.

## Memory Mapped Peripherals

* e.g. a UART peripheral
* Has registers, represented by a memory address
* Registers are usually consecutive in memory (not always)
* Peripherals can have instances (same layout of registers, different start address)
  * UART0, UART1, etc

## Registers

* *Registers* are comprised of one or more *fields*.
* Each field is at least 1 bit in length.
* Sometimes fields can only take from a limited set of values
* This is all in your datasheet!

## C Code

Embedded Code in C often uses shifts and bitwise-AND to make up registers from
fields.

```c []
#define UARTE_INTEN_CTS_SHIFT (0)
#define UARTE_INTEN_CTS_MASK (0x00000001)
#define UARTE_INTEN_RXRDY_SHIFT (2)
#define UARTE_INTEN_RXRDY_MASK (0x00000001)

// The other nine fields are skipped for brevity
uint32_t cts = 0;
uint32_t rxrdy = 1;

uint32_t inten_value = ((cts & UARTE_INTEN_CTS_MASK) << UARTE_INTEN_CTS_SHIFT)
    | ((rxrdy & UARTE_INTEN_RXRDY_MASK) << UARTE_INTEN_RXRDY_SHIFT);

*((volatile uint32_t*) 0x40002300) = inten_value;
```

## Rust Code

You *could* do this in Rust if you wanted...

```rust ignore
const UARTE0_INTEN: *mut u32 = 0x4000_2300 as *mut u32;
unsafe { UARTE0_INTEN.write_volatile(0x0000_0003); }
```

But this still seems very error-prone. Nothing stops you putting the wrong value
at the wrong address.

## Adding structure

In C, the various registers for a peripheral can also be grouped into a `struct`

```c []
typedef volatile struct uart0_reg_t {
    uint32_t tasks_startrx; // @ 0x000
    uint32_t tasks_stoprx; // @ 0x004
    // ...
    uint32_t inten; // @ 0x300
    uint32_t _padding[79]; 
    uint32_t baudrate; // @ 0x500
} uart0_reg_t

uart0_reg_t* const p_uart = (uart0_reg_t*) 0x40002000;
```

## Structures in Rust

We can do that too (and this is how our PAC works under the hood).

```rust [] ignore
pub struct RegisterBlock {
    pub tasks_startrx: VolatileCell<u32>, // @ 0x000
    pub tasks_stoprx: VolatileCell<u32>, // @ 0x004
    // ...
    pub inten: VolatileCell<u32>, // @ 0x300
    _reserved12: [u32; 79],
    pub baudrate: VolatileCell<u32>, // @ 0x500
}

let p_uart: &RegisterBlock = unsafe { &*(0x40002000 as *const RegisterBlock) };    
```

We use the
[`VolatileCell`](https://docs.rs/vcell/0.1.3/vcell/struct.VolatileCell.html) to
ensure reads/writes on the structure fields are always performed with volatile
pointer read/writes.

## Other approaches

This is shown to work in practice, but some developers prefer to generate a
pointer for each peripheral register.

```rust [] ignore
pub struct Uart { base: *mut u32 }

impl Uart {
    fn write_tasks_stoprx(&mut self, value: u32) {
        let ptr = base.offset(1);
        unsafe { ptr.volatile_write(value) }
    }

    fn read_baudrate(&self) -> u32 {
        let ptr = base.offset(0x140);
        unsafe { ptr.volatile_read() }
    }
}

let uart = Uart { base: 0x40002000 as *mut u32 };
```

Note:

There is some discussion about whether `VolatileCell` technically breaks Rust's
rules around references.

The pointer is a `*mut u32` so the offsets are all in 32-bit words, not bytes.

## CMSIS-SVD Files

A CMSIS-SVD (or just SVD) file is an XML description of all the peripherals,
registers and fields on an MCU.

We can use `svd2rust` to turn this into a Peripheral Access Crate.

```mermaid
graph LR
    svd[(SVD XML)] --> svd2rust[<tt>svd2rust</tt>] --> rust[(Rust Source)]
```

## The `svd2rust` generated API

```mermaid
graph TB
    Peripherals --> uarte1[.UARTE1: <b>UARTE1&nbsp;</b>]
    uarte1 --> uart1_baudrate[.baudrate: <b>BAUDRATE&nbsp;</b>]
    uarte1 --> uart1_inten[.inten: <b>INTEN&nbsp;</b>]
    Peripherals --> uarte2[.UARTE2: <b>UARTE2&nbsp;</b>]
    uarte2 --> uart2_baudrate[.baudrate: <b>BAUDRATE&nbsp;</b>]
    uarte2 --> uart2_inten[.inten: <b>INTEN&nbsp;</b>]
```

---

* The crate has a top-level `struct Peripherals` with members for each *Peripheral*
* Each *Peripheral* gets a `struct`, like `UARTE0`, `SPI1`, etc.
* Each *Peripheral* `struct` has members for each *Register*
* Each *Register* gets a `struct`, like `BAUDRATE`, `INTEN`, etc.
* Each *Register* `struct` has `read()`, `write()` and `modify()` methods

## The `svd2rust` generated API (2)

* The `read()` method returns a special proxy object, with methods for each *Field*
* The `write()` method takes a closure, which is given a special 'proxy' object, with methods for each *Field*
  * All the *Field* changes are batched together and written in one go
  * Any un-written *Fields* are set to a default value
* The `modify()` method gives you both
  * Any un-written *Fields* are left alone

## Using a PAC

```rust [] ignore
// nrf52840 is the PAC
let p = nrf52840::Peripherals::take().unwrap();
// Reading the 'baudrate' field
let current_baud_rate = p.UARTE1.baudrate.read().baudrate();
// Modifying multiple fields in one go
p.UARTE1.inten.modify(|_r, w| {
    w.cts().enabled();
    w.ncts().enabled();
    w.rxrdy().enabled();
    w    
});
```

## Wait, what's a closure?

* It's an anonymous function, declared in-line with your other code
* It can 'capture' local variables (although we don't use that feature here)
* It enables a very powerful Rust idiom, that you can't easily do in C...

## Let's take it in turns

* I, the callee, need to set some stuff up
* You, the caller, need to do a bit of work
* I, the callee, need to clean everything up

We can use a closure to insert the caller-provided code in the middle of our
function. We see this used [all
(1)](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.map) [over
(2)](https://doc.rust-lang.org/core/primitive.str.html#method.matches) [the
(3)](https://doc.rust-lang.org/std/thread/fn.spawn.html) Rust standard library!

## Quiz time

What are the three steps here?

```rust [] ignore
p.UARTE1.inten.modify(|_r, w| {
    w.cts().enabled();
    w.ncts().enabled();
    w.rxrdy().enabled();
    w    
});
```

Note:

1. Read the peripheral MMIO register contents as an integer
2. Call the closure to modify the integer
3. Write the integer back to the peripheral MMIO register

## Documentation

Docs can be generated from the source code.

See <https://docs.rs/nrf52840>

Note that `uarte0` is a *module* and `UARTE0` could mean either a `struct` type,
or a field on the `Peripherals` struct.
