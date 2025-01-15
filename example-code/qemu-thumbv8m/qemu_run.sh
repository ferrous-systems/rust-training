#!/bin/bash

# This requires you to previously run `cargo install defmt-print`

# See https://ferroussystems.hackmd.io/@jonathanpallant/ryA1S6QDJx for a description of all the relevant QEMU machines

ELF_BINARY=$1
# All suitable for thumbv8m.main-none-eabihf
MACHINE="-cpu cortex-m33 -machine mps2-an505"
# MACHINE="-cpu cortex-m33 -machine mps2-an521"
# MACHINE="-cpu cortex-m33 -machine mps3-an524"
# MACHINE="-cpu cortex-m55 -machine mps3-an547"
echo "ELF_BINARY=$ELF_BINARY"
shift
echo "Running on '$MACHINE'..."
echo "------------------------------------------------------------------------"
qemu-system-arm $MACHINE -semihosting-config enable=on,target=native -kernel $ELF_BINARY -serial telnet:localhost:4321,server,wait | defmt-print -e $ELF_BINARY $*
echo "------------------------------------------------------------------------"
