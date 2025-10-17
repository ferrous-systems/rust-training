//! Mutex wrapped CMSDK UART driver

use core::cell::RefCell;

use super::{Uart, Error};

/// A CMSDK UART you can store as a static variable
pub struct MutexUart {
    inner: critical_section::Mutex<RefCell<Option<Uart>>>,
}

impl MutexUart {
    /// Create a new, empty, placeholder.
    pub const fn empty() -> MutexUart {
        MutexUart {
            inner: critical_section::Mutex::new(RefCell::new(None)),
        }
    }

    /// Initialise this global UART.
    ///
    /// Pass in a `CmsdkUart` and it will be stored within and available at a later time.
    pub fn init(
        &self,
        mut uart: Uart,
        baud_rate: u32,
        system_clock: u32,
    ) -> Result<(), Error> {
        uart.init(baud_rate, system_clock)?;
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            guard.replace(uart);
        });
        Ok(())
    }

    /// Is the TX buffer full?
    pub fn tx_full(&self) -> bool {
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            let Some(uart) = guard.as_mut() else {
                return true;
            };
            uart.regs().read_status().txf()
        })
    }

    /// Write to the TX buffer.
    ///
    /// Returns Err(byte) if it's full.
    ///
    /// Returns `Ok(())` if not initialised
    pub fn write(&self, byte: u8) -> Result<(), u8> {
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            let Some(uart) = guard.as_mut() else {
                panic!("TX on uninitialised UART!");
            };
            match uart.write(byte) {
                Err(nb::Error::WouldBlock) => Err(byte),
                _ => Ok(()),
            }
        })
    }

    /// Turn TX interrupt on
    pub fn set_tx_interrupt(&self, enabled: bool) {
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            let Some(uart) = guard.as_mut() else {
                panic!("tx_interrupts_on on uninit MutexUart");
            };
            uart.regs().modify_control(|c| c.with_txie(enabled));
        })
    }

    /// Print some debug info to defmt
    pub fn dump_info(&self) {
        critical_section::with(|cs| {
            let mut guard = self.inner.borrow_ref_mut(cs);
            let Some(uart) = guard.as_mut() else {
                defmt::debug!("UART is not initialised");
                return;
            };
            defmt::debug!(
                "Control {}, Status {}, IntStatus {}",
                uart.regs().read_control(),
                uart.regs().read_status(),
                uart.regs().read_int_status()
            );
        });
    }
}

impl core::fmt::Write for MutexUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // convert from &mut MutexUart to &MutexUart
        let mut uart = &*self;
        // call the impl on &MutexUart
        <&MutexUart as core::fmt::Write>::write_str(&mut uart, s)
    }
}

impl core::fmt::Write for &MutexUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            'try_loop: loop {
                let result = critical_section::with(|cs| {
                    let mut guard = self.inner.borrow_ref_mut(cs);
                    let Some(uart) = guard.as_mut() else {
                        // if the UART isn't initialised, give up quietly
                        return Ok(());
                    };
                    uart.write(b)
                });
                match result {
                    Ok(()) => {
                        break 'try_loop;
                    }
                    Err(nb::Error::WouldBlock) => {
                        // try again
                    }
                    Err(nb::Error::Other(_)) => {
                        return Err(core::fmt::Error);
                    }
                }
            }
        }
        Ok(())
    }
}

unsafe impl Sync for MutexUart {}

// End of file
