#!/bin/bash

# This requires you to previously run `cargo install defmt-print`

ELF_BINARY=$1
echo "ELF_BINARY=$ELF_BINARY"
shift
qemu-system-arm -cpu cortex-m4 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel $ELF_BINARY | defmt-print -e $ELF_BINARY $*
