//! Register definitions for the CSMSDK UART

/// UART Status
#[bitbybit::bitfield(u32, defmt_bitfields)]
pub struct Status {
    /// TX Full
    #[bit(0, rw)]
    txf: bool,
    /// RX Full
    #[bit(1, rw)]
    rxf: bool,
    /// TX Overflow
    #[bit(2, rw)]
    txo: bool,
    /// RX Overflow
    #[bit(3, rw)]
    rxo: bool,
}

/// UART Control
#[bitbybit::bitfield(u32, defmt_bitfields)]
pub struct Control {
    /// TX Enabled
    #[bit(0, rw)]
    txe: bool,
    /// RX Enabled
    #[bit(1, rw)]
    rxe: bool,
    /// TX Interrupt Enabled
    #[bit(2, rw)]
    txie: bool,
    /// RX Interrupt Enabled
    #[bit(3, rw)]
    rxie: bool,
    /// TX Overflow Interrupt Enabled
    #[bit(4, rw)]
    txoie: bool,
    /// RX Overflow Interrupt Enabled
    #[bit(5, rw)]
    rxoie: bool,
}

/// UART Interrupt Status
#[bitbybit::bitfield(u32, default = 0, defmt_bitfields)]
pub struct IntStatus {
    /// TX Interrupt
    #[bit(0, rw)]
    txi: bool,
    /// RX Interrupt
    #[bit(1, rw)]
    rxi: bool,
    /// TX Overflow Interrupt
    #[bit(2, rw)]
    txoi: bool,
    /// RX Overflow Interrupt
    #[bit(3, rw)]
    rxoi: bool,
}

// End of file
