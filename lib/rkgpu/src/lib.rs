// SPDX-License-Identifier: BSD-3-Clause
// rkgpu/lib.rs

// Authors:  郭耸霄 <logname@mail.ustc.edu.cn>
// Authors:  蓝俊玮 <ljw13@mail.ustc.edu.cn>

// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#![no_std]

use core::cmp::{max, min};
use core::time::Duration;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color {
            red,
            green,
            blue,
        }
    }
}

pub const WHITE: Color = Color::new(255, 255, 255);
pub const BLACK: Color = Color::new(0, 0, 0);
pub const RED: Color = Color::new(255, 0, 0);
pub const GREEN: Color = Color::new(0, 255, 0);
pub const BLUE: Color = Color::new(0, 0, 255);
pub const CYAN: Color = Color::new(0, 255, 255);
pub const PURPLE: Color = Color::new(255, 0, 255);
pub const GRAY: Color = Color::new(105, 105, 105);

static mut _EMPTY: [u8; 0] = [0; 0];

static mut FB: &mut [u8] = unsafe { &mut _EMPTY };
static mut FB_CURSOR: &mut [u32] = &mut [0; 1000];
//static CURSOR: [u8; 16 * 16 * 4] = include!("cursor.txt");

pub unsafe fn init() {
    // static mut CURSOR_NEW: [u8; 64 * 64 * 4] = [0; 64 * 64 * 4];
    // for i in 0..16 {
    //     for j in 0..16 {
    //         CURSOR_NEW[(i * 64 + j) * 4 + 0] = CURSOR[(i * 16 + j) * 4 + 0];
    //         CURSOR_NEW[(i * 64 + j) * 4 + 1] = CURSOR[(i * 16 + j) * 4 + 1];
    //         CURSOR_NEW[(i * 64 + j) * 4 + 2] = CURSOR[(i * 16 + j) * 4 + 2];
    //         CURSOR_NEW[(i * 64 + j) * 4 + 3] = CURSOR[(i * 16 + j) * 4 + 3];
    //     }
    // }
    FB = GPU_DEIVCE.as_mut().unwrap().setup_framebuffer().expect("failed to get FB");
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    // draw_font(width / 2 - 4 * 16, height / 2 - 8 * 16, (0, 0, 0, 1), 3 + 48, 16);
    // GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    // rksched::this_thread::sleep_for(Duration::from_secs(1));
    // draw_font(width / 2 - 4 * 16, height / 2 - 8 * 16, (0, 0, 0, 1), 2 + 48, 16);
    // GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    // rksched::this_thread::sleep_for(Duration::from_secs(1));
    draw_font(width / 2 - 4 * 16, height / 2 - 8 * 16, WHITE, 255, '1', 16);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    rksched::this_thread::sleep_for(Duration::from_secs(1));
    draw_clear(CYAN);
    printg("Hello, world!\nHello, OSH-2022!\nHello, Runikraft!\n", 700, 10, RED, 255, 4);
    update_cursor(900, 500, true);
    draw_select(0, 0, RED);
    // rksched::this_thread::sleep_for(Duration::from_secs(1));
    // update_cursor(150, 100, false);
    // rksched::this_thread::sleep_for(Duration::from_secs(1));
    // update_cursor(200, 100, false);
    // rksched::this_thread::sleep_for(Duration::from_secs(1));
    //GPU_DEIVCE.as_mut().unwrap().setup_cursor(&CURSOR_NEW, 50, 50, 0, 0).expect("failed to set up cursor.");
}

pub enum DIRECTION {
    Horizontal,
    Vertical,
}

pub fn resolution() -> (u32, u32) {
    unsafe { GPU_DEIVCE.as_mut().unwrap().resolution() }
}

pub fn draw_line(direction: DIRECTION, start_x: u32, start_y: u32, length: u32, color: Color, alpha: u8, line_width: u32) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        match direction {
            Horizontal => {
                for y in 0..min(line_width, height - start_y) {
                    for x in 0..min(length, width - start_x) {
                        let idx = ((start_y + y) * width + x + start_x) * 4;
                        FB[idx as usize + 2] = color.red;
                        FB[idx as usize + 1] = color.green;
                        FB[idx as usize + 0] = color.blue;
                        FB[idx as usize + 3] = alpha;
                    }
                }
            }
            Vertical => {
                for x in 0..min(line_width, width - start_x) {
                    for y in 0..min(length, height - start_y) {
                        let idx = ((y + start_y) * width + x + start_x) * 4;
                        FB[idx as usize + 2] = color.red;
                        FB[idx as usize + 1] = color.green;
                        FB[idx as usize + 0] = color.blue;
                        FB[idx as usize + 3] = alpha;
                    }
                }
            }
        }

    }
}

