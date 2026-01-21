//! An example program for QEMU's Armv8-R Virtual Machine
//!
//! Written by Jonathan Pallant at Ferrous Systems
//!
//! Copyright (c) Ferrous Systems, 2025

#![no_std]
#![no_main]

extern crate alloc;

use aarch64_rt::entry;
use embedded_alloc::Heap;

use qemu_aarch64v8a as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

entry!(main);

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `aarch64-rt`
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    // Initialize the allocator BEFORE you use it
    {
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
        unsafe {
            let heap_start = core::ptr::addr_of_mut!(HEAP_MEM);
            HEAP.init(heap_start as usize, HEAP_SIZE);
        }
    }
    defmt::println!("This is the with_heap example.");
    let pi: f64 = core::f64::consts::PI;
    let s = alloc::format!("This is a heap allocated string, π = {:0.6}", pi);
    defmt::println!("s = {:?}", s.as_str());
    semihosting::process::exit(0);
}

// End of file
