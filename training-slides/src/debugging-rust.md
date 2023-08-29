# Debugging Rust

## tl;dr

Best option: *VSCode + CodeLLDB*

The best debugging experience on Windows, Linux, and macOS

## Honorable Mentions

* IntelliJ Rust
* `rr` / Pernosco for time-traveling and postmortem debugging

## How Debuggers Work

Debuggers use special metadata embedded into executable to correctly match bits of machine code to lines of source code, areas of memory to variables and their types, etc.

Kinda like Source Maps for JavaScript.

## How Debuggers Work 2

Two things have to happen for a debugger to work and provide decent developer experience:

* The compiler has to emit debug info.
* The debugger has to be modified / extended to *understand* this information.

## How Debuggers Work 3

Two things have to happen for a debugger to work and provide decent developer experience:

* *The compiler has to emit debug info.*
* The debugger has to be modified / extended to *understand* this information.

## Compiler

`rustc` uses `llvm` which emits debug info in DWARF or PDB format.

* [PDB](http://blog.llvm.org/2017/08/llvm-on-windows-now-supports-pdb-debug.html) is produced by `windows-msvc` toolchains (like `x86_64-pc-windows-msvc`)
* [DWARF](https://dwarfstd.org) is used by all other toolchains, including GNU toolchains on Windows (like `x86_64-pc-windows-gnu`)

## DWARF

* Open standard.
* Very C/C++ specific.
* Has custom field types for other languages to use.
* Rust tries to reuse existing C/C++ fields where possible, so many debuggers work out of the box.
* A companion to *ELF*...

## Extending DWARF

DWARF standard is growing organically over time and largely implementation driven.

## Extending DWARF 2

1. Come up with a new name for Rust-specific DWARF field.
2. Change the compiler to emit new debug info and use this field.
3. Change a debugger to understand this new field.
4. Propose the new field to be standardized, so that other debuggers can reuse the field, too.

Standardizing takes almost no time due to how few people in the world actually work on DWARF.

## PDB

* Proprietary format with no documentation.
* Like DWARF is very C/C++ centric.
* Harder to extend.
* Rust tries to reuse C/C++ fields as much as possible, so debugging is still reasonable.

---

You may have a better experience debugging Rust on macOS or Linux than on Windows, because of PDB.

## How Debuggers Work 4

Two things have to happen for a debugger to work and provide decent developer experience:

* The compiler has to emit debug info.
* *The debugger has to be modified / extended to understand this information.*

## Debuggers

* [GDB](https://sourceware.org/gdb/onlinedocs/gdb/Rust.html)
* [LLDB](https://lldb.llvm.org)

IDEs and editors rely on these two to provide GUI debugging

## GDB

* Supports a lot of languages.
* Adopts Rust-specific features quickly.
* Harder to contribute in general.

## LLDB

* *Default choice for Rust.*
* Part of LLVM that Rust uses for compilation.
* Used to support many languages, but the team decided to focus on C, C++, and Objective C only.
* Has extension API for supporting other languages, which is not enough for Rust.

## LLDB 2

Rust project maintains a fork of LLDB with extended support for the language.

* Part of overall LLVM fork.
* Constantly updated and well-maintained.
* Non-Rust-specific bug fixes get upstreamed to main LLVM repository

## Wrappers

Rust comes with `rust-gdb` and `rust-lldb` wrappers around debuggers.

They improve visualizing Rust values printed in console.

## Editors and IDEs

*Rust-analyzer* does not come with debugger support on its own.

Instead it relies on other editor / ide plugins for debugging support.

Prompts you to install one when you open a Rust project.

## VSCode Extensions

* [Microsoft C/C++ extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools).
* [Native Debug](https://marketplace.visualstudio.com/items?itemName=webfreak.debug).
* [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).

## CodeLLDB

* LLDB-only.
* *Maintains it's own fork of Rust's LLDB with even more Rust enhancements!*
* Downloads it on first installation.
* Seamless debugging experience.

---

Both Microsoft C/C++ and Native Debug support GDB *and* LLDB.

Microsoft's extension offers better support for displaying PDB information on Windows.

## IntelliJ-Rust

* A plugin for IDEA and CLion
* Produced by JetBrain.
* Like CodeLLDB also maintains it's own fork of Rust's LLDB for better DX.
* Requires a JB license.

## What to use?

* *VSCode + CodeLLDB offer the best debugging experience across all platforms.*
* [Microsoft recommends CodeLLDB even for Windows use.](https://docs.microsoft.com/en-us/windows/dev-environment/rust/setup)
* IntelliJ-Rust is great if that's your IDE of choice.
* Native Debug and Microsoft C/C++ extensions can work for you on platforms where only GDB is available.

## `rr`

* [A Linux-only terminal-based time-traveling debugger.](https://rr-project.org)
* Uses GDB under the hood, supports Rust.
* [Pernosco](https://pernos.co) - GUI debugging tool on top of `rr` on top of `gdb` - offers Rust support, too.
* May help you in very difficult situations.

## Things may not work well

* PDB may result in subpar debugging experience.
  * If possible try debugging your code on OSes other than Windows
  * Or try using GNU-based toolchain on Windows.
* Watch expressions are limited.
  * Can't use `match` or `if` expressions
  * Some method calls may not produce results.
* Some values can't be shown: function preferences, closures.
* Breakpoints may sometimes not work in closures and in async code.
* Trait objects and trait methods may be difficult for debugger to resolve.

## When debugger fails you

* Try to isolate the code in question into smaller functions.
* Add debug logging / tracing.
* Tests.

## Future

* *New [Rust Debugging](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html#debugging-aspirations-) Working Group:*
  * Unites people from Rust, GDB, and `rr`
  * people from LLVM, CodeLLDB, Rust-Analyzer, and IntelliJ Rust expressed interest in helping out.🎉
* Plans:
  * LLVM team is open to merge Rust-specific features into LLDB directly, may not need a Rust fork, or CodeLLDB / IntelliJ forks.
  * Further expand DWARF to cover tricky Rust features like trait object method references.
