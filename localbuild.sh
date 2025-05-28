#!/bin/bash

#
# Ferrous Systems Rust Training Build Script
#
# Copyright (c) Ferrous Systems, 2025
#
# Builds/tests all the example code

set -euo pipefail

# Keep these functions in alphabetical order

# Clear out all the build temporaries
function clean() {
    pushd example-code/native/ffi/use-c-in-rust
    cargo clean
    popd
    pushd example-code/native/ffi/use-rust-in-c
    cargo clean
    popd
    pushd example-code/native/stdout
    cargo clean
    popd
    pushd example-code/nrf52/bsp_demo
    cargo clean
    popd
    pushd example-code/qemu-aarch32v8r
    cargo clean
    popd
    pushd example-code/qemu-aarch64v8a
    cargo clean
    rm -rf ./target/production
    popd
    pushd example-code/qemu-thumbv7em
    cargo clean
    popd
    pushd training-slides
    rm -rf ./slides
    rm -rf ./book
    popd
    cargo clean
}

# Build and Format Check example-code/native/ffi/use-c-in-rust
function eg-native-ffi-use-c-in-rust() {
    echo "Running eg-native-ffi-use-c-in-rust..."
    pushd example-code/native/ffi/use-c-in-rust
    cargo run --locked
    cargo test
    cargo fmt --check
    popd
}

# Build and Format Check example-code/native/ffi/use-rust-in-c
function eg-native-ffi-use-rust-in-c() {
    echo "Running eg-native-ffi-use-rust-in-c..."
    pushd example-code/native/ffi/use-rust-in-c
    make run
    cargo test
    cargo fmt --check
    popd
}

# Build and Format Check example-code/native/stdout
function eg-native-stdout() {
    echo "Running eg-native-stdout..."
    pushd example-code/native/stdout
    RUSTC_BOOTSTRAP=1 cargo build --locked
    cargo fmt --check
    popd
}

# Build and Format Check example-code/nrf52/bsp_demo
function eg-nrf52-bsp-demo() {
    echo "Running eg-nrf52-bsp-demo..."
    pushd example-code/nrf52/bsp_demo
    cargo build --release --locked
    cargo fmt --check
    popd
}

# Build and Format Check example-code/qemu-aarch32v8r
function ferrocene-qemu-aarch32v8r() {
    echo "Running ferrocene-qemu-aarch32v8r..."
    pushd example-code/qemu-aarch32v8r
    criticalup install
    criticalup run cargo build --release --locked
    popd
}

# Build and Format Check example-code/qemu-aarch64v8a
function ferrocene-qemu-aarch64v8a() {
    echo "Running ferrocene-qemu-aarch64v8a..."
    pushd example-code/qemu-aarch64v8a
    criticalup install
    criticalup run cargo build --release --locked
    # Also do the raw rustc build
    ./build.sh "$(criticalup which rustc)"
    popd
}

# Build and Format Check example-code/qemu-thumbv7em
function ferrocene-qemu-thumbv7em() {
    echo "Running ferrocene-qemu-thumbv7em..."
    pushd example-code/qemu-thumbv7em
    criticalup install
    criticalup run cargo build --release --locked
    popd
}

# Build and Format Check example-code/qemu-aarch32v8r
function eg-qemu-aarch32v8r() {
    echo "Running eg-qemu-aarch32v8r..."
    pushd example-code/qemu-aarch32v8r
    RUSTC_BOOTSTRAP=1 cargo build --release --locked -Zbuild-std=core,alloc
    cargo fmt --check
    popd
}

# Build and Format Check example-code/qemu-aarch64v8a
function eg-qemu-aarch64v8a() {
    echo "Running eg-qemu-aarch64v8a..."
    pushd example-code/qemu-aarch64v8a
    cargo build --release --locked
    cargo fmt --check
    popd
}

# Build and Format Check example-code/qemu-thumbv7em 
function eg-qemu-thumbv7em() {
    echo "Running eg-qemu-thumbv7em..."
    pushd example-code/qemu-thumbv7em
    cargo build --release --locked
    cargo fmt --check
    popd
}

# Renders the training material as an mbook and as a slide deck.
#
# Also runs the mdbook tests.
function render-material() {
    echo "Running render-material..."
    pushd training-slides
    RUST_LOG=info mdbook build
    RUST_LOG=debug mdslides --template ./template.html --output-dir ./slides --mdbook-path . --index-template ./index-template.html
    cp -r ./book/images ./slides
    popd
}

# Tests all the cheatsheets
#
# This makes sure their headings are in the same order as the slide decks.
function test-cheatsheets() {
    echo "Running test-cheatsheets..."
    cargo xtask test-all-cheatsheets --locked
}

# Runs the mdbook tests.
function test-material() {
    echo "Running test-material..."
    pushd training-slides
    RUST_LOG=info mdbook test
    popd
}

# Build and Format Check the xtask
function xtask() {
    echo "Running xtask..."
    pushd xtask
    cargo build
    cargo fmt --check
    popd
}

if [ $# -ge 1 ]; then
    # Run the specified steps
    for cmd in "$@"; do
        if [ "$cmd" == "--help" ]; then
            # Tell them about the steps
            echo "Pass one or more of the following (or no argument to run them all):"
            echo ""
            echo "clean"
            echo "eg-native-ffi-use-c-in-rust"
            echo "eg-native-ffi-use-rust-in-c"
            echo "eg-native-stdout"
            echo "eg-nrf52-bsp-demo"
            echo "eg-qemu-aarch32v8r"
            echo "eg-qemu-aarch64v8a"
            echo "eg-qemu-thumbv7em"
            echo "ferrocene-qemu-aarch32v8r"
            echo "ferrocene-qemu-aarch64v8a"
            echo "ferrocene-qemu-thumbv7em"
            echo "render-material"
            echo "test-cheatsheets"
            echo "test-material"
            echo "xtask"
        else
            $cmd
        fi
    done
else
    # Run all the steps (except 'clean')
    eg-native-ffi-use-c-in-rust
    eg-native-ffi-use-rust-in-c
    eg-native-stdout
    eg-nrf52-bsp-demo
    eg-qemu-aarch32v8r
    eg-qemu-aarch64v8a
    eg-qemu-thumbv7em
    ferrocene-qemu-aarch32v8r
    ferrocene-qemu-aarch64v8a
    ferrocene-qemu-thumbv7em
    render-material
    test-cheatsheets
    test-material
    xtask
fi
