//! An interrupt-driven buffered CMSDK UART driver
//!
//! The CMSDK interrupt will fire an interrupt when the TX FIFO goes from full to not full.
//!
//! The trick is what we do when we send a byte and the UART is not currently sending.

use core::cell::RefCell;

use crate::uart::IntStatus;

use super::{CmsdkUart, Control, Error, Status};

/// Our context, stored inside a lock
struct Inner<const QLEN: usize> {
    /// Our UART
    uart: CmsdkUart,
    /// Our buffer
    buffer: heapless::spsc::Queue<u8, QLEN>,
}

/// A CMSDK UART with a buffer
pub struct BufferedUart<const QLEN: usize> {
    inner: critical_section::Mutex<RefCell<Option<Inner<QLEN>>>>,
}

impl<const QLEN: usize> BufferedUart<QLEN> {
    /// Make a new, empty, driver
    pub const fn empty() -> Self {
        Self {
            inner: critical_section::Mutex::new(RefCell::new(None)),
        }
    }

    /// Initialise this global UART.
    ///
    /// Pass in a `CmsdkUart` and it will be stored within and available at a later time.
    pub fn init(
        &self,
        mut uart: CmsdkUart,
        baud_rate: u32,
        system_clock: u32,
    ) -> Result<(), Error> {
        uart.init(baud_rate, system_clock)?;
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            *guard = Some(Inner {
                uart,
                buffer: heapless::spsc::Queue::new(),
            });
        });
        Ok(())
    }

    /// Transmit a byte slice, blocking until done
    ///
    /// This might leave bytes in the buffer that haven't yet been sent.
    pub fn tx_blocking(&self, bytes: &[u8]) {
        for b in bytes {
            self.tx_byte_blocking(*b);
        }
    }

    /// Transmit a byte, blocking until queued
    pub fn tx_byte_blocking(&self, byte: u8) {
        loop {
            let is_full = self.with(|inner| inner.buffer.is_full());
            if is_full {
                // sleep and try again later
                defmt::debug!("Buffer full, sleeping...");
                cortex_m::asm::wfi();
            } else {
                break;
            }
        }
        // OK, we definitely have space now
        self.with(|inner| {
            // If TX interrupts aren't on, turn them on. Because we're in a CS,
            // we can't be interrupt between that buffer enqueue and turning
            // interrupts on
            if !inner.uart.read_control().contains(Control::TXIE) {
                defmt::debug!("Sending 0x{=u8:02x} and turning TXIE on", byte);
                inner.uart.modify_control(|c| {
                    c.set(Control::TXIE, true);
                });
                _ = inner.uart.write(byte);
            } else {
                // we know we have space - we checked earlier
                defmt::debug!("Queued byte 0x{=u8:02x}", byte);
                unsafe {
                    inner.buffer.enqueue_unchecked(byte);
                }
            }
        })
    }

    /// Block until all bytes are gone
    pub fn flush(&self) {
        loop {
            let len = self.with(|inner| inner.buffer.len());
            if len != 0 {
                // sleep and try again
                cortex_m::asm::wfi();
            } else {
                break;
            }
        }
        loop {
            let is_txing = self.with(|inner| inner.uart.read_status().contains(Status::TXF));
            if is_txing {
                // sleep and try again
                cortex_m::asm::wfi();
            } else {
                break;
            }
        }
    }

    /// UART TX IRQ handler
    ///
    /// # Safety
    ///
    /// Only call this from a UART TX interrupt routine
    pub unsafe fn tx_isr(&self) {
        defmt::debug!("TX ISR");
        self.with(|inner| {
            inner.uart.clear_interrupts(IntStatus::TXI);
            if inner.buffer.is_empty() {
                // cancel TX interrupt
                defmt::debug!("TXIE going off");
                inner.uart.modify_control(|c| {
                    c.set(Control::TXIE, false);
                });
            } else if !inner.uart.read_status().contains(Status::TXF) {
                // load UART with next byte
                let byte = unsafe { inner.buffer.dequeue_unchecked() };
                defmt::debug!("Auto send 0x{=u8:02x}", byte);
                inner.uart.write(byte).expect("TX space");
            } else {
                defmt::warn!("Duff ISR - TX is full");
            }
        });
    }

    fn with<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut Inner<QLEN>) -> T,
    {
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            let Some(inner) = guard.as_mut() else {
                panic!("UART not initialised!");
            };
            f(inner)
        })
    }
}

unsafe impl<const QLEN: usize> Sync for BufferedUart<QLEN> {}

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
