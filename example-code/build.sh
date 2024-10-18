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
cargo build --release
cargo clean
popd
# Build qemu Aarch64 Armv8-A example
pushd ./qemu-aarch64v8a
./build.sh
cargo build
cargo clean
popd
# Build qemu Aarch32 Armv8-R/Armv7-R example
pushd ./qemu-aarch32v78r
# Can't use the shell script or the default target becuase armv8r isn't available
# outside Ferrocene
# ./build.sh
cargo build --target=armv7r-none-eabihf
cargo clean
popd
