.section .text.startup
.global _start
.code 32
.align 0

_start:
    // Set stack pointer
    ldr r3, =stack_top
    mov sp, r3
    // Jump to application
    bl kmain
    // In case the application returns, loop forever
    b .
