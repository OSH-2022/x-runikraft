// test rkgpu

#![no_std]
#![no_main]
extern crate rkboot;

use rkgpu::*;
use rkplat::drivers::virtio::__GPU_DEIVCE;
use core::time::Duration;
// use core::slice;
// use core::mem::{size_of, align_of};
// use core::ptr::NonNull;

#[no_mangle]
unsafe fn main(_args: &mut [&str]) -> i32 {
    init();
    draw_sudoku_lattices();
    printg("asdfh\nansi", 100, 100, RED, 255, 8);
    rksched::this_thread::sleep_for(Duration::from_secs(5));
    rkplat::println!("\nTest gpu0 passed!\n");
    return 0;
}

unsafe fn draw_sudoku_lattices() -> u8 {
    let (width, height) = __GPU_DEIVCE.as_mut().unwrap().resolution();
    if width >= 750 && height >= 750 {
        for x in 0..10 {
            if x % 3 == 0 {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, PURPLE, 255, 4);
            } else {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, BLUE, 255, 1);
            }
        }
        for y in 0..10 {
            if y % 3 == 0 {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, PURPLE, 255, 4);
            } else {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, BLUE, 255, 1);
            }
        }
        1
    } else { 0 }
}

unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 75 * pos_x as u32 + 20;
        let start_y: u32 = 75 * pos_y as u32 + 8;
        draw_font(start_x, start_y, BLACK, 255, (number + 48).into(), 4);
        0
    } else { 1 }
}
