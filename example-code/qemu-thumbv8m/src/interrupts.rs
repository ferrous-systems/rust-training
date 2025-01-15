//! Interrupt code for the MPS2-AN505

/// Our interrupt handlers must have this type
///
/// The CPU architecture expects it.
type IrqFunction = unsafe extern "C" fn();

/// Our default interrupt handler
#[no_mangle]
unsafe extern "C" fn DefaultInterrupt() {
    defmt::error!("Unexpected interrupt in bagging area.");
    core::arch::asm!("udf 0");
}

/// A list of all our interrupts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Interrupts {
    /// Non-Secure Watchdog Request
    NsWatchdogRequest = 0,
    /// Non_Secure Watchdog Interrupt
    NsWatchdogIrq = 1,
    /// S32K Timer
    S32kTimer = 2,
    /// Timer 0
    Timer0 = 3,
    /// Timer 1
    Timer1 = 4,
    /// Dual Timer
    DualTimer = 5,
    /// MPC Combined (Secure)
    MpcCombined = 9,
    /// PPC Combined (Secure)
    PpcCombined = 10,
    /// MSC Combined (Secure)
    MscCombined = 11,
    /// Bridge Error Combined (Secure)
    BridgeErrorCombined = 12,
    /// UART 0 Receive
    Uart0Rx = 32,
    /// UART 0 Transmit
    Uart0Tx = 33,
    /// UART 1 Receive
    Uart1Rx = 34,
    /// UART 1 Transmit
    Uart1Tx = 35,
    /// UART 2 Receive
    Uart2Rx = 36,
    /// UART 2 Transmit
    Uart2Tx = 37,
    /// UART 3 Receive
    Uart3Rx = 38,
    /// UART 3 Transmit
    Uart3Tx = 39,
    /// UART 4 Receive
    Uart4Rx = 40,
    /// UART 4 Transmit
    Uart4Tx = 41,
    /// UART 0 Combined
    Uart0Combined = 42,
    /// UART 1 Combined
    Uart1Combined = 43,
    /// UART 2 Combined
    Uart2Combined = 44,
    /// UART 3 Combined
    Uart3Combined = 45,
    /// UART 4 Combined
    Uart4Combined = 46,
    /// UART 0-4 Overflow
    UartOverflow = 47,
    /// Ethernet
    Ethernet = 48,
    /// Audio I2S
    AudioI2S = 49,
    /// Touch Screen
    TouchScreen = 50,
    /// SPI0
    Spi0 = 51,
    /// SPI1
    Spi1 = 52,
    /// SPI2
    Spi2 = 53,
    /// SPI3
    Spi3 = 54,
    /// SPI4
    Spi4 = 55,
    /// DMA0 Error
    Dma0Error = 56,
    /// DMA0 Terminal Count
    Dma0TerminalCount = 57,
    /// DMA0 Combined
    Dma0Combined = 58,
    /// DMA1 Error
    Dma1Error = 59,
    /// DMA1 Terminal Count
    Dma1TerminalCount = 60,
    /// DMA1 Combined
    Dma1Combined = 61,
    /// DMA2 Error
    Dma2Error = 62,
    /// DMA2 Terminal Count
    Dma2TerminalCount = 63,
    /// DMA2 Combined
    Dma2Combined = 64,
    /// DMA3 Error
    Dma3Error = 65,
    /// DMA3 Terminal Count
    Dma3TerminalCount = 66,
    /// DMA3 Combined
    Dma3Combined = 67,
    /// GPIO 0 Combined
    Gpio0Combined = 68,
    /// GPIO 1 Combined
    Gpio1Combined = 69,
    /// GPIO 2 Combined
    Gpio2Combined = 70,
    /// GPIO 3 Combined
    Gpio3Combined = 71,
    /// GPIO 0.0
    Gpio0_0 = 72,
    /// GPIO 0.1
    Gpio0_1 = 73,
    /// GPIO 0.2
    Gpio0_2 = 74,
    /// GPIO 0.3
    Gpio0_3 = 75,
    /// GPIO 0.4
    Gpio0_4 = 76,
    /// GPIO 0.5
    Gpio0_5 = 77,
    /// GPIO 0.6
    Gpio0_6 = 78,
    /// GPIO 0.7
    Gpio0_7 = 79,
    /// GPIO 0.8
    Gpio0_8 = 80,
    /// GPIO 0.9
    Gpio0_9 = 81,
    /// GPIO 0.10
    Gpio0_10 = 82,
    /// GPIO 0.11
    Gpio0_11 = 83,
    /// GPIO 0.12
    Gpio0_12 = 84,
    /// GPIO 0.13
    Gpio0_13 = 85,
    /// GPIO 0.14
    Gpio0_14 = 86,
    /// GPIO 0.15
    Gpio0_15 = 87,
    /// GPIO 1.0
    Gpio1_0 = 88,
    /// GPIO 1.1
    Gpio1_1 = 89,
    /// GPIO 1.2
    Gpio1_2 = 90,
    /// GPIO 1.3
    Gpio1_3 = 91,
    /// GPIO 1.4
    Gpio1_4 = 92,
    /// GPIO 1.5
    Gpio1_5 = 93,
    /// GPIO 1.6
    Gpio1_6 = 94,
    /// GPIO 1.7
    Gpio1_7 = 95,
    /// GPIO 1.8
    Gpio1_8 = 96,
    /// GPIO 1.9
    Gpio1_9 = 97,
    /// GPIO 1.10
    Gpio1_10 = 98,
    /// GPIO 1.11
    Gpio1_11 = 99,
    /// GPIO 1.12
    Gpio1_12 = 100,
    /// GPIO 1.13
    Gpio1_13 = 101,
    /// GPIO 1.14
    Gpio1_14 = 102,
    /// GPIO 1.15
    Gpio1_15 = 103,
    /// GPIO 2.0
    Gpio2_0 = 104,
    /// GPIO 2.1
    Gpio2_1 = 105,
    /// GPIO 2.2
    Gpio2_2 = 106,
    /// GPIO 2.3
    Gpio2_3 = 107,
    /// GPIO 2.4
    Gpio2_4 = 108,
    /// GPIO 2.5
    Gpio2_5 = 109,
    /// GPIO 2.6
    Gpio2_6 = 110,
    /// GPIO 2.7
    Gpio2_7 = 111,
    /// GPIO 2.8
    Gpio2_8 = 112,
    /// GPIO 2.9
    Gpio2_9 = 113,
    /// GPIO 2.10
    Gpio2_10 = 114,
    /// GPIO 2.11
    Gpio2_11 = 115,
    /// GPIO 2.12
    Gpio2_12 = 116,
    /// GPIO 2.13
    Gpio2_13 = 117,
    /// GPIO 2.14
    Gpio2_14 = 118,
    /// GPIO 2.15
    Gpio2_15 = 119,
    /// GPIO3.0
    Gpio3_0 = 120,
    /// GPIO3.1
    Gpio3_1 = 121,
    /// GPIO3.2
    Gpio3_2 = 122,
    /// GPIO3.3
    Gpio3_3 = 123,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupts {
    fn number(self) -> u16 {
        self as u16
    }
}

