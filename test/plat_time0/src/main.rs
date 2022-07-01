#![no_std]
#![no_main]

extern crate rkboot;
#[macro_use]
extern crate rkplat;

use rkplat::time::{wall_clock,monotonic_clock,get_ticks};

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let time1 = wall_clock();
    let time2 = monotonic_clock();
    let time3 = get_ticks();
    println!("wall_clock()={:?}\nmonotonic_clock()={:?}\nget_ticks()={:?}",time1,time2,time3);
    rkplat::println!("Test plat_time0 passed!");
    0
}
