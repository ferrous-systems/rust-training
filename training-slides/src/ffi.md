# Foreign Function Interface (FFI)

## What is it?

* For interfacing *Rust code* with *foreign functions*
* For interfacing *foreign code* with *Rust functions*

## Application Binary Interface (ABI)

(Like an API, but for machine code calling machine code)

---

The Rust ABI is *not* stable.

---

Rust also supports your platform's ABI(s).

(Windows has two...)

Note:

Processors don't understand 'function parameters'. They have registers, and they have the stack. The compiler of the caller function must decide where to place each argument - either in a register or on the stack. The compiler of the callee function (the function being called) must decide where to retrieve each argument from. There are also decisions to be made regarding which registers a function can freely re-use, and which registers must be carefully restore to their initial value on return. If a function can freely re-use a register, then the caller needs to think about saving and restoring the register contents. If each function is responsible to putting things back exactly as they were, then the caller has less work to do, but maybe you're saving and restoring registers that no-one cares about. When the stack is used, you also have agree whether the caller or the callee is responsible for resetting the stack point to where it was before the caller called the callee.

Think also what happens if you have a floating-point unit - do f32 and f64 values go into FPU registers, or are they placed in integer registers?

Clearly these two compilers must agree, otherwise the callee will not receive the correct arguments and your program will perform UB!

x86 is ~40 years old and many standards exist on how to do this. See https://en.wikipedia.org/wiki/X86_calling_conventions#Historical_background.

AMD64 is only ~15 years old, and there are two standards - the Microsoft one for Windows, and the Linux one (which is based on System V UNIX).

---

CPUs have registers, and they have a pointer to the stack (in RAM)

Where does this function find its arguments? Where does the return value go?

```rust []
struct SomeStruct(u32, f64);

fn hello(param1: i32, param2: f64) -> SomeStruct { todo!() }
```

## Libraries

Your Rust code might want to interact with shared/static libraries.

Or _be_ one.

## Efficient bindings

There are no conversion costs moving from C to Rust or vice-versa

## Using Rust from C

We have this amazing Rust library, we want to use in our existing C project.

```rust []
struct MagicAdder {
	amount: u32
}

impl MagicAdder {
	fn new(amount: u32) -> MagicAdder {
		MagicAdder {
			amount
		}
	}

	fn process_value(&self, value: u32) -> u32 {
		self.amount + value
	}
}
```

## Things TODO

- Tell C these functions exist
- Tell Rust to use C-compatible types and functions
- Link the external code as a library
- Provide some C types that match the Rust types
- Call our Rust functions

## C-flavoured Rust Code

```rust []
#[repr(C)]
struct MagicAdder {
	amount: u32
}

impl MagicAdder {
    fn new(amount: u32) -> MagicAdder { todo!() }
    fn process_value(&self, value: u32) -> u32 { todo!() }
}

#[no_mangle]
extern "C" fn magicadder_new(amount: u32) -> MagicAdder {
	MagicAdder::new(amount)
}

#[no_mangle]
extern "C" fn magicadder_process_value(adder: *const MagicAdder, value: u32) -> u32 {
	if let Some(ma) = unsafe { adder.as_ref() } {
		ma.process_value(value)
	} else {
		0
	}
}
```

Note:

The `.as_ref()` method on pointers *requires* that the pointer either be null, or that it point at a valid, aligned, fully initialized object. If they just feed you a random integer, bad things will happen, and we can't tell if they've done that!

## Matching C header

```c []
/// Designed to have the exact same shape as the Rust version
typedef struct magic_adder_t {
	uint32_t amount;
} magic_adder_t;

/// Wraps MagicAdder::new
magic_adder_t magicadder_new(uint32_t amount);

/// Wraps MagicAdder::process_value
uint32_t magicadder_process_value(magic_adder_t* self, uint32_t value);
```

## Making a library

You can tell `rustc` to make:

* binaries (bin)
* libraries (lib)
    - rlib
    - dylib
    - staticlib
    - cdylib

Note:

See https://doc.rust-lang.org/reference/linkage.html

## Cargo.toml

```toml
[package]
name = "magic_adder"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["lib", "staticlib", "cdylib"]
```

Note:

See ./examples/ffi_use_rust_in_c for a working example.

## Using C from Rust

---

