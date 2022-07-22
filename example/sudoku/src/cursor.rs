// SPDX-License-Identifier: BSD-3-Clause
// rkgpudev/cursor.rs

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

use core::cmp::{max, min};
use crate::DIRECTION::*;
use crate::*;


pub fn update_cursor(start_x: u32, start_y: u32, is_init: bool) {
    unsafe {
        let (width, height) = rkgpudev::resolution();
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
        rkgpudev::screen_flush();
    }
}

pub fn draw_select(start_x: u32, start_y: u32, color: Color) {
    draw_line(Horizontal, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Horizontal, start_x + 5, start_y + 70, 65, color, 255, 1);
    draw_line(Vertical, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Vertical, start_x + 70, start_y + 5, 65, color, 255, 1);
    super::screen_flush();
}
