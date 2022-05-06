#![no_std]
#![no_main]

extern crate rkplat;

#[no_mangle]
unsafe fn main() {
    let address = 0 as *mut u8;
    *address = 2;
}
