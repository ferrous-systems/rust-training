#![no_std]

use aarch64_paging::{
    mair::{Mair, MairAttribute, NormalMemory},
    paging::Attributes,
};
use aarch64_rt::initial_pagetable;
use aarch64_rt::InitialPagetable;

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

/// Attributes to use for device memory in the initial identity map.
const DEVICE_ATTRIBUTES: Attributes = Attributes::VALID
    .union(Attributes::ATTRIBUTE_INDEX_0)
    .union(Attributes::ACCESSED)
    .union(Attributes::UXN);

/// Attributes to use for normal memory in the initial identity map.
const MEMORY_ATTRIBUTES: Attributes = Attributes::VALID
    .union(Attributes::ATTRIBUTE_INDEX_1)
    .union(Attributes::INNER_SHAREABLE)
    .union(Attributes::ACCESSED)
    .union(Attributes::NON_GLOBAL);

/// Indirect memory attributes to use.
///
/// These are used for `ATTRIBUTE_INDEX_0` and `ATTRIBUTE_INDEX_1` in `DEVICE_ATTRIBUTES` and
/// `MEMORY_ATTRIBUTES` respectively.
const MAIR: Mair = Mair::EMPTY
    .with_attribute(0, MairAttribute::DEVICE_NGNRE)
    .with_attribute(
        1,
        MairAttribute::normal(
            NormalMemory::WriteBackNonTransientReadWriteAllocate,
            NormalMemory::WriteBackNonTransientReadWriteAllocate,
        ),
    );

initial_pagetable!(
    {
        let mut idmap = [0; 512];
        // 1 GiB of device memory starting at 0
        idmap[0] = DEVICE_ATTRIBUTES.bits();
        // 1 GiB of normal memory starting at 1 GiB
        idmap[1] = MEMORY_ATTRIBUTES.bits() | 0x00_4000_0000;
        InitialPagetable(idmap)
    },
    MAIR.0
);

/// Called when the application raises an unrecoverable `panic!`.
///
/// Prints the panic to the console and then exits QEMU using a semihosting
/// breakpoint.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:?}", info);
    semihosting::process::exit(1);
}

#[unsafe(no_mangle)]
extern "C" fn sync_exception_current(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn irq_current(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn fiq_current(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn serr_current(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn sync_lower(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn irq_lower(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn fiq_lower(_elr: u64, _spsr: u64) {}

#[unsafe(no_mangle)]
extern "C" fn serr_lower(_elr: u64, _spsr: u64) {}
