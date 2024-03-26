# Installing Ferrocene

## What's in the box?

* `rustc` - a compiler (★)
* `lld` - the LLVM linker (★)
* `cargo`/`rustdoc`/`rustfmt`/`clippy` - our usual friends
* `llvm-tools` - objcopy, size, etc
* `rust-analyzer` - for IDE integration
* `rust-src` - libstd source code
* `rust-std-xxx` - precompiled standard libraries (☆)
* `ferrocene-self-test` - checks your installation
* `ferrocene-docs-xxx` - documentation

★: qualified tool ☆: certification in progress

## Installation

* Ferrocene runs on x86-64 Linux, only (for now)
* Ferrocene 23.06 shipped as tarballs
* Ferrocene 24.05+ now has `criticalup`
* Either way, get `rustc` in your `PATH`

## Read the Manual

* `/opt/ferrocene-23.06/share/doc/ferrocene/html/user-manual`
* <https://public-docs.ferrocene.dev/main/user-manual/>

## Targets

* Supported targets installed as per the rest of the toolchain
* [Currently]:
  * `x86_64-unknown-linux-gnu`
  * `aarch64-unknown-none`
  * `thumbv7em-none-eabi` + `-eabihf` (☆)
  * `armv8r-none-eabihf` (☆)
  * `wasm32-unknown-unknown` (☆)

☆: experimental

[Currently]: https://public-docs.ferrocene.dev/main/user-manual/targets/index.html

## Downloading a release

Check out the Customer Portal

<https://customers.ferrocene.dev>

## Using criticalup

* Our equivalent of `rustup`
* Fetches the appropriate Ferrocene toolchain packages
* No global Ferrocene toolchain - always specified per-project
* Need a `criticalup.toml` file and a login token

## criticalup.toml

```toml
manifest-version = 1

[products.ferrocene]
release = "beta-24.05-2024-03-21"
packages = [
  "rustc-${rustc-host}", "rust-std-${rustc-host}", "cargo-${rustc-host}",
  "rust-src", "rust-std-aarch64-unknown-none"
]
```

## Process

1. Make a [token](https://customers.ferrocene.dev/users/tokens)
2. Login with `criticalup auth set`
3. Go to your project dir
4. Run `criticalup install`
5. Add `~/.local/share/criticalup/bin` to your PATH
6. Or `rustup toolchain link ferrocene ~/.local/share/criticalup/bin`

## Example

```console
$ criticalup auth set
$ criticalup install
info: installing product 'ferrocene' (beta-24.05-2024-03-21)
info: downloading component 'cargo-x86_64-unknown-linux-gnu' for 'ferrocene' (beta-24.05-2024-03-21)
...
info: downloading component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (beta-24.05-2024-03-21)
info: installing component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (beta-24.05-2024-03-21)
# export PATH=$HOME/.local/share/criticalup/bin:$PATH
# rustc --version
rustc 1.76.0 (28b06b23f 2024-03-20) (Ferrocene by Ferrous Systems)
```
