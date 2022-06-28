// SPDX-License-Identifier: BSD-3-Clause
// blkdev.rs

// Authors:  郭耸霄 <logname@mail.ustc.edu.cn>

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

use core::cmp::min;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

static mut _EMPTY: [u8; 0] = [0; 0];

pub static mut FB: &mut [u8] = unsafe { &mut _EMPTY };

pub unsafe fn init() {
    FB = GPU_DEIVCE.as_mut().unwrap().setup_framebuffer().expect("failed to get FB");
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = (y * width as usize + x) * 4;
            FB[idx] = x as u8;
            FB[idx + 1] = y as u8;
            FB[idx + 2] = (x + y) as u8;
        }
    }
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}

pub enum DIRECTION {
    Horizontal,
    Vertical,
}

pub unsafe fn draw_line(direction: DIRECTION, start_x: u32, start_y: u32, length: u32, rgb: (u8, u8, u8, u8)) {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    match direction {
        Horizontal => {
            for x in 0..min(length, width - start_x) {
                let idx = (start_y * width + x) * 4;
                FB[idx as usize + 0] = rgb.0;
                FB[idx as usize + 1] = rgb.1;
                FB[idx as usize + 2] = rgb.2;
                FB[idx as usize + 3] = rgb.3;
            }
        }
        Vertical => {
            for y in 0..min(length, height - start_y) {
                let idx = (y * width + start_x) * 4;
                FB[idx as usize + 0] = rgb.0;
                FB[idx as usize + 1] = rgb.1;
                FB[idx as usize + 2] = rgb.2;
                FB[idx as usize + 3] = rgb.3;
            }
        }
    }
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}

pub unsafe fn draw_clear() {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = (y * width as usize + x) * 4;
            FB[idx] = 255;
            FB[idx + 1] = 255;
            FB[idx + 2] = 255;
            FB[idx + 3] = 1;
        }
    }
}

static DIC: [u128; 126] = include!("dic.txt");

pub unsafe fn draw_font(start_x: u32, start_y: u32, rgb: (u8, u8, u8, u8), font: u8) -> u8 {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    if start_x + 8 <= width && start_y + 16 <= height {
        let mut pos = DIC[font as usize];
        for x in start_x..start_x + 7 {
            for y in start_y..start_y + 7 {
                let idx = ((y * width  + x) * 4) as usize;
                if pos & (1 << 127) == 1 << 127 {
                    FB[idx] = rgb.0;
                    FB[idx + 1] = rgb.1;
                    FB[idx + 2] = rgb.2;
                    FB[idx + 3] = rgb.3;
                } else {
                    FB[idx] = 255;
                    FB[idx + 1] = 255;
                    FB[idx + 2] = 255;
                    FB[idx + 3] = 255;
                }
                pos <<= 1;
            }
        }
        0
    }
    else {1}
}

pub unsafe fn draw_sudoku_lattices() -> u8 {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    if width >= 500 && height >= 500 {
        for x in 0..10 {
            draw_line(Vertical, x * 50, 0, 450, (0, 0, 0, 1));
        }
        for y in 0..10 {
            draw_line(Horizontal, 0, y * 50, 450, (0, 0, 0, 1));
        }
        1
    }
    else {0}
}

pub unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 50 * pos_x as u32 + 20;
        let start_y: u32 = 50 * pos_y as u32 + 20;
        draw_font(start_x, start_y, (0, 0, 0, 1), number+48);
        0
    }
    else {1}
}