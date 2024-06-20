#!/bin/sh

TARGET_DIR=target/production
BINARY=${TARGET_DIR}/qemu-demo
qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel ${BINARY}