We have this amazing C library, we want to use as-is in our Rust project.

`cool_library.h`:

```c []
/** Parse a null-terminated string */
unsigned int cool_library_function(const unsigned char* p);
```

`cool_library.c`:

```c []
#include "hello.h"

unsigned int cool_library_function(const unsigned char* s) {
    unsigned int result = 0;
    for(const char* p = s; *p; p++) {
        result *= 10;
        if ((*p < '0') || (*p > '9')) { return 0; }
        result += (*p - '0');
    }
    return result;
}
```

## Things TODO

- Tell Rust these functions exist
- Link the external code as a library
- Call those with `unsafe { ... }`
- Transmute data for C functions

## Naming things is hard

```rust
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
```

<p>&nbsp;<!-- spacer for "run" button --></p>

Disables some Rust naming lints

## Binding functions

```c
/** Parse a null-terminated string */
unsigned int cool_library_function(const char* p);
```

```rust []
use std::ffi::{c_char, c_uint}; // also in core::ffi

extern "C" {
    // We state that this function exists, but there's no definition.
    // The linker looks for this 'symbol name' in the other objects
    fn cool_library_function(p: *const c_char) -> c_uint;
}
```

Note:

You cannot do `extern "C" fn some_function();` with no function body - you must use the block.

## Primitive types

Some C types have direct Rust equivalents.

The [`core::ffi`](https://doc.rust-lang.org/stable/core/ffi/index.html) module
also defines a bunch of useful types and aliases.

| C             | Rust                   |
| ------------- | ---------------------- |
| int32_t       | i32                    |
| unsigned int  | c_uint                 |
| unsigned char | u8 (not char!)         |
| void          | ()                     |
| char\*        | CStr or \*const c_char |

Note:

On some systems, a C `char` is not 8 bits in size. Rust does not support those
platforms, and likely never will. Rust does support platforms where `int` is
only 16-bits in size.

## Calling this

```rust [] ignore
use std::ffi::{c_char, c_uint};

extern "C" {
    fn cool_library_function(p: *const c_char) -> c_uint;
}

fn main() {
    let s = c"123"; // <-- a null-terminated string!
    let result: u32 = unsafe { cool_library_function(s.as_ptr()) };
    println!("cool_library_function({s:?}) => {result}");
}
```

## Some more specific details...

## Cargo (build-system) support

* Build native code via build-dependency crates:
    - [cc](https://crates.io/crates/cc), [cmake](https://crates.io/crates/cmake), ...
* `build.rs` can give linker extra arguments

## Opaque types

When not knowing (or caring) about internal layout, [opaque structs](https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs) can be used.

```rust []
/// This is like a 'struct FoobarContext;' in C
#[repr(C)]
pub struct FoobarContext { _priv: [i32; 0] }

extern "C" {
	fn foobar_init() -> *mut FoobarContext;
	fn foobar_do(ctx: *mut FoobarContext, foo: i32);
	fn foobar_destroy(ctx: *mut FoobarContext);
}

/// Use this in your Rust code
pub struct FoobarHandle(*mut FoobarContext);
```

## Callbacks

`extern "C"` applies to function pointers given to extern functions too.

```rust [] ignore
use std::ffi::c_void;

pub type FooCallback = extern "C" fn(state: *mut c_void);

extern "C" {
    pub fn libfoo_register_callback(state: *mut c_void, cb: FooCallback);
}

extern "C" fn my_callback(_state: *mut c_void) {
    // Do stuff here
}

fn main() {
    unsafe { libfoo_register_callback(core::ptr::null_mut(), my_callback); }
}
```

## But this is a lot of manual work?

There's a better way!

## Making C headers from Rust

[cbindgen](https://crates.io/crates/cbindgen)

## Making Rust source from C headers

[bindgen](https://crates.io/crates/bindgen)

## Loading auto-generated Rust source

```rust ignore
#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
```

## Calling these tools:

* On the command line
* Executing a command in `build.rs`
* Calling a library function in `build.rs`

## sys crates

`xxxx-sys` is a Rust crate that provides a thin wrapper around some C library `xxxx`.

You normally have a higher-level `xxxx` crate that provides a Rust interface

Note:

For example libgit2-sys (wraps libgit2), or nrfxlib-sys (nRF9160 support)
