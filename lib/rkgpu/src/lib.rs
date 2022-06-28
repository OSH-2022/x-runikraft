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
use core::time::Duration;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

static mut _EMPTY: [u8; 0] = [0; 0];

pub static mut FB: &mut [u8] = unsafe { &mut _EMPTY };

pub unsafe fn init() {
    FB = GPU_DEIVCE.as_mut().unwrap().setup_framebuffer().expect("failed to get FB");
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    draw_font(width/2-4*16,height/2-8*16,(0,0,0,1),3+48,16);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    rksched::this_thread::sleep_for(Duration::from_secs(1));
    draw_font(width/2-4*16,height/2-8*16,(0,0,0,1),2+48,16);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    rksched::this_thread::sleep_for(Duration::from_secs(1));
    draw_font(width/2-4*16,height/2-8*16,(0,0,0,1),1+48,16);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    rksched::this_thread::sleep_for(Duration::from_secs(1));
    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = (y * width as usize + x) * 4;
            FB[idx] = 255;
            FB[idx + 1] = 255;
            FB[idx + 2] = 255;
            FB[idx + 3] = 1;
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

static DIC: [u128; 127] = include!("dic.txt");

pub unsafe fn draw_font(start_x: u32, start_y: u32, rgb: (u8, u8, u8, u8), ascii: u8, size: u8) -> u8 {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    if start_x + 8 * size as u32 <= width && start_y + 16 * size as u32 <= height {
        let pos=DIC[ascii as usize];
        for y in start_y..start_y + 16 * size as u32 {
            for x in start_x..start_x + 8 * size as u32 {
                let idx = ((y * width + x) * 4) as usize;
                let num = ((y-start_y) / size as u32 * 8 + (x-start_x) / size as u32) as usize;
                if pos & (1 << (127 - num)) == (1 << (127 - num)) {
                    FB[idx] = rgb.0;
                    FB[idx + 1] = rgb.1;
                    FB[idx + 2] = rgb.2;
                    FB[idx + 3] = rgb.3;
                } else {
                    FB[idx] = 255;
                    FB[idx + 1] = 255;
                    FB[idx + 2] = 255;
                    FB[idx + 3] = 1;
                }
            }
        }
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
        0
    } else { 1 }
}

pub unsafe fn draw_sudoku_lattices() -> u8 {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    if width >= 750 && height >= 750 {
        for x in 0..10 {
            draw_line(Vertical, x * 75, 0, 675, (0, 0, 0, 1));
        }
        for y in 0..10 {
            draw_line(Horizontal, 0, y * 75, 675, (0, 0, 0, 1));
        }
        1
    } else { 0 }
}

pub unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 75 * pos_x as u32 +20;
        let start_y: u32 = 75 * pos_y as u32 +8;
        draw_font(start_x, start_y, (0, 0, 0, 1), number + 48, 4);
        0
    } else { 1 }
}