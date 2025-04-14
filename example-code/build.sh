#!/usr/bin/env bash

set -euo pipefail

# Check the example code
pushd ./native/ffi/use-c-in-rust
cargo build --all --locked
cargo test --locked
popd
pushd ./native/stdout
cargo build --all --locked
popd
# And the C based example
pushd native/ffi/use-rust-in-c
make
make clean
popd
# And the nRF52 examples
pushd ./nrf52/bsp_demo
cargo build --release --locked
popd
# Build qemu Aarch64 Armv8-A example
pushd ./qemu-aarch64v8a
./build.sh
cargo build --locked
popd
# Build qemu Aarch32 Armv8-Rexample
pushd ./qemu-aarch32v8r
RUSTC_BOOTSTRAP=1 cargo build --target=armv8r-none-eabihf --locked -Zbuild-std=core,alloc
popd
# Build qemu Armv7E-M example
pushd qemu-thumbv7em
cargo build --locked
popd