extern "C" {
    fn NsWatchdogRequest();
    fn NsWatchdogIrq();
    fn S32kTimer();
    fn Timer0();
    fn Timer1();
    fn DualTimer();
    fn MpcCombined();
    fn PpcCombined();
    fn MscCombined();
    fn BridgeErrorCombined();
    fn Uart0Rx();
    fn Uart0Tx();
    fn Uart1Rx();
    fn Uart1Tx();
    fn Uart2Rx();
    fn Uart2Tx();
    fn Uart3Rx();
    fn Uart3Tx();
    fn Uart4Rx();
    fn Uart4Tx();
    fn Uart0Combined();
    fn Uart1Combined();
    fn Uart2Combined();
    fn Uart3Combined();
    fn Uart4Combined();
    fn UartOverflow();
    fn Ethernet();
    fn AudioI2S();
    fn TouchScreen();
    fn Spi0();
    fn Spi1();
    fn Spi2();
    fn Spi3();
    fn Spi4();
    fn Dma0Error();
    fn Dma0TerminalCount();
    fn Dma0Combined();
    fn Dma1Error();
    fn Dma1TerminalCount();
    fn Dma1Combined();
    fn Dma2Error();
    fn Dma2TerminalCount();
    fn Dma2Combined();
    fn Dma3Error();
    fn Dma3TerminalCount();
    fn Dma3Combined();
    fn Gpio0Combined();
    fn Gpio1Combined();
    fn Gpio2Combined();
    fn Gpio3Combined();
    fn Gpio0_0();
    fn Gpio0_1();
    fn Gpio0_2();
    fn Gpio0_3();
    fn Gpio0_4();
    fn Gpio0_5();
    fn Gpio0_6();
    fn Gpio0_7();
    fn Gpio0_8();
    fn Gpio0_9();
    fn Gpio0_10();
    fn Gpio0_11();
    fn Gpio0_12();
    fn Gpio0_13();
    fn Gpio0_14();
    fn Gpio0_15();
    fn Gpio1_0();
    fn Gpio1_1();
    fn Gpio1_2();
    fn Gpio1_3();
    fn Gpio1_4();
    fn Gpio1_5();
    fn Gpio1_6();
    fn Gpio1_7();
    fn Gpio1_8();
    fn Gpio1_9();
    fn Gpio1_10();
    fn Gpio1_11();
    fn Gpio1_12();
    fn Gpio1_13();
    fn Gpio1_14();
    fn Gpio1_15();
    fn Gpio2_0();
    fn Gpio2_1();
    fn Gpio2_2();
    fn Gpio2_3();
    fn Gpio2_4();
    fn Gpio2_5();
    fn Gpio2_6();
    fn Gpio2_7();
    fn Gpio2_8();
    fn Gpio2_9();
    fn Gpio2_10();
    fn Gpio2_11();
    fn Gpio2_12();
    fn Gpio2_13();
    fn Gpio2_14();
    fn Gpio2_15();
    fn Gpio3_0();
    fn Gpio3_1();
    fn Gpio3_2();
    fn Gpio3_3();
}

