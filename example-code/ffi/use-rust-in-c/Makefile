# Example Makefile to build a Rust library as part of a C program.
# Copyright (c) Ferrous Systems 2023

# Note: this project is in a workspace, two levels up.
TARGET_DIR = ../../target

all: example

rebuild: clean all

clean:
	rm -rf example $(TARGET_DIR)

# The `example` binary requires the Rust library and some C source 
example: example.c example.h $(TARGET_DIR)/debug/libffi_use_rust_in_c.a
	cc -o example example.c $(TARGET_DIR)/debug/libffi_use_rust_in_c.a

# The Rust library is built by calling cargo
$(TARGET_DIR)/debug/libffi_use_rust_in_c.a: src/lib.rs Cargo.toml
	cargo build

.PHONY: all clean rebuild
