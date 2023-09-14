#!/usr/bin/env bash

set -euo pipefail

# Check the example code
pushd ./native
cargo build --all
popd
# And the C based example
pushd native/ffi/use-rust-in-c
make
popd
# And the nRF52 examples
pushd ./nrf52/bsp_demo
cargo build
popd
