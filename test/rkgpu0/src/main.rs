// test rkgpu

#![no_std]
#![no_main]
extern crate rkboot;

use rkgpu::*;
use core::time::Duration;
// use core::slice;
// use core::mem::{size_of, align_of};
// use core::ptr::NonNull;

#[no_mangle]
unsafe fn main(_args: &mut [&str])->i32 {
    init();
    rksched::this_thread::sleep_for(Duration::from_secs(1));
    rkplat::println!("\nTest rkgpu0 passed!\n");
    return 0;
}
