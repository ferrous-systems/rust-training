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
    /// UART 0 Receive
    Uart0Rx = 0,
    /// UART 0 Transmit
    Uart0Tx = 1,
    /// UART 1 Receive
    Uart1Rx = 2,
    /// UART 1 Transmit
    Uart1Tx = 3,
    /// UART 2 Receive
    Uart2Rx = 4,
    /// UART 2 Transmit
    Uart2Tx = 5,
    /// GPIO 0 Combined
    Gpio0Combined = 6,
    /// GPIO 1 Combined
    Gpio1Combined = 7,
    /// Timer 0
    Timer0 = 8,
    /// Timer 1
    Timer1 = 9,
    /// Dual Timer
    DualTimer = 10,
    /// SPI0 and SPI1
    Spi01 = 11,
    /// UART0, UART1 and UART2 Overflow
    Uart012Overflow = 12,
    /// Ethernet
    Ethernet = 13,
    /// Audio I²S
    AudioI2S = 14,
    /// Touch Screen
    TouchScreen = 15,
    /// GPIO 2 Combined
    Gpio2Combined = 16,
    /// GPIO 3 Combined
    Gpio3Combined = 17,
    /// UART 3 Receive
    Uart3Rx = 18,
    /// UART 3 Transmit
    Uart3Tx = 19,
    /// UART 4 Receive
    Uart4Rx = 20,
    /// UART 4 Transmit
    Uart4Tx = 21,
    /// SPI2
    Spi2 = 22,
    /// SPI3 and SPI4
    Spi34 = 23,
    /// Gpio 0, Pin 0
    Gpio0_0 = 24,
    /// Gpio 0, Pin 1
    Gpio0_1 = 25,
    /// Gpio 0, Pin 2
    Gpio0_2 = 26,
    /// Gpio 0, Pin 3
    Gpio0_3 = 27,
    /// Gpio 0, Pin 4
    Gpio0_4 = 28,
    /// Gpio 0, Pin 5
    Gpio0_5 = 29,
    /// Gpio 0, Pin 6
    Gpio0_6 = 30,
    /// Gpio 0, Pin 7
    Gpio0_7 = 31,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupts {
    fn number(self) -> u16 {
        self as u16
    }
}

extern "C" {
    fn Uart0Rx();
    fn Uart0Tx();
    fn Uart1Rx();
    fn Uart1Tx();
    fn Uart2Rx();
    fn Uart2Tx();
    fn Gpio0Combined();
    fn Gpio1Combined();
    fn Timer0();
    fn Timer1();
    fn DualTimer();
    fn Spi01();
    fn Uart012Overflow();
    fn Ethernet();
    fn AudioI2S();
    fn TouchScreen();
    fn Gpio2Combined();
    fn Gpio3Combined();
    fn Uart3Rx();
    fn Uart3Tx();
    fn Uart4Rx();
    fn Uart4Tx();
    fn Spi2();
    fn Spi34();
    fn Gpio0_0();
    fn Gpio0_1();
    fn Gpio0_2();
    fn Gpio0_3();
    fn Gpio0_4();
    fn Gpio0_5();
    fn Gpio0_6();
    fn Gpio0_7();
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

/// Our Cortex-M Interrupt Vector Table
///
/// The exceptions and other items in the first 16 entries are handled by the
/// `cortex-m-rt` crate. These are the remaining vectors for the peripheral
/// interrupts.
///
/// Normally you would use `svd2rust` to generate this, but we've done it by
/// hand to show that it isn't magical.
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 32] = [
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
    // GPIO 0 Combined
    Vector::function(Gpio0Combined),
    // GPIO 1 Combined
    Vector::function(Gpio1Combined),
    // Timer 0
    Vector::function(Timer0),
    // Timer 1
    Vector::function(Timer1),
    // Dual Timer
    Vector::function(DualTimer),
    // SPI0 and SPI1
    Vector::function(Spi01),
    // UART0, UART1 and UART2 Overflow
    Vector::function(Uart012Overflow),
    // Ethernet
    Vector::function(Ethernet),
    // Audio I²S
    Vector::function(AudioI2S),
    // Touch Screen
    Vector::function(TouchScreen),
    // GPIO 2 Combined
    Vector::function(Gpio2Combined),
    // GPIO 3 Combined
    Vector::function(Gpio3Combined),
    // UART 3 Receive
    Vector::function(Uart3Rx),
    // UART 3 Transmit
    Vector::function(Uart3Tx),
    // UART 4 Receive
    Vector::function(Uart4Rx),
    // UART 4 Transmit
    Vector::function(Uart4Tx),
    // SPI2
    Vector::function(Spi2),
    // SPI3 and SPI4
    Vector::function(Spi34),
    // Gpio 0, Pin 0
    Vector::function(Gpio0_0),
    // Gpio 0, Pin 1
    Vector::function(Gpio0_1),
    // Gpio 0, Pin 2
    Vector::function(Gpio0_2),
    // Gpio 0, Pin 3
    Vector::function(Gpio0_3),
    // Gpio 0, Pin 4
    Vector::function(Gpio0_4),
    // Gpio 0, Pin 5
    Vector::function(Gpio0_5),
    // Gpio 0, Pin 6
    Vector::function(Gpio0_6),
    // Gpio 0, Pin 7
    Vector::function(Gpio0_7),
];
