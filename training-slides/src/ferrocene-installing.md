# Installing and Using Ferrocene

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

---

![Portal](./images/portal.png)

<https://customers.ferrocene.dev>

## Toolchain Versions

*channels* contain *releases*

## Channels

* nightly
* pre-rolling
* rolling
* beta-24.05
* beta-24.08
* stable-24.05
* stable-24.08
* etc

## Releases

* nightly-2024-08-29
* pre-rolling-2024-08-28
* rolling-2024-08-08
* beta-24.05-2024-06-19
* beta-24.08-2024-08-22
* stable-24.05.0
* stable-24.08.0
* etc

## Targets

We have two dimensions:

* Qualified, or not
* Host or Cross-compiled

## Qualified Targets

* Production Ready
* Passes the Rust Test Suite
* Support is available
* Signed qualification material
  * stable channel only

Note:

In stable-24.08 and earlier, these were called "Supported Targets"

Each release has a User Manual and it is important to follow the instructions
for that target in that release otherwise you may be outside the qualification
scope. As an example, we don't let you give arbitrary arguments to the linker -
you can only pass the arguments we say are OK.

## Quality Managed (QM) Targets

* Production Ready
* Passes the Rust Test Suite
* Support is available
* ~~Signed qualification material~~

Note:

It may be that the target is en-route to being a Qualified Target, or it may be
that it is deemed unlikely that the target would be useful in a safety critical
context. Talk to us if you would like a QM Target available as a Qualified
Target.

## Experimental Targets

* Not Production Ready
* Not qualified
* Might not pass the test suite
* But useful for getting started early

Note:

A Ferrocene 'Experimental Target' is broadly equivalent to an upstream Tier 2 or
Tier 1 target, depending on whether we're running the Test Suite in CI. And, to
be fair, plenty of people use upstream Rust in production.

## Host Targets

* Ferrocene runs on a limited number of hosts:
  * See [the public docs](https://public-docs.ferrocene.dev/main/user-manual/targets/index.html)
  * Or [the customer portal](https://customers.ferrocene.dev)
* Ferrocene is installed with [`criticalup`](https://criticalup.ferrocene.dev)
  * It's also [open-source](https://github.com/ferrocene/criticalup)
  * Or, you can install a specific Ferrocene release from tarballs
* Hosts always compile for themselves (proc-macros, `build.rs`, etc)

## Cross-Compilation Targets

* Compiling for a machine that is not the current host
* The list of targets from release to release
* See [the public docs](https://public-docs.ferrocene.dev/main/user-manual/targets/index.html)
* Or [the customer portal](https://customers.ferrocene.dev)

## Using criticalup

* Our equivalent of `rustup`
* Fetches the appropriate Ferrocene toolchain packages
  * Packages are signed with [`criticaltrust`](https://docs.rs/criticaltrust)
* Need a `criticalup.toml` file for each project, and a global login token
  * Token only required to *download* a toolchain
  * You can burn the toolchain to a CD-R if you want

## criticalup.toml

```toml
manifest-version = 1

[products.ferrocene]
release = "stable-24.08.0"
packages = [
  "rustc-${rustc-host}", "rust-std-${rustc-host}", "cargo-${rustc-host}",
  "rust-src", "rust-std-aarch64-unknown-none"
]
```

## Installing Ferrocene

1. Make a [token](https://customers.ferrocene.dev/users/tokens)
2. Store your token with `criticalup auth set`
3. Go to your project dir
4. Run `criticalup install`

## Example

```console
$ criticalup auth set
$ criticalup install
info: installing product 'ferrocene' (stable-24.08.0)
info: downloading component 'cargo-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
...
info: downloading component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
info: installing component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
$ criticalup run rustc --version
```

## Where was it installed?

Linux:

```text
/home/jonathan/.local/share/criticalup/toolchains/<hash>/bin/rustc
```

macOS:

```text
/Users/jonathan/Library/Application Support/criticalup/toolchains/<hash>/bin/rustc
```

Windows:

```text
C:\Users\jonathan\AppData\Local\criticalup\toolchains\<hash>\bin\rustc.exe
```

## Running Ferrocene

* You can execute `rustc` or `cargo` directly
* You can use the `rustc` proxy
* You can point `rustup` at a Ferrocene install
* You can point `rustup` at the `criticalup` proxy

## Example

```console
$ criticalup auth set
$ criticalup install
info: installing product 'ferrocene' (stable-24.08.0)
info: downloading component 'cargo-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
...
info: downloading component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
info: installing component 'rustc-x86_64-unknown-linux-gnu' for 'ferrocene' (stable-24.08.0)
$ criticalup run rustc --version
rustc 1.79.0 (02baf75fd 2024-08-23) (Ferrocene by Ferrous Systems)
$ criticalup run cargo build --release
...
```
