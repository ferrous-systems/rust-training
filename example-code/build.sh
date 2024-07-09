#!/usr/bin/env bash

set -euo pipefail

# Check the example code
pushd ./native/ffi/use-c-in-rust
cargo build --all
cargo test
cargo clean
popd
pushd ./native/stdout
cargo build --all
cargo clean
popd
# And the C based example
pushd native/ffi/use-rust-in-c
make
make clean
popd
# And the nRF52 examples
pushd ./nrf52/bsp_demo
cargo build
cargo clean
popd
# And the qemu Aarch64 Armv8-A example
pushd ./qemu-aarch64v8a
criticalup install
./build.sh
criticalup run cargo build
criticalup run cargo clean
popd
# And the qemu Aarch32 Armv8-R/Armv7-R example
pushd ./qemu-aarch32v78r
criticalup install
./build.sh
criticalup run cargo build
criticalup run cargo build --target=armv7r-none-eabihf
criticalup run cargo clean
popd
