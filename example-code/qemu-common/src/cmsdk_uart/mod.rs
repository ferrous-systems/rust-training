//! UART driver for the CMSDK UART
//!
//! Registers:
//!
//! * Data
//!   * 7-0: Data value
//! * State
//!   * 0: TXBF
//!   * 1: RXBF
//!   * 2: TXOV
//!   * 3: RXOV
//! * CTRL
//!   * 0: TXE
//!   * 1: RXE
//!   * 2: TXIE
//!   * 3: RXIE
//!   * 4: TXOE
//!   * 5: RXOE
//! * INTSTATUS:
//!   * 0: TXI
//!   * 1: RXI
//!   * 2: TXOI
//!   * 3: RXOI
//! * BAUDDIV
//!   * 19-0: Divider (minimum value is 16)
//!
//! This UART only has a one byte buffer.

mod basic;
pub use basic::*;

mod mutex;
pub use mutex::*;

mod buffered;
pub use buffered::*;

mod registers;

/// Error codes from this module
#[derive(Debug, defmt::Format)]
pub enum Error {
    /// Invalid instance.
    InvalidInstance,
    /// Invalid baudrate.
    InvalidBaudRate,
}

// End of file
