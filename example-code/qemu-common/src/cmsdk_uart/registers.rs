//! Register definitions for the CSMSDK UART

defmt::bitflags! {
    /// UART Status
    pub struct Status: u32 {
        /// TX Full
        const TXF = 1;
        /// RX Full
        const RXF = 2;
        /// TX Overflow
        const TXO = 4;
        /// RX Overflow
        const RXO = 8;
    }
}

defmt::bitflags! {
    /// UART Control
    pub struct Control: u32 {
        /// TX Enabled
        const TXE   = 1;
        /// RX Enabled
        const RXE   = 2;
        /// TX Interrupt Enabled
        const TXIE  = 4;
        /// RX Interrupt Enabled
        const RXIE  = 8;
        /// TX Overflow Interrupt Enabled
        const TXOIE = 16;
        /// RX Overflow Interrupt Enabled
        const RXOIE = 32;
    }
}

defmt::bitflags! {
    /// UART Interrupt Status
    pub struct IntStatus: u32 {
        /// TX Interrupt
        const TXI   = 1;
        /// RX Interrupt
        const RXI   = 2;
        /// TX Overflow Interrupt
        const TXOI  = 4;
        /// RX Overflow Interrupt
        const RXOI  = 8;
    }
}

// End of file
