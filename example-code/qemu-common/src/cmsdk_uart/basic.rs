//! Basic CMSDK driver

use super::{Control, Error, IntStatus, Status};

/// A CMSDK UART driver
pub struct CmsdkUart {
    base_addr: *mut u32,
}

impl CmsdkUart {
    /// Byte offset to the DATA register
    const OFFSET_DATA: usize = 0;

    /// Byte offset to the STATUS register
    const OFFSET_STATUS: usize = 4;

    /// Byte offset to the CONTROL register
    const OFFSET_CONTROL: usize = 8;

    /// Byte offset to the INTSTATUS/INTCLEAR register
    const OFFSET_INT_STATUS: usize = 12;

    /// Byte offset to the PID0 register
    const OFFSET_PID0: usize = 0xFE0;

    /// Byte offset to the PID1 register
    const OFFSET_PID1: usize = 0xFE4;

    /// Byte offset to the CID0-3 registers
    const OFFSET_CID0: usize = 0xFF0;

    /// What we expect in the CID registers
    const VALID_CID: [u32; 4] = [0x0D, 0xF0, 0x05, 0xB1];

    /// What we expect in the PID0 and half of PID1
    const VALID_PID: u16 = 0x821;

    /// Create a new CMSDK UART driver.
    ///
    /// # Safety
    ///
    /// * Ensure only one driver exists for any UART at a time.
    /// * Ensure the base address points to a valid CMSDK MMIO instance, with
    ///   at least 32-bit alignment.
    pub const unsafe fn new(base_addr: usize) -> CmsdkUart {
        CmsdkUart {
            base_addr: base_addr as *mut u32,
        }
    }

    /// Initialise the UART
    ///
    /// Most CMSDK UARTs power-up in the 'disabled' state, which will cause the
    /// TXFIFO to never empty.
    pub fn init(&mut self, baud_rate: u32, system_clock: u32) -> Result<(), Error> {
        defmt::debug!(
            "Init UART @ {=usize:08x}, baud_rate={=u32}, system_clock={=u32}",
            self.base_addr as usize,
            baud_rate,
            system_clock
        );
        // calculate divisor
        let divisor = system_clock / baud_rate;
        // check it is >= 16
        if divisor < 16 {
            return Err(Error::InvalidBaudRate);
        }
        // enable TX and RX
        self.modify_control(|c| {
            c.set(Control::TXE, true);
            c.set(Control::RXE, true);
        });
        Ok(())
    }

    /// Write a byte, if possible
    pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        let status = self.read_status();
        if status.contains(Status::TXF) {
            defmt::debug!(
                "Blocking on UART @ {=usize:08x} Status: {}",
                self.base_addr as usize,
                status
            );
            return Err(nb::Error::WouldBlock);
        }
        let data_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_DATA) };
        unsafe {
            data_ptr.write_volatile(byte as u32);
        }
        Ok(())
    }

    /// Write a byte, blocking until space available
    pub fn write_blocking(&mut self, byte: u8) {
        _ = nb::block!(self.write(byte));
    }

    /// Check that this is a valid CMSDK UART instance
    pub fn check(&self) -> Result<(), Error> {
        defmt::debug!("Checking UART @ 0x{=usize:08x}", self.base_addr as usize);
        let cid_base = unsafe { self.base_addr.byte_add(Self::OFFSET_CID0) };
        for (idx, value) in Self::VALID_CID.iter().enumerate() {
            let read_value = unsafe { cid_base.add(idx).read_volatile() };
            if read_value != *value {
                return Err(Error::InvalidInstance);
            }
        }
        let pid0 = unsafe { self.base_addr.byte_add(Self::OFFSET_PID0).read_volatile() } as u8;
        let pid1 =
            (unsafe { self.base_addr.byte_add(Self::OFFSET_PID1).read_volatile() } & 0x0F) as u8;
        let pid = u16::from_be_bytes([pid1, pid0]);
        defmt::trace!(
            "PID0 {=u8:02x} PID1 {=u8:02X} PID {=u16:04x}",
            pid0,
            pid1,
            pid
        );
        if pid != Self::VALID_PID {
            return Err(Error::InvalidInstance);
        }
        Ok(())
    }

    /// Read the status register
    pub fn read_status(&self) -> Status {
        let status_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_STATUS) };
        Status::from_bits(unsafe { status_ptr.read_volatile() }).unwrap()
    }

    /// Read the control register
    pub fn read_control(&self) -> Control {
        let control_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_CONTROL) };
        Control::from_bits(unsafe { control_ptr.read_volatile() }).unwrap()
    }

    /// Modify the control register
    pub fn modify_control<F>(&self, f: F)
    where
        F: FnOnce(&mut Control),
    {
        let control_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_CONTROL) };
        let mut control = Control::from_bits(unsafe { control_ptr.read_volatile() }).unwrap();
        f(&mut control);
        unsafe {
            control_ptr.write_volatile(control.bits());
        }
    }

    /// Read the Interrupt Status register
    pub fn read_intstatus(&self) -> IntStatus {
        let intstatus_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_INT_STATUS) };
        IntStatus::from_bits(unsafe { intstatus_ptr.read_volatile() }).unwrap()
    }

    /// Clear interrupts
    pub fn clear_interrupts(&self, mask: IntStatus) {
        let intstatus_ptr = unsafe { self.base_addr.byte_add(Self::OFFSET_INT_STATUS) };
        unsafe {
            // write 1 to clear
            intstatus_ptr.write_volatile(mask.bits());
        }
    }
}

impl core::fmt::Write for CmsdkUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_blocking(*b);
        }
        Ok(())
    }
}
