// SPDX-License-Identifier: BSD-3-Clause
// rkgpu/lib.rs

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

pub mod color;
pub mod cursor;
pub mod output;

pub use color::*;
pub use cursor::*;
pub use output::*;

use core::cmp::{max, min};
use core::time::Duration;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

static mut _EMPTY: [u8; 0] = [0; 0];
static DIC: [u128; 127] = include!("dic.txt");
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
}

pub fn resolution() -> (u32, u32) {
    unsafe { GPU_DEIVCE.as_mut().unwrap().resolution() }
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

pub fn screen_flush() {
    unsafe {
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}