//! Basic CMSDK UART driver

use super::{
    registers::{Control, IntStatus, Status},
    Error,
};

/// Represents the MMIO registers for a CMSDK UART Peripheral
///
/// We use the `derive_mmio::Mmio` macro to automatically generate the
/// [`MmioRegisters`] wrapper type, which has methods for reading/writing each
/// of these registers.
#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct Registers {
    #[mmio(Read, Write)]
    data: u32,
    #[mmio(PureRead)]
    status: Status,
    #[mmio(Read, Write, Modify)]
    control: Control,
    #[mmio(Read, Write)]
    int_status: IntStatus,
    _reserved: [u32; 1012],
    #[mmio(Read)]
    pid: [u32; 2],
    _reserved2: [u32; 2],
    #[mmio(Read)]
    cid: [u32; 4],
}

/// A CMSDK UART driver
pub struct CmsdkUart {
    tx: Tx,
    rx: Rx,
}

impl CmsdkUart {
    /// What we expect in the CID registers
    const VALID_CID: [u32; 4] = [0x0D, 0xF0, 0x05, 0xB1];

    /// What we expect in the PID0 and half of PID1
    const VALID_PID: u16 = 0x821;

    /// Create a new CMSDK UART driver from a register block.
    pub const fn new(regs: MmioRegisters<'static>) -> Self {
        Self {
            // Safety: TX only uses TX related registers.
            tx: Tx(unsafe { regs.clone() }),
            rx: Rx(regs),
        }
    }

    /// Create a new CMSDK UART driver.
    ///
    /// # Safety
    ///
    /// * Ensure only one driver exists for any UART at a time, or that you
    ///   never race on register accesses if multiple drivers exist.
    /// * Ensure the base address points to a valid CMSDK MMIO instance, with
    ///   at least 32-bit alignment.
    pub const unsafe fn new_with_raw_addr(base_addr: usize) -> Self {
        Self::new(unsafe { Registers::new_mmio_at(base_addr) })
    }

    /// Initialise the UART
    ///
    /// Most CMSDK UARTs power-up in the 'disabled' state, which will cause the
    /// TXFIFO to never empty.
    pub fn init(&mut self, baud_rate: u32, system_clock: u32) -> Result<(), Error> {
        defmt::debug!(
            "Init UART @ {=usize:08x}, baud_rate={=u32}, system_clock={=u32}",
            self.rx.0.pointer_to_data() as usize,
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
        self.regs()
            .modify_control(|c| c.with_txe(true).with_rxe(true));
        // show the settings
        defmt::debug!("{}", self.rx.0.read_control());
        Ok(())
    }

    /// Split the UART into TX and RX halves.
    pub fn split(self) -> (Tx, Rx) {
        (self.tx, self.rx)
    }

    /// Direct access to the underlying MMIO registers.
    #[inline]
    pub fn regs(&mut self) -> &mut MmioRegisters<'static> {
        &mut self.rx.0
    }

    /// Read a byte from the UART, non-blocking
    ///
    /// If the UART FIFO is empty, you get nb::Error::WouldBlock. But
    /// otherwise it cannot fail.
    pub fn read(&mut self) -> nb::Result<u8, core::convert::Infallible> {
        let status = self.regs().read_status();
        if !status.rxf() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.regs().read_data() as u8)
    }

    /// Write a byte, if possible
    pub fn write(&mut self, byte: u8) -> nb::Result<(), core::convert::Infallible> {
        let status = self.regs().read_status();
        if status.txf() {
            defmt::debug!(
                "Blocking on UART @ {=usize:08x}, {}",
                self.regs().pointer_to_data() as usize,
                status
            );
            return Err(nb::Error::WouldBlock);
        }
        self.regs().write_data(byte as u32);
        Ok(())
    }

    /// Write a byte, blocking until space available
    pub fn write_blocking(&mut self, byte: u8) {
        self.tx.write_blocking(byte);
    }

    /// Check that this is a valid CMSDK UART instance
    pub fn check(&mut self) -> Result<(), Error> {
        defmt::debug!(
            "Checking UART @ 0x{=usize:08x}",
            self.rx.0.pointer_to_data() as usize
        );
        let cid_read = [
            self.regs().read_cid(0).unwrap(),
            self.regs().read_cid(1).unwrap(),
            self.regs().read_cid(2).unwrap(),
            self.regs().read_cid(3).unwrap(),
        ];
        defmt::debug!("CIDS: {:?} vs {:?}", cid_read, Self::VALID_CID);
        if cid_read != Self::VALID_CID {
            return Err(Error::InvalidInstance);
        }
        let pid0 = self.regs().read_pid(0).unwrap() as u8;
        let pid1 = self.regs().read_pid(1).unwrap() as u8 & 0x0F;
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

    /// Get the current interrupt status for the UART
    #[inline]
    pub fn read_int_status(&mut self) -> IntStatus {
        self.regs().read_int_status()
    }

    /// Clear the given flags in the interrupt status register
    #[inline]
    pub fn clear_interrupts(&mut self, mask: IntStatus) {
        self.regs().write_int_status(mask);
    }

    /// ENable RX interrupts.
    #[inline]
    pub fn enable_rx_interrupts(&mut self) {
        self.rx.enable_interrupts();
    }
}

