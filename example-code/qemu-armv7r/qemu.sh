#!/bin/sh

TARGET_DIR=target/production
BINARY=${1:-${TARGET_DIR}/no_heap}
qemu-system-arm -machine versatileab -cpu cortex-r5f -semihosting -nographic -kernel ${BINARY}
