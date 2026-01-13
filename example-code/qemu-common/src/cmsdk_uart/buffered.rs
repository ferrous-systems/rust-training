//! An interrupt-driven buffered CMSDK UART driver
//!
//! The CMSDK UART will fire an interrupt when the TX FIFO goes from full to not full.

use core::cell::RefCell;
use core::convert::Infallible;

use super::registers::IntStatus;

use super::{CmsdkUart, Error};

/// Our context, stored inside a lock
struct Inner<const QLEN: usize> {
    /// Our UART
    uart: CmsdkUart,
    /// Our transmission buffer
    tx_buffer: heapless::spsc::Queue<u8, QLEN>,
    /// Our reception buffer
    rx_buffer: heapless::spsc::Queue<u8, QLEN>,
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
            uart.enable_rx_interrupt();
            guard.replace(Inner {
                uart,
                tx_buffer: heapless::spsc::Queue::new(),
                rx_buffer: heapless::spsc::Queue::new(),
            });
        });
        Ok(())
    }

    /// Read the available buffered bytes into the provided buffer.
    ///
    /// Returns the number of read bytes.
    pub fn read(&self, buf: &mut [u8]) -> usize {
        self.with(|inner| {
            let rx_count = inner.rx_buffer.len();
            if rx_count == 0 {
                return 0;
            }
            let rx_count = core::cmp::min(rx_count, buf.len());
            for b in buf.iter_mut().take(rx_count) {
                // We checked that at least rx_count bytes are available and
                // the CS ensures we haven't been interrupted since to change
                // that count.
                let byte = inner.rx_buffer.dequeue().unwrap();
                defmt::debug!("< RXQ 0x{=u8:02x}", byte);
                *b = byte;
            }
            rx_count
        })
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
            let finished = self.with(|inner| {
                // If TX interrupts aren't on, turn them on. Because we're in a CS,
                // we can't be interrupted between that buffer enqueue and turning
                // interrupts on
                if !inner.uart.registers.read_control().txie() {
                    defmt::debug!("> TXQ 0x{=u8:02x}, TXIE on", byte);
                    // this will fire the ISR when the byte has finished sending
                    inner.uart.registers.modify_control(|c| c.with_txie(true));
                    // send the byte to the UART - we know it won't fail
                    // because our TX interrupt was off indicating that there
                    // is no TX in progress.
                    _ = inner.uart.write(byte);
                    // Unfortunately QEMU doesn't model the delay in sending
                    // bytes, so the TX ISR will fire at this point and TXIE
                    // will be turned off - meaning we never actually queue
                    // data to send. But if you priority mask the TX ISR
                    // temporarily, you can see this code working.
                    true
                } else if !inner.tx_buffer.is_full() {
                    // TX interrupts are on, so we are in the middle of sending a byte.
                    // Queue this byte, to send when the previous one has finished
                    defmt::debug!("> TXQ 0x{=u8:02x}", byte);
                    unsafe {
                        inner.tx_buffer.enqueue_unchecked(byte);
                    }
                    true
                } else {
                    // buffer is full ... we need to try again
                    false
                }
            });
            if finished {
                break;
            }
        }
    }

    /// Block until all bytes are gone
    pub fn flush(&self) {
        while self.with(|inner| inner.tx_buffer.len()) != 0 {
            core::hint::spin_loop();
        }
        while self.with(|inner| inner.uart.registers.read_status().txf()) {
            core::hint::spin_loop();
        }
    }

    /// UART TX IRQ handler
    ///
    /// Checks if the TX interrupt flag is set, and if so, loads as much
    /// data as it can into the UART, and turns off the TX interrupt if
    /// the buffer runs out.
    pub fn tx_isr(&self) {
        const TXI_FLAG: IntStatus = IntStatus::DEFAULT.with_txi(true);
        defmt::debug!("- TX ISR");
        self.with(|inner| {
            let int_status = inner.uart.registers.read_int_status();
            if int_status.txi() {
                inner.uart.clear_interrupts(TXI_FLAG);
                while !inner.uart.registers.read_status().txf() && !inner.tx_buffer.is_empty() {
                    // UART is not full - load UART with next byte
                    let byte = unsafe { inner.tx_buffer.dequeue_unchecked() };
                    defmt::debug!("> TX 0x{=u8:02x}", byte);
                    inner.uart.write(byte).expect("TX space");
                }
                if inner.tx_buffer.is_empty() {
                    // cancel TX interrupt
                    defmt::debug!("- TX buffer empty ... turning TXIE off");
                    inner.uart.registers.modify_control(|c| c.with_txie(false));
                }
            }
        });
    }

    /// UART RX IRQ handler
    ///
    /// Checks if the RX interrupt flag is set, and if so, loads as much
    /// data as it can into the UART, and turns off the RX interrupt if
    /// the buffer runs out.
    pub fn rx_isr(&self) {
        const RXI_FLAG: IntStatus = IntStatus::DEFAULT.with_rxi(true);
        defmt::debug!("- RX ISR");
        self.with(|inner| {
            let int_status = inner.uart.registers.read_int_status();
            if int_status.rxi() {
                inner.uart.clear_interrupts(RXI_FLAG);
                // Drop old data if buffer full.
                if inner.rx_buffer.is_full() {
                    // Buffer is full so dequeing one byte should work.
                    let _ = inner.rx_buffer.dequeue().unwrap();
                }
                let byte = inner.uart.registers.read_data() as u8;
                defmt::debug!("< RX 0x{=u8:02x}", byte);
                // We guaranteed that there is space.
                inner.rx_buffer.enqueue(byte).unwrap();
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

impl<const QLEN: usize> embedded_io::ErrorType for BufferedUart<QLEN> {
    type Error = Infallible;
}

impl<const QLEN: usize> embedded_io::Write for BufferedUart<QLEN> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // convert from &mut BufferedUart to &BufferedUart
        let mut uart = &*self;
        // call the impl on &BufferedUart
        <&BufferedUart<QLEN> as embedded_io::Write>::write(&mut uart, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // convert from &mut BufferedUart to &BufferedUart
        let mut uart = &*self;
        // call the impl on &BufferedUart
        <&BufferedUart<QLEN> as embedded_io::Write>::flush(&mut uart)
    }
}

impl<const QLEN: usize> embedded_io::ErrorType for &BufferedUart<QLEN> {
    type Error = Infallible;
}

impl<const QLEN: usize> embedded_io::Write for &BufferedUart<QLEN> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx_blocking(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.tx_blocking(buf);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // convert from &mut BufferedUart to &BufferedUart
        let uart = &*self;
        uart.flush();
        Ok(())
    }
}

impl<const QLEN: usize> embedded_io::Read for BufferedUart<QLEN> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(BufferedUart::read(self, buf))
    }
}

impl<const QLEN: usize> embedded_io::Read for &BufferedUart<QLEN> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(BufferedUart::read(self, buf))
    }
}

// End of file
