#!/usr/bin/env bash

set -euo pipefail

# Build qemu Aarch64 Armv8-A example
pushd ./qemu-aarch64v8a
criticalup install
./build.sh "$(criticalup which rustc)"
criticalup run cargo build --release --locked
popd
# And the qemu Aarch32 Armv8-R example
pushd ./qemu-aarch32v8r
criticalup install
criticalup run cargo build --release --locked
popd
# Build qemu Armv7E-M example
pushd qemu-thumbv7em
criticalup install
criticalup run cargo build --locked
popd