impl core::fmt::Write for CmsdkUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.tx.write_str(s)
    }
}

/// UART TX driver.
pub struct Tx(MmioRegisters<'static>);

impl Tx {
    /// Steal the new CMSDK UART TX driver, circumventing ownership checks.
    ///
    /// # Safety
    ///
    /// * Ensure only one driver exists for any UART TX at a time, or that you
    ///   never race on register accesses if multiple drivers exist.
    /// * Ensure the base address points to a valid CMSDK MMIO instance, with
    ///   at least 32-bit alignment.
    pub unsafe fn steal(base_addr: usize) -> Tx {
        let regs = unsafe { Registers::new_mmio_at(base_addr) };
        Tx(regs)
    }

    /// Raw register access.
    #[inline]
    pub fn regs(&mut self) -> &mut MmioRegisters<'static> {
        &mut self.0
    }

    /// Write a byte, blocking until space available
    pub fn write_blocking(&mut self, byte: u8) {
        _ = nb::block!(self.write(byte));
    }

    /// Disable the TX driver.
    #[inline]
    pub fn disable(&mut self) {
        self.0.modify_control(|c| c.with_txe(false));
    }

    /// Enable the TX driver.
    #[inline]
    pub fn enable(&mut self) {
        self.0.modify_control(|c| c.with_txe(true));
    }

    /// Enable TX interrupts.
    #[inline]
    pub fn enable_interrupts(&mut self) {
        self.0
            .modify_control(|c| c.with_txie(true).with_txoie(true));
    }

    /// Disable TX interrupts.
    #[inline]
    pub fn disable_interrupts(&mut self) {
        self.0
            .modify_control(|c| c.with_txie(false).with_txoie(false));
    }

    /// Clear TX interrupts
    #[inline]
    pub fn clear_interrupts(&mut self) {
        self.0.write_int_status(
            IntStatus::builder()
                .with_txi(true)
                .with_rxi(false)
                .with_txoi(true)
                .with_rxoi(false)
                .build(),
        );
    }

    /// Write a byte in a non-blocking manner using [nb].
    pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        let status = self.0.read_status();
        if status.txf() {
            defmt::debug!(
                "Blocking on UART @ {=usize:08x}, {}",
                self.0.pointer_to_data() as usize,
                status
            );
            return Err(nb::Error::WouldBlock);
        }
        self.0.write_data(byte as u32);
        Ok(())
    }
}

impl core::fmt::Write for Tx {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            self.write_blocking(*b);
        }
        Ok(())
    }
}

/// UART RX driver.
pub struct Rx(MmioRegisters<'static>);

impl Rx {
    /// Steal the new CMSDK UART RX driver, circumventing ownership checks.
    ///
    /// # Safety
    ///
    /// * Ensure only one driver exists for any UART RX at a time, or that you
    ///   never race on register accesses if multiple drivers exist.
    /// * Ensure the base address points to a valid CMSDK MMIO instance, with
    ///   at least 32-bit alignment.
    pub unsafe fn steal(base_addr: usize) -> Rx {
        let regs = unsafe { Registers::new_mmio_at(base_addr) };
        Rx(regs)
    }

    /// Enable RX interrupts.
    #[inline]
    pub fn enable_interrupts(&mut self) {
        self.0.modify_control(|mut c| {
            c.set_rxie(true);
            c.set_rxoie(true);
            c
        });
    }

    /// Disable RX interrupts.
    #[inline]
    pub fn disable_interrupts(&mut self) {
        self.0.modify_control(|mut c| {
            c.set_rxie(false);
            c.set_rxoie(false);
            c
        });
    }

    /// Read the UART in a non-blocking manner.
    pub fn read(&mut self) -> nb::Result<u8, Error> {
        let status = self.0.read_status();
        if !status.rxf() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.0.read_data() as u8)
    }

    /// Clear RX interrupts
    pub fn clear_interrupts(&mut self) {
        self.0.write_int_status(
            IntStatus::builder()
                .with_txi(false)
                .with_rxi(true)
                .with_txoi(false)
                .with_rxoi(true)
                .build(),
        );
    }
}
