# The Shape of a Rust Program

---

* Embedded systems come in many shapes and sizes
* Rust tries to be flexible and support developers

## Some Terms

* Binary
* Static Library
* Dynamic Library
* RTOS

Note:

A binary is a collection of executable machine code and data, typically but not
exclusively in ELF format, with a defined 'entry point'. The CPU should jump to
the address of the 'entry point' and start executing from there.

A static library is an archive containing object code, typically with a `.a`
extension. The object code contains gaps where the run-time addresses need to be
plugged in by a linker, before it can be considered executable code.

A dynamic library looks more like a binary (and is typically in ELF format), but
it still contains gaps that need to be plugged by a dynamic linker (also known
as a loader). Linux `.so` files and Windows `.dll` files are in this category.

A Real-Time Operating System manages the execution of one or more tasks,
typically with pre-emptive context switching, but not exclusively.

## 1) Flat Binaries

* Top-level is a Rust Binary
  * Typically `main.rs`
* Program runs on start-up
  * Started by the reset vector, or the boot ROM
* Can pull in an RTOS or async runtime, as a static library
* Linker sees everything
* Flat address space
* The most common approach
* See [RTIC](https://rtic.rs/), [embassy](https://embassy.dev), [Eclipse ThreadX](https://ferrous-systems.com/blog/rust-and-threadx/), or [FreeRTOS](https://github.com/ferrous-systems/freertos-experiments)

## 2) Bootloader + Application

* Two binaries, linked separately
* First binary (e.g. bootloader) starts the second (e.g. application)
* Sometimes the second calls back into the first
* Use linker scripts to divide up memory
* Also often used to implement Arm Secure Mode (TrustZone) APIs
* See [RP2350 HAL](https://docs.rs/rp235x-hal) or [the nRF9160 SPM](https://github.com/nrf-rs/nrf9160-dk)

Note:

The RP2350 Bootloader is in ROM, but it's still a binary. It inspects the
application in flash (optionally performing a hash check or a cryptographic
signature check) before jumping to the application. The application can then
make calls back into the ROM bootloader, by calling a function that lives at a
well-known address (or that has a function pointer that is stored at a
well-known address). The bootloader in ROM starts in the Arm Cortex-M33's
'Secure' state, but can switch the CPU into 'non-secure' state before running
the application, if that's what the application metadata says to do.

The nRF9160 Secure Partition Manager is similar, but must be written to the
start of the nRF9160's flash. It also expects the exclusive use of a particular
block of SRAM and so you must avoid that region of SRAM in your application. See
the nrf9160-hal's `memory.x` file for an example.

## 3) Tasks are Libraries

* Each 'task' is a static library
* The OS provides a 'skeleton' binary
  * It imports and calls your tasks
* Tasks provide an entry point, and some mechanism to call the OS
  * Typically SVC calls
* See
  [Zephyr](https://zephyrproject.org/zephyr-weekly-update-rust-coming-to-zephyr/)
  and [RTEMS](https://docs.rtems.org/branches/master/user/rust/index.html)

Note:

SVC is the Arm mnemonic for performing a system call. These are also known as
'software interrupts' and earlier Arm architectures used the mnemonic SWI.

## 3) Tasks are Binaries (dynamic linking)

* Some systems have multiple 'flash slots'
  * The run-time address is not known at link time
* Enforces isolation between tasks - has to use SVC calls
* Rust does not currently support [RWPI] or [ROPI] code
* Rust has some support for [PIC/PIE] code
  * But then you have to write a dynamic linker for fix the code at load time
* See [TockOS](https://tockos.org) or Linux/macOS/Windows/QNX...

[RWPI]: https://developer.arm.com/documentation/dui0774/l/Compiler-Command-line-Options/-frwpi---fno-rwpi?lang=en
[ROPI]: https://developer.arm.com/documentation/dui0774/l/Compiler-Command-line-Options/-fropi---fno-ropi?lang=en
[PIC/PIE]: https://mropert.github.io/2018/02/02/pic_pie_sanitizers/

Note:

As of 2024, TockOS only allows Rust applications to be installed in the first
flash slot, for this reason. C applications can be installed into any flash
slot, because ROPI/RWPI works for C.

RWPI is read-write position independence, and involves static data not having a
fixed address but instead being accessed via a reserved register that always
contains the 'static base pointer' (i.e. the base address of the RW data).

ROPI is read-only position independence, and involves executable code not having
a fixed address but instead being accessed via PC-relative jumps.

PIC/PIE is position independent code / executable. This involves non-PC-relative
jumps to code or data being made via a Global Offset Table (GOT). The GOT needs
modifying at load time, once you know where everything is in memory. Linux
programs and shared libraries are PIE/PIC.

## 4) Tasks are Binaries (static linking)

* Like (3), but you have a tool work out the linking once you have all the
  binaries
* Doesn't require ROPI or RWPI
* But you have to know the full set of tasks in advance
* See [Hubris](https://hubris.oxide.computer)

## Summary

1. Flat Binaries
2. Bootloader + Application
3. Tasks are Libraries
4. Tasks are Binaries (dynamic linking)
5. Tasks are Binaries (static linking)

Remember, these are *embedded systems issues*, not necessarily *Rust-specific issues*.
