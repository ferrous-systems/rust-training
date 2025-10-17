//! A driver for the MPS2-AN386 timers

pub use qemu_common::cmsdk_timer::*;

pub const TIMER_0_ADDR: usize = 0x4000_0000;
pub const TIMER_1_ADDR: usize = 0x4000_1000;
