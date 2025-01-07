//! Code that implements the `critical-section` traits on 64-bit Aarch64.

struct SingleCoreCriticalSection;
critical_section::set_impl!(SingleCoreCriticalSection);

/// Reads the CPU interrupt status bit from DAIF
///
/// Returns true if interrupts enabled.
#[inline]
pub fn interrupts_enabled() -> bool {
    const DAIF_I_BIT: u32 = 1 << 7;
    let r: u32;
    unsafe {
        core::arch::asm!("mrs {0:x}, DAIF", out(reg) r, options(nomem, nostack, preserves_flags))
    };
    r & DAIF_I_BIT != 0
}

unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let was_active = interrupts_enabled();
        // Disable interrupts by masking IRQs (leave FIQ enabled)
        core::arch::asm!("msr DAIFset, #7", options(nomem, nostack, preserves_flags));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        was_active
    }

    unsafe fn release(was_active: critical_section::RawRestoreState) {
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            // Enable interrupts by unmasking IRQs
            core::arch::asm!("msr DAIFclr, #7", options(nomem, nostack, preserves_flags));
        }
    }
}
