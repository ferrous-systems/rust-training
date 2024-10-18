#!/usr/bin/env bash

set -euo pipefail

# Build qemu Aarch64 Armv8-A example
pushd ./qemu-aarch64v8a
criticalup install
./build.sh "$(criticalup which rustc)"
criticalup run cargo build --release
criticalup run cargo clean
popd
# And the qemu Aarch32 Armv8-R/Armv7-R example
pushd ./qemu-aarch32v78r
criticalup install
./build.sh "$(criticalup which rustc)"
criticalup run cargo build --release
criticalup run cargo build --target=armv7r-none-eabihf --release
criticalup run cargo clean
popd
