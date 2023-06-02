#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Check the example code
cargo build --all --manifest-path ${SCRIPT_DIR}/Cargo.toml
# And the C based example
make -C ${SCRIPT_DIR}/ffi/use-rust-in-c
