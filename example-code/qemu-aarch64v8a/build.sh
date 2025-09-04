#!/bin/bash

set -euo pipefail

TARGET_DIR=target/production
RUSTC="${RUSTC:-rustc}"
SYSROOT=$("${RUSTC}" --print sysroot)
OBJDUMP=$(ls "${SYSROOT}"/lib/rustlib/*/bin/llvm-objdump)
RUSTC_FLAGS="--target aarch64-unknown-none -Copt-level=s"

WITH_HEAP_OUTPUT_BINARY=${TARGET_DIR}/with_heap
WITH_HEAP_OUTPUT_MAP=${TARGET_DIR}/with_heap.map
WITH_HEAP_OUTPUT_ASM=${TARGET_DIR}/with_heap.asm
NO_HEAP_OUTPUT_BINARY=${TARGET_DIR}/no_heap
NO_HEAP_OUTPUT_MAP=${TARGET_DIR}/no_heap.map
NO_HEAP_OUTPUT_ASM=${TARGET_DIR}/no_heap.asm

rm -rf ${TARGET_DIR}
mkdir -p ${TARGET_DIR}

# ############################################################################
echo "Printing version"
# ############################################################################
"${RUSTC}" --version

# ############################################################################
echo "Running rustc for critical-section"
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	--crate-type=lib \
	--crate-name=critical_section \
	--emit=dep-info,metadata,link \
	--out-dir ${TARGET_DIR} \
	--cfg 'feature="restore-state-bool"' \
	--edition 2021 \
	vendor/critical-section/src/lib.rs

# ############################################################################
echo "Running rustc for linked-list-allocator"
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	--crate-type=lib \
	--crate-name=linked_list_allocator \
	--emit=dep-info,metadata,link \
	--out-dir ${TARGET_DIR} \
	-L ${TARGET_DIR} \
	--edition 2018 \
	vendor/linked_list_allocator/src/lib.rs

# ############################################################################
echo "Running rustc for embedded-alloc"
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	--crate-type=lib \
	--crate-name=embedded_alloc \
	--emit=dep-info,metadata,link \
	--out-dir ${TARGET_DIR} \
	-L ${TARGET_DIR} \
	--extern critical_section=${TARGET_DIR}/libcritical_section.rmeta \
	--extern linked_list_allocator=${TARGET_DIR}/liblinked_list_allocator.rmeta \
	--edition 2018 \
	vendor/embedded-alloc/src/lib.rs

# ############################################################################
echo Running rustc for lib...
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	--crate-type=lib \
	--crate-name=qemu_aarch64v8a \
	--emit=dep-info,metadata,link \
	--out-dir ${TARGET_DIR} \
	-L ${TARGET_DIR} \
	--edition 2021 \
	--extern critical_section=${TARGET_DIR}/libcritical_section.rmeta \
	--extern embedded_alloc=${TARGET_DIR}/libembedded_alloc.rmeta \
	src/lib.rs

# ############################################################################
echo Running rustc for no_heap...
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	--crate-type=bin \
	-Clink-arg=-Tlinker.ld \
	--edition 2021 \
	-L ${TARGET_DIR} \
	--extern qemu_aarch64v8a=${TARGET_DIR}/libqemu_aarch64v8a.rlib \
	-o ${NO_HEAP_OUTPUT_BINARY} \
	src/bin/no_heap.rs
echo Generating asm for no_heap...
"${OBJDUMP}" -Cd ${NO_HEAP_OUTPUT_BINARY} > ${NO_HEAP_OUTPUT_ASM}
echo Generating map for no_heap...
"${OBJDUMP}" -Ct ${NO_HEAP_OUTPUT_BINARY} > ${NO_HEAP_OUTPUT_MAP}

# ############################################################################
echo Running rustc for with_heap...
# ############################################################################
"${RUSTC}" ${RUSTC_FLAGS} \
	-Clink-arg=-Tlinker.ld \
	--edition 2021 \
	-L ${TARGET_DIR} \
	--extern qemu_aarch64v8a=${TARGET_DIR}/libqemu_aarch64v8a.rlib \
	--extern embedded_alloc=${TARGET_DIR}/libembedded_alloc.rlib \
	-o ${WITH_HEAP_OUTPUT_BINARY} \
	src/bin/with_heap.rs
echo Generating asm for with_heap...
"${OBJDUMP}" -Cd ${WITH_HEAP_OUTPUT_BINARY} > ${WITH_HEAP_OUTPUT_ASM}
echo Generating map for with_heap...
"${OBJDUMP}" -Ct ${WITH_HEAP_OUTPUT_BINARY} > ${WITH_HEAP_OUTPUT_MAP}

