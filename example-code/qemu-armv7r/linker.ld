/* RAM starts at 0x00000000 */

MEMORY {
    RAM : ORIGIN = 0x00000000, LENGTH = 128M
}

ENTRY(_start)
SECTIONS {
    .startup ORIGIN(RAM) : {
        *(.text.startup)
    } > RAM
    .text : { *(.text .text*) } > RAM
    .rodata : { *(.rodata .rodata*) } > RAM
    .data : { *(.data .data*) } > RAM
    .bss : { *(.bss .bss* COMMON) } > RAM
    /DISCARD/ : {
        *(.note .note*)
    }
    . = ALIGN(16);
    . += 0x100000; /* 1024kB of stack memory */
    stack_top = .;
}
