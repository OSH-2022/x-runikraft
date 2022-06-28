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
    draw_sudoku_lattices();
    show_sudoku_number(0, 0, 0);
    show_sudoku_number(0, 5, 9);
    show_sudoku_number(8, 8, 4);
    show_sudoku_number(4, 4, 7);
    show_sudoku_number(2, 7, 1);
    rksched::this_thread::sleep_for(Duration::from_secs(5));
    rkplat::println!("\nTest rkgpu0 passed!\n");
    return 0;
}
