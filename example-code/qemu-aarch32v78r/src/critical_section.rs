//! Code that implements the `critical-section` traits on Cortex-R.

struct SingleCoreCriticalSection;
critical_section::set_impl!(SingleCoreCriticalSection);

/// Reads the CPU interrupt status bit from CPSR
///
/// Returns true if interrupts enabled.
#[inline]
pub fn interrupts_enabled() -> bool {
    const CPSR_I_BIT: u32 = 1 << 7;
    let r: u32;
    unsafe {
        core::arch::asm!("mrs {}, CPSR", out(reg) r, options(nomem, nostack, preserves_flags))
    };
    r & CPSR_I_BIT != 0
}

unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let was_active = interrupts_enabled();
        core::arch::asm!("cpsid i", options(nomem, nostack, preserves_flags));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        was_active
    }

    unsafe fn release(was_active: critical_section::RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            core::arch::asm!("cpsie i", options(nomem, nostack, preserves_flags));
        }
    }
}
