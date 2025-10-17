#!/bin/bash

# This requires you to previously run `cargo install defmt-print`

# See https://ferroussystems.hackmd.io/@jonathanpallant/ryA1S6QDJx for a description of all the relevant QEMU machines
TELNET=false
ELF_BINARY=""

for arg in "$@"; do
    case $arg in
        --telnet)
            TELNET=true
            ;;
        *)
            ELF_BINARY=$arg
            ;;
    esac
done

# All suitable for thumbv7em-none-eabihf
MACHINE="-cpu cortex-m4 -machine mps2-an386"
# MACHINE="-cpu cortex-m7 -machine mps2-387"
# MACHINE="-cpu cortex-m7 -machine mps2-500"
LOG_FORMAT='{[{L}]%bold} {s} {({ff}:{l:1})%dimmed}'
echo "ELF_BINARY=$ELF_BINARY"
shift
if [ `basename $ELF_BINARY` == "defmt" ]; then
  SERIAL_PORT=""
else
  if [[ $TELNET == true ]]; then
    echo "Waiting for telnet connection on localhost:4321"
    SERIAL_PORT_0="-serial telnet:localhost:4321,server,wait"
  else
    rm target/uart*.log
    echo "Writing UART output to target/uart*.log"
    SERIAL_PORT_0="-serial file:target/uart0.log"
  fi
  SERIAL_PORT="$SERIAL_PORT_0 \
    -serial file:target/uart1.log \
    -serial file:target/uart2.log \
    -serial file:target/uart3.log \
    -serial file:target/uart4.log"
fi

echo "Running on '$MACHINE'..."
echo "------------------------------------------------------------------------"
qemu-system-arm $MACHINE -semihosting-config enable=on,target=native -nographic -kernel $ELF_BINARY $SERIAL_PORT | defmt-print -e $ELF_BINARY --log-format="$LOG_FORMAT"
echo "------------------------------------------------------------------------"
