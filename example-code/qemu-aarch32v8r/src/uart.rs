//! A driver the Arm CMSDK Uart

pub use qemu_common::cmsdk_uart::*;

/// Address of UART0 on an MPS3-AN536
pub const UART0_ADDR: usize = 0xe7c0_0000;

// End of file
