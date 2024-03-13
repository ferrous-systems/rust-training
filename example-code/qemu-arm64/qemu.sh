#!/bin/sh

TARGET_DIR=target/production
BINARY=${TARGET_DIR}/basic-rust
qemu-system-aarch64 -machine virt -cpu cortex-a57 -semihosting -nographic -kernel ${BINARY}
