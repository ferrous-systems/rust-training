# Justfile for the Ferrous Systems Rust Training
#
# Copyright (c) Ferrous Systems, 2025

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
  @just --choose

everything: build-book build-slides test-book build-rust format-check-rust

format-check: format-check-rust

format: format-rust

build: build-book build-slides build-rust ferrocene-rust

clean: clean-rust
	rm -rf ./training-slides/book
	rm -rf ./training-slides/slides

serve: build-book build-slides
	cd training-slides && python3 -m http.server

build-book:
	cd training-slides && RUST_LOG=info mdbook build

test-book:
	cd training-slides && RUST_LOG=info mdbook test

build-slides: build-book
	cd training-slides && RUST_LOG=debug mdslides --template ./template.html --output-dir ./slides --mdbook-path . --index-template ./index-template.html
	cp -r ./training-slides/book/images ./training-slides/slides

assemble version: build-book build-slides clean-rust
	echo "Making ./rust-training-{{ version }}..."
	rm -rf ./rust-training-{{ version }}
	mkdir -p ./rust-training-{{ version }}
	mv ./training-slides/slides ./rust-training-{{ version }}/training-slides-presentation
	mv ./training-slides/book ./rust-training-{{ version }}/training-slides-book
	cp -r ./example-code ./rust-training-{{ version }}
	echo "Compressing ./rust-training-{{ version }}.zip..."
	zip -r ./rust-training-{{ version }}.zip ./rust-training-{{ version }}

make-cheatsheet LANG:
	cargo xtask make-cheatsheet {{LANG}}

test-cheatsheet LANG:
	cargo xtask test-cheatsheet {{LANG}}

test-all-cheatsheets:
	cargo xtask test-all-cheatsheets

# This is a script because we want to check everything, rather than stop on first failure
format-check-rust:
	#!/bin/sh
	FAIL=0
	cargo fmt --check --manifest-path example-code/native/ffi/use-c-in-rust/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/native/ffi/use-rust-in-c/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/native/stdout/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/nrf52/bsp_demo/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/qemu-aarch32v8r/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/qemu-aarch64v8a/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path example-code/qemu-thumbv7em/Cargo.toml || FAIL=1
	cargo fmt --check --manifest-path xtask/Cargo.toml || FAIL=1
	if [[ "$FAIL" == 1 ]]; then exit 1; else echo "Formatting all OK"; fi

format-rust:
	cargo fmt --manifest-path example-code/native/ffi/use-c-in-rust/Cargo.toml
	cargo fmt --manifest-path example-code/native/ffi/use-rust-in-c/Cargo.toml
	cargo fmt --manifest-path example-code/native/stdout/Cargo.toml
	cargo fmt --manifest-path example-code/nrf52/bsp_demo/Cargo.toml
	cargo fmt --manifest-path example-code/qemu-aarch32v8r/Cargo.toml
	cargo fmt --manifest-path example-code/qemu-aarch64v8a/Cargo.toml
	cargo fmt --manifest-path example-code/qemu-thumbv7em/Cargo.toml
	cargo fmt --manifest-path xtask/Cargo.toml

build-rust: build-native-ffi-use-c-in-rust build-native-ffi-use-rust-in-c build-native-stdout build-nrf52-bsp-demo build-qemu-aarch32v8r build-qemu-aarch64v8a build-qemu-aarch64v8a-no-cargo build-qemu-thumbv7em build-xtask

build-native-ffi-use-c-in-rust:
	cd example-code/native/ffi/use-c-in-rust && cargo build --release

build-native-ffi-use-rust-in-c:
	cd example-code/native/ffi/use-rust-in-c && cargo build --release

build-native-stdout:
	cd example-code/native/stdout && RUSTC_BOOTSTRAP=1 cargo build --release

build-nrf52-bsp-demo:
	cd example-code/nrf52/bsp_demo && cargo build --release

build-qemu-aarch32v8r:
	cd example-code/qemu-aarch32v8r && cargo +nightly build --release

build-qemu-aarch64v8a:
	cd example-code/qemu-aarch64v8a && cargo build --release

build-qemu-aarch64v8a-no-cargo:
	cd example-code/qemu-aarch64v8a && RUSTC=$(rustup which rustc) ./build.sh

build-qemu-thumbv7em:
	cd example-code/qemu-thumbv7em && cargo build --release

build-xtask:
	cd xtask && cargo build

[windows]
[working-directory: "example-code/native/ffi/use-rust-in-c/windows-example"]
msbuild:
	msbuild.exe windows-example.sln /p:Configuration=Release
	.\x64\Release\windows-example.exe

ferrocene-rust: ferrocene-qemu-aarch64v8a ferrocene-qemu-aarch32v8r ferrocene-qemu-thumbv7em

ferrocene-qemu-aarch64v8a:
	#!/bin/sh
	cd example-code/qemu-aarch64v8a
	criticalup install
	criticalup run cargo build --release

ferrocene-qemu-aarch64v8a-no-cargo:
	#!/bin/sh
	cd example-code/qemu-aarch64v8a
	criticalup install
	RUSTC=$(criticalup which rustc) ./build.sh

ferrocene-qemu-aarch32v8r:
	#!/bin/sh
	cd example-code/qemu-aarch32v8r
	criticalup install
	criticalup run cargo build --release

ferrocene-qemu-thumbv7em:
	#!/bin/sh
	cd example-code/qemu-thumbv7em
	criticalup install
	criticalup run cargo build --release

clean-rust:
	cargo clean --manifest-path example-code/native/ffi/use-c-in-rust/Cargo.toml
	cargo clean --manifest-path example-code/native/ffi/use-rust-in-c/Cargo.toml
	cargo clean --manifest-path example-code/native/stdout/Cargo.toml
	cargo clean --manifest-path example-code/nrf52/bsp_demo/Cargo.toml
	cargo clean --manifest-path example-code/qemu-aarch32v8r/Cargo.toml
	cargo clean --manifest-path example-code/qemu-aarch64v8a/Cargo.toml
	cargo clean --manifest-path example-code/qemu-thumbv7em/Cargo.toml
	cargo clean --manifest-path example-code/qemu-common/Cargo.toml
