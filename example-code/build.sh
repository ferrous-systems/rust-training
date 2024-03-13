#!/usr/bin/env bash

set -euo pipefail

# Check the example code
pushd ./native/ffi/use-c-in-rust
cargo build --all
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
