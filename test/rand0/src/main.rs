#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate rkplat;
extern crate rkboot;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    for _ in 0..10 {
        println!("0x{:016x}",rkswrand::random::<u64>());
    }

    println!("Test rand0 passed!");
    0
}
