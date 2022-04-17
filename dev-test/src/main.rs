#![no_std]
#![no_main]

#[macro_use]
extern crate runikraft;

use rkalloc::RKalloc;
use rkalloc_empty::RKallocEmpty;

static mut HEAP_SPACE: [u8;1000] = [0;1000];

#[no_mangle]
fn main() {
    let mut alloc;
    unsafe {
        alloc = RKallocEmpty::new(HEAP_SPACE.as_mut_ptr(),1000);
    }
    println!("Hello, world!");
    let p1 = unsafe{alloc.malloc(10)};
    println!("p1={:?}",p1);
    let p2 = unsafe{alloc.malloc(5)};
    println!("p2={:?}",p2);
}
