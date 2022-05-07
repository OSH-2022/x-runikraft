#![no_std]
#![no_main]

extern crate rkboot;

#[no_mangle]
unsafe fn main() {
    let address = 0 as *mut u8;
    *address = 2;
}
