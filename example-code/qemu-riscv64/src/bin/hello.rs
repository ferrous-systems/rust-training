//! Simple Hello, World for RISC-V

#![no_std]
#![no_main]

#[riscv_rt::entry]
fn main() -> ! {
    qemu_riscv64::println!("Hello, world!");
    panic!("oops");
}

