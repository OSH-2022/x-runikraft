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

use rkgpu::{draw_font,DIRECTION,draw_line,resolution};
unsafe fn draw_sudoku_lattices() -> u8 {
    let (width, height) = resolution();
    if width >= 750 && height >= 750 {
        for x in 0..10 {
            if x % 3 == 0 {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, (0, 0, 0, 1), 4);
            } else {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, (0, 0, 0, 1), 1);
            }
        }
        for y in 0..10 {
            if y % 3 == 0 {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, (0, 0, 0, 1), 4);
            } else {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, (0, 0, 0, 1), 1);
            }
        }
        1
    } else { 0 }
}

unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 75 * pos_x as u32 + 20;
        let start_y: u32 = 75 * pos_y as u32 + 8;
        draw_font(start_x, start_y, (0, 0, 0, 1), number + 48, 4);
        0
    } else { 1 }
}
