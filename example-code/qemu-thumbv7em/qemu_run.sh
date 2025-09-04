#!/bin/bash

# This requires you to previously run `cargo install defmt-print`

# See https://ferroussystems.hackmd.io/@jonathanpallant/ryA1S6QDJx for a description of all the relevant QEMU machines

ELF_BINARY=$1
# All suitable for thumbv7em-none-eabihf
MACHINE="-cpu cortex-m4 -machine mps2-an386"
# MACHINE="-cpu cortex-m7 -machine mps2-387"
# MACHINE="-cpu cortex-m7 -machine mps2-500"
LOG_FORMAT='{[{L}]%bold} {s} {({ff}:{l:1})%dimmed}'
echo "ELF_BINARY=$ELF_BINARY"
shift
if [ `basename $ELF_BINARY` != "defmt" ]; then
   SERIAL_PORT="-serial telnet:localhost:4321,server,wait"
else
	SERIAL_PORT=""
fi
echo "Running on '$MACHINE'..."
echo "------------------------------------------------------------------------"
qemu-system-arm $MACHINE -semihosting-config enable=on,target=native -kernel $ELF_BINARY $SERIAL_PORT | defmt-print -e $ELF_BINARY $* --log-format="$LOG_FORMAT"
echo "------------------------------------------------------------------------"