pub fn draw_clear(color: Color) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        for y in 0..height as usize {
            for x in 0..width as usize {
                let idx = (y * width as usize + x) * 4;
                FB[idx + 2] = color.red;
                FB[idx + 1] = color.green;
                FB[idx + 0] = color.blue;
                FB[idx + 3] = 255;
            }
        }
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}

static DIC: [u128; 127] = include!("dic.txt");

pub fn draw_font(start_x: u32, start_y: u32, color: Color, alpha: u8, ch: char, size: u8) -> u8 {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        if start_x + 8 * size as u32 <= width && start_y + 16 * size as u32 <= height {
            let pos = DIC[ch as usize];
            for y in start_y..start_y + 16 * size as u32 {
                for x in start_x..start_x + 8 * size as u32 {
                    let idx = ((y * width + x) * 4) as usize;
                    let num = ((y - start_y) / size as u32 * 8 + (x - start_x) / size as u32) as usize;
                    if pos & (1 << (127 - num)) == (1 << (127 - num)) {
                        FB[idx + 2] = color.red;
                        FB[idx + 1] = color.green;
                        FB[idx + 0] = color.blue;
                        FB[idx + 3] = alpha;
                    } else {
                        FB[idx + 2] = CYAN.red;
                        FB[idx + 1] = CYAN.green;
                        FB[idx + 0] = CYAN.blue;
                        FB[idx + 3] = 255;
                    }
                }
            }

            0
        } else { 1 }
    }
}


pub fn printg(ascii_str: &str, start_x: u32, start_y: u32, color: Color, alpha: u8, size: u8) {
    unsafe {
        let mut x = start_x;
        let mut y = start_y;
        for ascii in ascii_str.chars() {
            if ascii == '\n' {
                x = start_x;
                y += 16 * size as u32;
            } else {
                draw_font(x, y, color, alpha, ascii, size);
                x += 8 * size as u32;
            }
        }
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}


pub fn update_cursor(start_x: u32, start_y: u32, is_init: bool) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        if !is_init {
            for i in 1..(FB_CURSOR[0] + 1) as usize {
                FB[FB_CURSOR[5 * i - 4] as usize + 2] = FB_CURSOR[5 * i - 4 + 1] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 1] = FB_CURSOR[5 * i - 4 + 2] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 0] = FB_CURSOR[5 * i - 4 + 3] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 3] = FB_CURSOR[5 * i - 4 + 4] as u8;
            }
        }
        let mut idx_cursor = 1;
        for y in max(start_y, 1) - 1..min(start_y + 1, height - 1) + 1 {
            for x in max(start_x, 10) - 10..min(start_x + 10, width - 1) + 1 {
                let idx = (y * width + x) * 4;
                FB_CURSOR[idx_cursor] = idx;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 2] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 1] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 0] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 3] as u32;
                idx_cursor += 1;
            }
        }
        for x in max(start_x, 1) - 1..min(start_x + 1, width - 1) + 1 {
            for y in max(start_y, 10) - 10..min(start_y + 10, height - 1) + 1 {
                let idx = (y * width + x) * 4;
                FB_CURSOR[idx_cursor] = idx;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 2] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 1] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 0] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 3] as u32;
                idx_cursor += 1;
            }
        }
        FB_CURSOR[0] = (idx_cursor / 5) as u32;
        draw_line(Horizontal, (max(10, start_x) - 10) as u32, (max(1, start_y) - 1) as u32, min(21, start_x + 10), BLACK, 255, 3);
        draw_line(Vertical, (max(1, start_x) - 1) as u32, (max(10, start_y) - 10) as u32, min(21, start_y + 10), BLACK, 255, 3);
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}
pub unsafe fn draw_select(start_x: u32, start_y: u32, color: Color) {
    draw_line(Horizontal, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Horizontal, start_x + 5, start_y + 70, 65, color, 255, 1);
    draw_line(Vertical, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Vertical, start_x + 70, start_y + 5, 65, color, 255, 1);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}

pub fn screen_flush(){
    unsafe {
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}