#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Fail to allocate memory. layout={:?}",layout);
}
