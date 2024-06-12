#!/bin/bash

set -euo pipefail

TARGET_DIR=target/production
OUTPUT_BINARY=${TARGET_DIR}/basic-rust
RUSTC=$(criticalup which rustc)
SYSROOT=$(criticalup run rustc --print sysroot)
OBJDUMP=$(ls "${SYSROOT}"/lib/rustlib/*/bin/llvm-objdump)
OUTPUT_MAP=${TARGET_DIR}/basic-rust.map
OUTPUT_ASM=${TARGET_DIR}/basic-rust.asm

rm -rf ${TARGET_DIR}
mkdir -p ${TARGET_DIR}
echo Running rustc...
"${RUSTC}" --target armv8r-none-eabihf \
	-Ctarget-cpu=cortex-r52 \
	-Clink-arg=-Tlinker.ld \
	-Copt-level=s \
	--edition 2021 \
	-o ${OUTPUT_BINARY} \
	src/main.rs
echo Generating asm...
"${OBJDUMP}" -Cd ${OUTPUT_BINARY} > ${OUTPUT_ASM}
echo Generating map...
"${OBJDUMP}" -Ct ${OUTPUT_BINARY} > ${OUTPUT_MAP}
