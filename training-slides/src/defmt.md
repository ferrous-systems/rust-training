# Using defmt

---

defmt is the *Deferred Formatter*

## Motivation

* You have a microcontroller
* You want to know what it is doing

## Classical Approach

* Set up a UART,
* have a function that writes logs to the UART, and
* instrument your code with logger calls.

```c
#define INFO(msg, ...) do { \
    if (g_level >= LEVEL_INFO) { \
        fprintf(g_uart, "INFO: " msg, __VA_ARGS__ ) \
    }  \
} while(0)

INFO("received %u bytes", rx_bytes);
```

## Downsides

* Code size - where do the strings live?
* Waiting for the UART

## An idea

* Who actually needs the strings?
* Your serial terminal
* Which is on your laptop...

---

Do the logging strings even need to be in Flash?

## defmt

* Deferred Formatting
* Strings are *interned* into a .defmt section
  * Is in the ELF file
  * Is not in Flash
* Arguments are packed in binary format
* Tools to reconstruct log messages on the host side

## Benefits

* Uses less flash space
* Less data to transfer over the wire

## Downsides

* Now you need a special viewer tool
* Which needs the *exact* ELF file your chip is running

## Example

```rust ignore
let rx_bytes = 300u16;
defmt::error!("received {=u16} bytes", rx_bytes);
```

<p>&nbsp;<!-- spacer for "run" button --></p>

This will transmit just: `[3, 44, 1]`

Note:

The string index we give here as `3`, and `44, 1` is 300 encoded as
little-endian bytes.

## Type Hints

The braces can contain `{[pos][=Type][:Display]}`:

* `pos`: a numeric argument position (e.g. `0`)
* `Type`: a type hint
* `Display`: a display hint

## More Examples

```rust ignore
defmt::info!("enabled: {=bool}, ready: {=bool}", enabled, ready);
// enabled: true, ready: false

defmt::trace!("{{ X: {0=0..8}, Y: {0=8..16}, Z: {0=16..19} }}", some_bitfield);
// { X: 125, Y: 3, Z: 2 }

defmt::error!("data = {=[u8]:#02x}", some_byte_slice)
// data = [0x00, 0x01, 0x02, 0x03]
```

Note:

The `x..y` syntax is the bitfield syntax. `[u8]` is the u8 slice syntax, and
`:#02x` means two-digit hex in the alternate (`0x`) style.

Using type hints can produce a more efficient encoding.

## Printing structs and enums

```rust
#[derive(Debug)]
struct Data {
    x: [u8; 5],
    y: f64
}

fn print(data: &Data) {
    println!("data = {:?}", data);
}
```

## Printing structs and enums with defmt

```rust ignore
#[derive(defmt::Format)]
struct Data {
    x: [u8; 5],
    y: f64
}

fn print(data: &Data) {
    defmt::info!("data = {=?}", data);
}
```

Note:

The `=?` is optional, as it is the default. It means *render this using the
defmt::Format trait*.

In defmt, there is not `Debug` vs `Display` distinction - it is up to the host
to decide how best to format the values.

## Optionally enabling defmt

* If a library uses `defmt::Format`, the application must set up a logger
* Portable libraries don't want this. Instead:

```rust ignore
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct Data {
    x: [u8; 5],
    y: f64
}
```

## A better transport

* UART is slow
* Background DMA from a ring-buffer is complicated to set up
* Can we do better?

## SEGGER RTT

* *Real Time Transport*
* Dedicated memory area
* Marked with magic numbers
* Can be found and read by your Debug Probe
* Without interrupting the CPU!
* High speed, near-zero-cost byte-pipe

## defmt-rtt

* Implement's SEGGER's RTT protocol
* Wired up as a defmt global logger
* Your *binary* just needs to:

```rust ignore
use defmt_rtt as _;
```

Note:

The `defmt` calls in your libraries are able to find the 'logging sink' created
by the `defmt-rtt` crate though the use of a type in `defmt-rtt` annotated with:

```rust ignore
#[defmt::global_logger]
```

This creates a bunch of `unsafe` `#[no_mangle]` functions, like:

```rust ignore
#[inline(never)]
#[no_mangle]
unsafe fn _defmt_acquire() {
    <Logger as defmt::Logger>::acquire()
}
```

## Log Level

You can control the log level at compile time with an environment variable:

```text
DEFMT_LOG=info cargo build
```

Note:

Windows users will use different syntax for cmd.exe vs Powershell.

## Host tools

* Knurling's `probe-run` was the first
* The `probe-rs` CLI now has support (recommended)
* Or use `defmt-print`

## Using probe-rs

<pre><code data-trim data-noescape>
$ probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/radio-puzzle-solution
<span class="eg b">      Erasing</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 35.52 KiB/s (eta 0s )
<span class="eg b">  Programming</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 49.90 KiB/s (eta 0s )
<span class="eg b">     Finished</span> in 0.79s
0 DEBUG Initializing the board
└─ dk::init @ /Users/jp/ferrous-systems/rust-exercises/nrf52-code/boards/dk/src/lib.rs:208
1 DEBUG Clocks configured
└─ dk::init @ /Users/jp/ferrous-systems/rust-exercises/nrf52-code/boards/dk/src/lib.rs:219
</code></pre>

## Customise the format

<pre><code data-trim data-noescape>
$ probe-rs run --chip nRF52840_xxAA ... --log-format oneline
<span class="eg b">      Erasing</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 35.52 KiB/s (eta 0s )
<span class="eg b">  Programming</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 49.90 KiB/s (eta 0s )
<span class="eg b">     Finished</span> in 0.79s
00:00:00.000000 <span class="b">[DEBUG]</span> Initializing the board (<span class="gr b">dk</span> dk/src/lib.rs:317)
00:00:00.000000 <span class="b">[DEBUG]</span> Clocks configured (<span class="gr b">dk</span> dk/src/lib.rs:335)
00:00:00.000000 <span class="b">[DEBUG]</span> RTC started (<span class="gr b">dk</span> dk/src/lib.rs:354)
</code></pre>

## Set it as your runner

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52840_xxAA"
```

<pre><code data-trim data-noescape>
$ cargo run
<span class="eg b">    Finished</span> dev [optimized + debuginfo] target(s) in 0.03s
<span class="eg b">     Running</span> `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/radio-puzzle-solution`
<span class="eg b">     Erasing</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 35.52 KiB/s (eta 0s )
<span class="eg b"> Programming</span> ✔ [00:00:00] [#########################] 16.00 KiB/16.00 KiB @ 49.90 KiB/s (eta 0s )
<span class="eg b">    Finished</span> in 0.79s
00:00:00.000000 <span class="b">[DEBUG]</span> Initializing the board (<span class="gr">dk</span> dk/src/lib.rs:317)
00:00:00.000000 <span class="b">[DEBUG]</span> Clocks configured (<span class="gr">dk</span> dk/src/lib.rs:335)
00:00:00.000000 <span class="b">[DEBUG]</span> RTC started (<span class="gr">dk</span> dk/src/lib.rs:354)
</code></pre>

## More info

There's a book!

<https://defmt.ferrous-systems.com>

## Re-entrancy

`defmt::info!` (etc) can be called anywhere, even from an interrupt.

How do you make that safe?

## Critical Sections

* defmt-rtt uses the `critical-section` crate
* More on this elsewhere
