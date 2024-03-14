#!/bin/sh

TARGET_DIR=target/production
BINARY=${TARGET_DIR}/basic-rust
/Users/jonathan/Documents/open-source/qemu/build/qemu-system-arm -machine mps3-an536 -cpu cortex-r52 -semihosting -nographic -kernel ${BINARY}
