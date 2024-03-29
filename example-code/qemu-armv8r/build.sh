#!/bin/bash

set -euo pipefail

TARGET_DIR=target/production
OUTPUT_BINARY=${TARGET_DIR}/basic-rust
OUTPUT_MAP=${TARGET_DIR}/basic-rust.map
OUTPUT_ASM=${TARGET_DIR}/basic-rust.asm
OUR_PREFIX="${PREFIX:=arm-none-eabi}"
AS=${OUR_PREFIX}-as
AR=${OUR_PREFIX}-ar
OBJDUMP=${OUR_PREFIX}-objdump
RUSTC="rustc\
	--target armv8r-none-eabihf \
	-L ${TARGET_DIR}"

rm -rf ${TARGET_DIR}
mkdir -p ${TARGET_DIR}
echo Running as...
${AS} src/boot.S -march=armv8-r -mfpu=fp-armv8 -o ${TARGET_DIR}/boot.o
echo Running ar..
${AR} rcs ${TARGET_DIR}/libboot.a ${TARGET_DIR}/boot.o
echo Running rustc...
${RUSTC} src/main.rs -Clink-arg=-Tlinker.ld -lboot -Copt-level=s --edition 2021 -o ${OUTPUT_BINARY}
echo Generating asm...
${OBJDUMP} -d ${OUTPUT_BINARY} > ${OUTPUT_ASM}
echo Generating map...
${OBJDUMP} -t ${OUTPUT_BINARY} > ${OUTPUT_MAP}
