//! An interrupt-driven buffered CMSDK UART driver

use core::sync::atomic::{AtomicBool, Ordering};

use super::{CmsdkUart, Error, MutexUart};

/// A CMSDK UART with a buffer
pub struct BufferedUart<const QLEN: usize> {
    /// Our thread-safe UART
    inner: MutexUart,
    /// A buffer to put stuff in
    buffer: heapless::mpmc::MpMcQueue<u8, QLEN>,
    /// Are interrupts on or off?
    tx_interrupts: AtomicBool,
}

impl<const QLEN: usize> BufferedUart<QLEN> {
    /// Make a new, empty, driver
    pub const fn empty() -> BufferedUart<QLEN> {
        BufferedUart {
            inner: MutexUart::empty(),
            buffer: heapless::mpmc::MpMcQueue::new(),
            tx_interrupts: AtomicBool::new(false),
        }
    }

    /// Initialise this global UART.
    ///
    /// Pass in a `CmsdkUart` and it will be stored within and available at a later time.
    pub fn init(&self, uart: CmsdkUart, baud_rate: u32, system_clock: u32) -> Result<(), Error> {
        self.inner.init(uart, baud_rate, system_clock)
    }

    /// Call this from your TX interrupt handler
    pub fn tx_poll(&self) {
        if let Some(byte) = self.buffer.dequeue() {
            defmt::debug!("Send {=u8:02x}", byte);
            if self.inner.tx(byte).is_err() {
                defmt::warn!("BufferedUart bug - dropped byte");
            }
        } else {
            defmt::debug!("Buffer empty - TX interrupts off");
            self.disable_tx_interrupt();
        }
    }

    /// Send some data, sleeping while the buffer is full.
    pub fn tx_blocking(&self, data: &[u8]) {
        for byte in data {
            loop {
                if self.buffer.enqueue(*byte).is_err() {
                    // sleep until next interrupt
                    self.enable_tx_interrupt();
                    self.inner.dump_info();
                    cortex_m::asm::wfe();
                } else {
                    defmt::debug!("Queued {=u8:02x}", *byte);
                    break;
                }
            }
            self.enable_tx_interrupt();
        }
    }

    /// Turn on TX interrupts
    fn enable_tx_interrupt(&self) {
        if self
            .tx_interrupts
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            defmt::debug!("TX IRQ on");
            self.inner.tx_interrupts(true);
        }
    }

    /// Turn off TX interrupts
    fn disable_tx_interrupt(&self) {
        if self
            .tx_interrupts
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            self.inner.tx_interrupts(false);
            defmt::debug!("TX IRQ off");
        }
    }
}

impl<const QLEN: usize> core::fmt::Write for BufferedUart<QLEN> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // convert from &mut BufferedUart to &BufferedUart
        let mut uart = &*self;
        // call the impl on &BufferedUart
        <&BufferedUart<QLEN> as core::fmt::Write>::write_str(&mut uart, s)
    }
}

impl<const QLEN: usize> core::fmt::Write for &BufferedUart<QLEN> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.tx_blocking(s.as_bytes());
        Ok(())
    }
}

// End of file
