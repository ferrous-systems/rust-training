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

```mermaid
graph TB
    app(Application<br/><tt>my_application</tt>)
    bsc[Board Support<br/><tt>nrf52840_dk</tt>]
    hal[MCU HAL Implementation<br/><tt>nrf52480_hal</tt>]
    lcd_driver[SPI LCD Driver<br/><tt>ssd1306</tt>]
    hal_traits[[HAL Traits<br/><tt>embedded_hal</tt>]]
    pac[MCU PAC<br/><tt>nrf52840</tt>]
    rt[Core Runtime<br/><tt>cortex_m_rt</tt>]
    cp[Core Peripherals<br/><tt>cortex_m</tt>]

    subgraph Key
        note1[Embedded Working Group]
        note2[nrf-rs]
        note3[You]
        note4[Others]
    end

    direction TB
    app --> bsc
    app & bsc --> hal
    app --> lcd_driver
    app & lcd_driver --> hal_traits
    hal -- Implements --o hal_traits
    app & hal --> pac
    app & pac --> rt
    app & pac & rt --> cp

    class app binary;
    class bsc library;
    class lcd_driver library;
    class hal mcu_library;
    class pac mcu_library;
    class hal_traits ewg_library;
    class rt ewg_library;
    class cp ewg_library;

    class note1 ewg_library;
    class note2 mcu_library;
    class note3 binary;
    class note4 library;

    classDef binary fill:#fb8,stroke:#333,stroke-width:4px;
    classDef library fill:#cf9,stroke:#333,stroke-width:2px;
    classDef ewg_library fill:#f9c,stroke:#333,stroke-width:2px;
    classDef mcu_library fill:#9cf,stroke:#333,stroke-width:2px;
```

## Don't worry

There's a lot here. We're going to take it step by step, starting at the bottom.
