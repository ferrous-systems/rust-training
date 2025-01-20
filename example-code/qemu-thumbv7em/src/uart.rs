//! A driver for the MPS2-AN386 UARTs

pub use qemu_common::cmsdk_uart::*;

/// UART 0 on the MPS2-AN385 and compatibles
pub const UART0_ADDR: usize = 0x4000_4000;

/// UART 1 on the MPS2-AN385 and compatibles
pub const UART1_ADDR: usize = 0x4000_5000;

/// UART 2 on the MPS2-AN385 and compatibles
pub const UART2_ADDR: usize = 0x4000_6000;

/// UART 3 on the MPS2-AN385 and compatibles
pub const UART3_ADDR: usize = 0x4000_7000;

/// UART 4 on the MPS2-AN385 and compatibles
pub const UART4_ADDR: usize = 0x4000_9000;
