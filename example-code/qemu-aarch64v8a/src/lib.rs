#![no_std]

pub mod critical_section;
pub mod virt_uart;

/// An Aarch64 Exception Level
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExceptionLevel {
    /// User code
    EL0,
    /// Kernel code
    EL1,
    /// Hypervisor code
    EL2,
    /// Secure Kernel code
    EL3,
}

/// Reads the CPU Exception Level from `CurrentEL`
#[inline]
pub fn exception_level() -> ExceptionLevel {
    let r: u32;
    unsafe {
        core::arch::asm!("mrs {0:x}, CurrentEL", out(reg) r, options(nomem, nostack, preserves_flags))
    };
    match (r >> 2) & 0b11 {
        0 => ExceptionLevel::EL0,
        1 => ExceptionLevel::EL1,
        2 => ExceptionLevel::EL2,
        _ => ExceptionLevel::EL3,
    }
}

core::arch::global_asm!(
    r#"

.section .text.startup
.global _start

// Assumes we are in EL1

_start:
    // Set stack pointer
    ldr x30, =stack_top
    mov sp, x30
    // Set FPEN bits [21:20] to 0b11 to prevent trapping.
    mov x0, #3 << 20
    msr CPACR_EL1, x0
    // Clear interrupt mask bit to enable interrupts
    msr DAIFclr, #0x7
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .
"#,
);
