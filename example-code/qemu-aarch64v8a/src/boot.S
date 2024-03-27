.section .text.startup
.global _start

// Assumes we are in EL1

_start:
    // Set stack pointer
    ldr x30, =stack_top
    mov sp, x30
    // Set FPEN bits [21:20] to 0b11 to prevent trapping.
    mov x0, #3 << 20
    msr cpacr_el1, x0   
    // Clear interrupt bit
    msr daifclr, #0x4   
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .
