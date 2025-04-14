#!/bin/sh

TARGET_DIR=target/armv8r-none-eabihf/debug
BINARY=${1:-${TARGET_DIR}/no_heap}
qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel ${BINARY}
