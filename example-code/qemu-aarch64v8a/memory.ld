/*
RAM starts at 0x40000000.

See https://github.com/qemu/qemu/blob/master/hw/arm/virt.c
*/

MEMORY {
    image : ORIGIN = 0x40000000, LENGTH = 2M
}
