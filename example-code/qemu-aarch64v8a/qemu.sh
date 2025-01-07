#!/bin/sh

TARGET_DIR=target/production
BINARY=${1:-${TARGET_DIR}/no_heap}
qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel ${BINARY}
