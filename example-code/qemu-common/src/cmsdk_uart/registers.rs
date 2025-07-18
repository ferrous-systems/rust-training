//! Register definitions for the CSMSDK UART

/// UART Status
#[bitbybit::bitfield(u32)]
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

impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(
            f,
            "Status {{ txf={0=0..1}, rxf={0=1..2}, txo={0=2..3}, rxo={0=3..4} }}",
            self.raw_value()
        )
    }
}

/// UART Control
#[bitbybit::bitfield(u32)]
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

impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(
            f,
            "Control {{ txe={0=0..1} rxe={0=1..2} txie={0=2..3} rxie={0=3..4} txoie={0=4..5} rxoie={0=5..6} }}",
            self.raw_value()
        )
    }
}

/// UART Interrupt Status
#[bitbybit::bitfield(u32, default = 0)]
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

impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(
            f,
            "IntStatus {{ txi={0=0..1} rxi={0=1..2} txoi={0=2..3} rxoi={0=3..4} }}",
            self.raw_value()
        )
    }
}

// End of file
