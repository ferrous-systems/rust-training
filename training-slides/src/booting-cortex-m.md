# Booting a Cortex-M Microcontroller

---

In this deck, we're talking specifically about Arm Cortex-M based microcontrollers.

Other Arm processors, and processors from other companies may vary.

## Terms

* Processor - the core that executes instructions
* SoC - the *system-on-a-chip* that contains a processor, some peripherals, and usually some memory
* Flash - the *flash memory* that the code and the constants live in
* RAM - the *random-access memory* that the global variables, heap and stack live in

## An example

* Arm Cortex-M4 - a processor core from Arm
  * Use the `thumbv7em-none-eabi` or `thumbv7em-none-eabihf` targets
* nRF52840 - a SoC from Nordic Semi that uses that processor core

## An example (2)

* Arm Cortex-M0+ - a smaller, simpler, processor core from Arm
  * Use the `thumbv6m-none-eabi` target
* RP2040 - a SoC from Raspberry Pi that uses *two* of those processor cores

## Booting a Cortex-M

The [Arm Architecture Reference Manual](https://developer.arm.com/documentation/ddi0403/ee/?lang=en) explains:

* The CPU boots at a well-defined address
* That word should contain a 32-bit RAM address for the stack pointer
* The word after should contain a 32-bit code address for the 'Reset' function
* The following 14 32-bit words are the exception handlers
* After that comes words for each interrupt handler
 
The chip does everything else.

## The steps

1. Make an array, or struct, with those two (or more) words in it
2. Convince the linker to put it at the right memory address
3. Profit

## C vector table 

```c
__attribute__ ((section(".nvic_table"))) unsigned long myvectors[] =
{
    (unsigned long) &_stack_top,
    (unsigned long) rst_handler, 
    (unsigned long) nmi_handler, 
    // ...
}
```

## Rust vector table

```rust ignore
#[link_section=".nvic_table"]
#[no_mangle]
pub static ISR_VECTORS: [Option<Handler>; 155] = [
    Some(_stack_top),
    Some(rst_handler),
    Some(nmi_handler),
    // ...
]
```

Note:

The cortex-m-rt crate does it more nicely than this. Stuffing the `_stack_top` address in an array of function-pointers - yuck!

## C Reset Handler

Can be written in C! But it's hazardous.

```c
extern unsigned long _start_data_flash, _start_data, _end_data;
extern unsigned long _bss_start, _bss_end;

void rst_handler(void) {
    unsigned long *src = &_start_data_flash;
    unsigned long *dest = &_start_data;
    while (dest < &_end_data) {
        *dest++ = *src++;
    }
    dest = &_bss_start,
    while (dest < &_bss_end) {
        *dest++ = 0;
    }
    main();
    while(1) { }
}
```

Note:

Global variables are not initialised when this function is executed. What if the C code touches an uninitialised global variable? C programmers don't worry so much about this. Rust programmers definitely worry about this.

## Rust Reset Handler (1)

```rust ignore
extern "C" {
    static mut _start_data_flash: usize;
    static mut _start_data: usize;
    static mut _end_data: usize;
    static mut _bss_start: usize;
    static mut _bss_end: usize;
}
```

## Rust Reset Handler (2)

```rust ignore
#[no_mangle]
pub unsafe extern "C" fn rst_handler() {
    let mut src: *mut usize = &mut _start_data_flash;
    let mut dest: *mut usize = &mut _start_data;
    while dest < &mut _end_data as *mut usize {
        dest.volatile_write(src.read());
        dest = dest.add(1);
        src = src.add(1);
    }
    dest = &mut _bss_start as *mut usize;
    while dest < &mut _end_data as *mut usize {
        dest.volatile_write(0);
        dest = dest.add(1);
    }
    main();
}
```

Note:

This is technically undefined behaviour because globals haven't been initialised yet.

## Linker scripts

* In Rust, they work exactly like they do in C.
* Same `.text`, `.rodata`, `.data`, `.bss` sections

## The cortex-m-rt crate

Does all this work for you, in raw Arm assembly language to avoid UB.

See [Reset](https://github.com/rust-embedded/cortex-m/blob/c-m-rt-v0.7.3/cortex-m-rt/src/lib.rs#L501), [Linker script](https://github.com/rust-embedded/cortex-m/blob/c-m-rt-v0.7.3/cortex-m-rt/link.x.in), and [Vector table](https://github.com/rust-embedded/cortex-m/blob/c-m-rt-v0.7.3/cortex-m-rt/src/lib.rs#L1130)

## The #[entry] macro

* Attaches your `fn main()` to the reset function in cmrt
* Hides your `fn main()` so no-one else can call it
* Remaps `static mut FOO: T` to `static FOO: &mut T` so they are safe

## Using the crate

See [Cortex-M Quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
