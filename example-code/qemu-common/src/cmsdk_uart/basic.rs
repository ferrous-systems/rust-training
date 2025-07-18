//! Basic CMSDK UART driver

use super::{Control, Error, IntStatus, Status};

/// Represents the MMIO registers for a CMSDK UART Peripheral
///
/// We use the `derive_mmio::Mmio` macro to automatically generate the
/// [`MmioRegisters`] wrapper type, which has methods for reading/writing each
/// of these registers.
#[derive(derive_mmio::Mmio)]
#[repr(C)]
struct Registers {
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
    pub(crate) registers: MmioRegisters<'static>,
}

impl CmsdkUart {
    /// What we expect in the CID registers
    const VALID_CID: [u32; 4] = [0x0D, 0xF0, 0x05, 0xB1];

    /// What we expect in the PID0 and half of PID1
    const VALID_PID: u16 = 0x821;

    /// Create a new CMSDK UART driver.
    ///
    /// # Safety
    ///
    /// * Ensure only one driver exists for any UART at a time, or that you
    ///   never race on register accesses if multiple drivers exist.
    /// * Ensure the base address points to a valid CMSDK MMIO instance, with
    ///   at least 32-bit alignment.
    pub const unsafe fn new(base_addr: usize) -> CmsdkUart {
        CmsdkUart {
            registers: unsafe { Registers::new_mmio_at(base_addr) },
        }
    }

    /// Initialise the UART
    ///
    /// Most CMSDK UARTs power-up in the 'disabled' state, which will cause the
    /// TXFIFO to never empty.
    pub fn init(&mut self, baud_rate: u32, system_clock: u32) -> Result<(), Error> {
        defmt::debug!(
            "Init UART @ {=usize:08x}, baud_rate={=u32}, system_clock={=u32}",
            self.registers.pointer_to_data() as usize,
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
        self.registers
            .modify_control(|c| c.with_txe(true).with_rxe(true));
        Ok(())
    }

    /// Write a byte, if possible
    pub fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        let status = self.registers.read_status();
        if status.txf() {
            defmt::debug!(
                "Blocking on UART @ {=usize:08x} Status: {}",
                self.registers.pointer_to_data() as usize,
                status
            );
            return Err(nb::Error::WouldBlock);
        }
        self.registers.write_data(byte as u32);
        Ok(())
    }

    /// Write a byte, blocking until space available
    pub fn write_blocking(&mut self, byte: u8) {
        _ = nb::block!(self.write(byte));
    }

    /// Check that this is a valid CMSDK UART instance
    pub fn check(&mut self) -> Result<(), Error> {
        defmt::debug!(
            "Checking UART @ 0x{=usize:08x}",
            self.registers.pointer_to_data() as usize
        );
        let cid_read = [
            self.registers.read_cid(0).unwrap(),
            self.registers.read_cid(1).unwrap(),
            self.registers.read_cid(2).unwrap(),
            self.registers.read_cid(3).unwrap(),
        ];
        defmt::debug!("CIDS: {:?} vs {:?}", cid_read, Self::VALID_CID);
        if cid_read != Self::VALID_CID {
            return Err(Error::InvalidInstance);
        }
        let pid0 = self.registers.read_pid(0).unwrap() as u8;
        let pid1 = self.registers.read_pid(1).unwrap() as u8 & 0x0F;
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

    /// Clear interrupts
    pub fn clear_interrupts(&mut self, mask: IntStatus) {
        self.registers.write_int_status(mask);
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
