//.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 alloc_error_handler.rs --crate-type lib --target riscv64gc-unknown-none-elf
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[alloc_error_handler]
pub fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!();
}