/// Represents an interrupt vector
///
/// Use the reserved value 0 to represent unset vectors
///
/// We could use `Option<IrqFunction>` but we cannot rely on `None` always
/// having the value `0`.
pub union Vector {
    /// An IRQ handler function
    handler: IrqFunction,
    /// A placeholder (should be the value 0)
    reserved: usize,
}

impl Vector {
    /// Mark a reserved space
    pub const fn reserved() -> Self {
        Vector { reserved: 0 }
    }

    /// Point to the given function
    pub const fn function(f: IrqFunction) -> Self {
        Vector { handler: f }
    }
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 124] = [
    // Non-Secure Watchdog Request
    Vector::function(NsWatchdogRequest),
    // Non_Secure Watchdog Interrupt
    Vector::function(NsWatchdogIrq),
    // S32K Timer
    Vector::function(S32kTimer),
    // Timer 0
    Vector::function(Timer0),
    // Timer 1
    Vector::function(Timer1),
    // Dual Timer
    Vector::function(DualTimer),
    // IRQ 6..=8 are reserved
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    // MPC Combined (Secure)
    Vector::function(MpcCombined),
    // PPC Combined (Secure)
    Vector::function(PpcCombined),
    // MSC Combined (Secure)
    Vector::function(MscCombined),
    // Bridge Error Combined (Secure)
    Vector::function(BridgeErrorCombined),
    // IRQ 13..=31 are reserved
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    Vector::reserved(),
    // UART 0 Receive
    Vector::function(Uart0Rx),
    // UART 0 Transmit
    Vector::function(Uart0Tx),
    // UART 1 Receive
    Vector::function(Uart1Rx),
    // UART 1 Transmit
    Vector::function(Uart1Tx),
    // UART 2 Receive
    Vector::function(Uart2Rx),
    // UART 2 Transmit
    Vector::function(Uart2Tx),
    // UART 3 Receive
    Vector::function(Uart3Rx),
    // UART 3 Transmit
    Vector::function(Uart3Tx),
    // UART 4 Receive
    Vector::function(Uart4Rx),
    // UART 4 Transmit
    Vector::function(Uart4Tx),
    // UART 0 Combined
    Vector::function(Uart0Combined),
    // UART 1 Combined
    Vector::function(Uart1Combined),
    // UART 2 Combined
    Vector::function(Uart2Combined),
    // UART 3 Combined
    Vector::function(Uart3Combined),
    // UART 4 Combined
    Vector::function(Uart4Combined),
    // UART 0-4 Overflow
    Vector::function(UartOverflow),
    // Ethernet
    Vector::function(Ethernet),
    // Audio I2S
    Vector::function(AudioI2S),
    // Touch Screen
    Vector::function(TouchScreen),
    // SPI0
    Vector::function(Spi0),
    // SPI1
    Vector::function(Spi1),
    // SPI2
    Vector::function(Spi2),
    // SPI3
    Vector::function(Spi3),
    // SPI4
    Vector::function(Spi4),
    // DMA0 Error
    Vector::function(Dma0Error),
    // DMA0 Terminal Count
    Vector::function(Dma0TerminalCount),
    // DMA0 Combined
    Vector::function(Dma0Combined),
    // DMA1 Error
    Vector::function(Dma1Error),
    // DMA1 Terminal Count
    Vector::function(Dma1TerminalCount),
    // DMA1 Combined
    Vector::function(Dma1Combined),
    // DMA2 Error
    Vector::function(Dma2Error),
    // DMA2 Terminal Count
    Vector::function(Dma2TerminalCount),
    // DMA2 Combined
    Vector::function(Dma2Combined),
    // DMA3 Error
    Vector::function(Dma3Error),
    // DMA3 Terminal Count
    Vector::function(Dma3TerminalCount),
    // DMA3 Combined
    Vector::function(Dma3Combined),
    // GPIO 0 Combined
    Vector::function(Gpio0Combined),
    // GPIO 1 Combined
    Vector::function(Gpio1Combined),
    // GPIO 2 Combined
    Vector::function(Gpio2Combined),
    // GPIO 3 Combined
    Vector::function(Gpio3Combined),
    // GPIO 0.0
    Vector::function(Gpio0_0),
    // GPIO 0.1
    Vector::function(Gpio0_1),
    // GPIO 0.2
    Vector::function(Gpio0_2),
    // GPIO 0.3
    Vector::function(Gpio0_3),
    // GPIO 0.4
    Vector::function(Gpio0_4),
    // GPIO 0.5
    Vector::function(Gpio0_5),
    // GPIO 0.6
    Vector::function(Gpio0_6),
    // GPIO 0.7
    Vector::function(Gpio0_7),
    // GPIO 0.8
    Vector::function(Gpio0_8),
    // GPIO 0.9
    Vector::function(Gpio0_9),
    // GPIO 0.10
    Vector::function(Gpio0_10),
    // GPIO 0.11
    Vector::function(Gpio0_11),
    // GPIO 0.12
    Vector::function(Gpio0_12),
    // GPIO 0.13
    Vector::function(Gpio0_13),
    // GPIO 0.14
    Vector::function(Gpio0_14),
    // GPIO 0.15
    Vector::function(Gpio0_15),
    // GPIO 1.0
    Vector::function(Gpio1_0),
    // GPIO 1.1
    Vector::function(Gpio1_1),
    // GPIO 1.2
    Vector::function(Gpio1_2),
    // GPIO 1.3
    Vector::function(Gpio1_3),
    // GPIO 1.4
    Vector::function(Gpio1_4),
    // GPIO 1.5
    Vector::function(Gpio1_5),
    // GPIO 1.6
    Vector::function(Gpio1_6),
    // GPIO 1.7
    Vector::function(Gpio1_7),
    // GPIO 1.8
    Vector::function(Gpio1_8),
    // GPIO 1.9
    Vector::function(Gpio1_9),
    // GPIO 1.10
    Vector::function(Gpio1_10),
    // GPIO 1.11
    Vector::function(Gpio1_11),
    // GPIO 1.12
    Vector::function(Gpio1_12),
    // GPIO 1.13
    Vector::function(Gpio1_13),
    // GPIO 1.14
    Vector::function(Gpio1_14),
    // GPIO 1.15
    Vector::function(Gpio1_15),
    // GPIO 2.0
    Vector::function(Gpio2_0),
    // GPIO 2.1
    Vector::function(Gpio2_1),
    // GPIO 2.2
    Vector::function(Gpio2_2),
    // GPIO 2.3
    Vector::function(Gpio2_3),
    // GPIO 2.4
    Vector::function(Gpio2_4),
    // GPIO 2.5
    Vector::function(Gpio2_5),
    // GPIO 2.6
    Vector::function(Gpio2_6),
    // GPIO 2.7
    Vector::function(Gpio2_7),
    // GPIO 2.8
    Vector::function(Gpio2_8),
    // GPIO 2.9
    Vector::function(Gpio2_9),
    // GPIO 2.10
    Vector::function(Gpio2_10),
    // GPIO 2.11
    Vector::function(Gpio2_11),
    // GPIO 2.12
    Vector::function(Gpio2_12),
    // GPIO 2.13
    Vector::function(Gpio2_13),
    // GPIO 2.14
    Vector::function(Gpio2_14),
    // GPIO 2.15
    Vector::function(Gpio2_15),
    // GPIO3.0
    Vector::function(Gpio3_0),
    // GPIO3.1
    Vector::function(Gpio3_1),
    // GPIO3.2
    Vector::function(Gpio3_2),
    // GPIO3.3
    Vector::function(Gpio3_3),
];
