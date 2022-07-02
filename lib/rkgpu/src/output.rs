// SPDX-License-Identifier: BSD-3-Clause
// rkgpu/output.rs

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
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::*;


pub enum DIRECTION {
    Horizontal,
    Vertical,
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
