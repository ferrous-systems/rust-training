# Overview of Bare-Metal Rust

## A Layered Approach

When building bare-metal Systems in Rust, we use Rust crates to help us build a modular system.

The elements in our system are:

* The program you are writing
* The MCU are running on
* The PCB (or Board) your MCU is on
* The external devices connected to your MCU

## The Layers

To support these elements, we (usually) have these layers.

* Application
* Board Support
* External Drivers (e.g. SPI LCD Driver)
* Hardware Abstraction Layer Traits
* MCU Hardware Abstraction Layer Implementation
* MCU Peripheral Access Crate
* Core Peripherals
* Core Runtime

---

```dot process
digraph {
    node [shape=oval, width=1.5, fillcolor=khaki1, style=filled];
    app [label="Application\n(my_application)"]

    node [shape=record, width=1.5, fillcolor=lightblue, style=filled];
    bsc [label="Board Support\n(nrf52840_dk)"]
    hal [label="MCU HAL Implementation\n(nrf52480_hal)"]
    pac [label="MCU PAC\n(nrf52840-pac)"]

    node [shape=record, width=1.5, fillcolor=orange, style=filled];
    lcd_driver [label="SPI LCD Driver\n(ssd1306)"]

    node [shape=folder, width=1.5, fillcolor=green3, style=filled];
    hal_traits [label="HAL Traits\n(embedded_hal)"]

    node [shape=record, width=1.5, fillcolor=green3, style=filled];
    rt [label="Core Runtime\n(cortex_m_rt)"]
    cp [label="Core Peripherals\n(cortex_m)"]

    app -> bsc;
    bsc -> hal;
    bsc -> hal_traits;
    app -> lcd_driver;
    lcd_driver -> hal_traits;
    hal -> hal_traits [label="implements", style="dashed"]
    hal -> pac;
    app -> rt;
    pac -> rt;
    pac -> cp;
    rt -> cp;
}
```

## Don't worry

There's a lot here. We're going to take it step by step, starting at the bottom.
