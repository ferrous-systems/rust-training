//! A driver the Arm PL011 Uart
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2024

/// A driver for a virtual PL011 Uart
///
/// It skips almost all the important initialisation, but it works on QEMU.
pub struct Uart<const ADDR: usize>();

impl Uart<0x0900_0000> {
    /// Create a new UART object for UART0
    ///
    /// # Safety
    ///
    /// Only construct one object per UART at any given time.
    pub unsafe fn new_uart0() -> Self {
        let mut u = Uart();
        u.set_control(Self::CONTROL_UARTEN | Self::CONTROL_TXE);
        u
    }
}

impl<const ADDR: usize> Uart<ADDR> {
    const FLAG_TXFF: u32 = 1 << 5;
    const CONTROL_UARTEN: u32 = 1 << 0;
    const CONTROL_TXE: u32 = 1 << 8;

    const DATA_OFFSET: usize = 0x000 >> 2;
    const FLAG_OFFSET: usize = 0x018 >> 2;
    const CONTROL_OFFSET: usize = 0x030 >> 2;

    /// Write a byte (blocking if there's no space)
    pub fn write(&mut self, byte: u8) {
        // Check the TX FIFO Full bit
        while (self.get_flags() & Self::FLAG_TXFF) != 0 {}
        self.write_data(byte);
    }

    /// Write to the data register
    fn write_data(&mut self, value: u8) {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::DATA_OFFSET);
            ptr.write_volatile(value as u32);
        }
    }

    /// Read from the Flag Register
    fn get_flags(&mut self) -> u32 {
        unsafe {
            let ptr = (ADDR as *const u32).add(Self::FLAG_OFFSET);
            ptr.read_volatile()
        }
    }

    /// Write to the control register
    fn set_control(&mut self, value: u32) {
        unsafe {
            let ptr = (ADDR as *mut u32).add(Self::CONTROL_OFFSET);
            ptr.write_volatile(value);
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
