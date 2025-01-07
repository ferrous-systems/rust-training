//! A driver the Arm CMSDK Uart
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

/// A driver for CMSDK Uart
pub struct Uart<const ADDR: usize>();

impl Uart<0xe7c0_0000> {
    /// Create a new UART object for UART0
    ///
    /// # Safety
    ///
    /// Only construct one object per UART at any given time.
    pub unsafe fn new_uart0() -> Self {
        Uart()
    }
}

impl<const ADDR: usize> Uart<ADDR> {
    const STATUS_TX_FULL: u32 = 1 << 0;
    const CONTROL_TX_EN: u32 = 1 << 0;

    const DATA_OFFSET: usize = 0x000 >> 2;
    const STATUS_OFFSET: usize = 0x004 >> 2;
    const CONTROL_OFFSET: usize = 0x008 >> 2;
    const BAUD_OFFSET: usize = 0x010 >> 2;

    /// Turn on TX and RX
    pub fn enable(&mut self, baudrate: u32, system_clock: u32) {
        let divider = system_clock / baudrate;
        self.set_bauddiv(divider);
        self.set_control(Self::CONTROL_TX_EN);
    }

    /// Write a byte (blocking if there's no space)
    pub fn write(&mut self, byte: u8) {
        // Check the Buffer Full bit
        while (self.get_status() & Self::STATUS_TX_FULL) != 0 {}
        self.set_data(byte as u32);
    }

    /// Write the data register
    fn set_data(&mut self, data: u32) {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::DATA_OFFSET);
            ptr.write_volatile(data)
        }
    }

    /// Read the status register
    fn get_status(&self) -> u32 {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::STATUS_OFFSET);
            ptr.read_volatile()
        }
    }

    /// Set the control register
    fn set_control(&mut self, data: u32) {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::CONTROL_OFFSET);
            ptr.write_volatile(data)
        }
    }

    /// Set the baud rate divider register
    fn set_bauddiv(&mut self, data: u32) {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::BAUD_OFFSET);
            ptr.write_volatile(data)
        }
    }
}

impl<const N: usize> core::fmt::Write for Uart<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}

// End of file
