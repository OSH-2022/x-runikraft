// SPDX-License-Identifier: BSD-3-Clause
// sudoku/main.rs

// Authors:  蓝俊玮 <ljw13@mail.ustc.edu.cn>
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

use core::time::Duration;
use rkplat::drivers::virtio::__GPU_DEIVCE;
use rkgpu::*;
use rkplat::time::wall_clock;
use rktimeconv::*;
use crate::{mutex, Sudoku};

pub unsafe fn draw_sudoku_lattices(color0: Color, color1: Color) -> u8 {
    let (width, height) = __GPU_DEIVCE.as_mut().unwrap().resolution();
    if width >= 750 && height >= 750 {
        for x in 0..10 {
            if x % 3 == 0 {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, color0, 255, 4);
            } else {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, color1, 255, 1);
            }
        }
        for y in 0..10 {
            if y % 3 == 0 {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, color0, 255, 4);
            } else {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, color1, 255, 1);
            }
        }
        screen_flush();
        1
    } else { 0 }
}

pub unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8, color: Color) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 75 * pos_x as u32 + 20;
        let start_y: u32 = 75 * pos_y as u32 + 6;
        if number == 0 {
            draw_font(start_x, start_y, LIGHT_CYAN, 255, ' ', 4);
        } else {
            draw_font(start_x, start_y, color, 255, (number + 48).into(), 4);
        }
        screen_flush();
        0
    } else { 1 }
}

pub fn show_time(_null: *mut u8) {
    loop {
        let timepoint_from_unix: Duration = wall_clock();
        let timepoint: TimePoint = TimePoint::from_unix_time(timepoint_from_unix);
        let time = alloc::format!("Time: {:04}-{:02}-{:02} {:02}:{:02}:{:02}(UTC)", timepoint.year(), timepoint.month() + 1,
                                  timepoint.day(), timepoint.hour(), timepoint.min(), timepoint.second());
        printg(time.as_str(), 700, 300, BLUE, 255, 2);
        rksched::this_thread::sleep_for(Duration::from_secs(1));
    }
}

pub fn error_hinter(_null: *mut u8) {
    unsafe {
        loop {
            mutex.wait();
            printg("You can't write this number HERE!", 700, 500, RED, 255, 2);
            rksched::this_thread::sleep_for(Duration::from_secs(1));
            printg("                                 ", 700, 500, RED, 255, 2);
        }
    }
}


impl Sudoku {
    // 打印当前数独信息
    pub unsafe fn map_print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                // show_sudoku_number(pos_x: u8, pos_y: u8, number: u8);
                if self.tag[i][j] == 0 {
                    show_sudoku_number(i as u8, j as u8, self.map[i][j] as u8, GRAY);
                    continue;
                } else {
                    show_sudoku_number(i as u8, j as u8, self.map[i][j] as u8, BLACK);
                    // print!("{} ", self.map[i][j]);
                }
            }
            // println!("");
        }
    }
}