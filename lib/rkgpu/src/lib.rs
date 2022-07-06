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
pub mod output;

pub use color::*;
pub use output::*;

//use core::cmp::{max, min};
//use core::time::Duration;
use rkplat::drivers::virtio::__GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

static mut _EMPTY: [u8; 0] = [0; 0];
static DIC: [u128; 127] = include!("dic.txt");
pub static mut FB: &mut [u8] = unsafe { &mut _EMPTY };
pub static mut FB_CURSOR: &mut [u32] = &mut [0; 1000];

pub unsafe fn init() {
    let flag = rkplat::lcpu::save_irqf();
    FB = __GPU_DEIVCE.as_mut().unwrap().setup_framebuffer().expect("failed to get FB");
    draw_clear(LIGHT_CYAN);
    rkplat::lcpu::restore_irqf(flag);
}

pub fn resolution() -> (u32, u32) {
    unsafe { __GPU_DEIVCE.as_mut().unwrap().resolution() }
}

pub fn draw_clear(color: Color) {
    let flag = rkplat::lcpu::save_irqf();
    unsafe {
        let (width, height) = resolution();
        for y in 0..height as usize {
            for x in 0..width as usize {
                let idx = (y * width as usize + x) * 4;
                FB[idx + 2] = color.red;
                FB[idx + 1] = color.green;
                FB[idx + 0] = color.blue;
                FB[idx + 3] = 255;
            }
        }
        __GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
    rkplat::lcpu::restore_irqf(flag);
}

pub fn screen_flush() {
    let flag = rkplat::lcpu::save_irqf();
    unsafe {
        __GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
    rkplat::lcpu::restore_irqf(flag);
}
