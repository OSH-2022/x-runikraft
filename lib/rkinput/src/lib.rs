// SPDX-License-Identifier: BSD-3-Clause
// blkdev.rs

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


use core::time::Duration;
use rkgpu::{update_cursor, draw_select, RED, CYAN};
use rkplat::drivers::virtio::{GPU_DEIVCE, INPUT_DEIVCE, InputEvent};
use rkplat::println;

const EV_REL: u16 = 0x02;
const BTN_LEFT: u16 = 0x110;
const BTN_RIGHT: u16 = 0x111;
const BTN_MIDDLE: u16 = 0x112;
const REL_X: u16 = 0x00;
const REL_Y: u16 = 0x01;

pub static mut MOUSE_X: u32 = 0;
pub static mut MOUSE_Y: u32 = 0;

pub static mut SELECT_X: u32 = 0;
pub static mut SELECT_Y: u32 = 0;
pub static mut INPUT_NUMBER: usize = 100;

const EV_KEY: u16 = 0x01;
const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const KEY_PAGEUP: u16 = 104;
const KEY_PAGEDOWN: u16 = 109;
const KEY_HOME: u16 = 102;
const KEY_END: u16 = 107;
const KEY_W: u16 = 17;
const KEY_S: u16 = 31;
const KEY_A: u16 = 30;
const KEY_D: u16 = 32;
const KEY_H: u16 = 35;
const KEY_O: u16 = 24;
const KEY_1: u16 = 1;
const KEY_2: u16 = 2;
const KEY_3: u16 = 3;
const KEY_4: u16 = 4;
const KEY_5: u16 = 5;
const KEY_6: u16 = 6;
const KEY_7: u16 = 7;
const KEY_8: u16 = 8;
const KEY_9: u16 = 9;
const KEY_BACKSPACE: u16 = 14;
const SHORT_STEP: u32 = 1;
const LONG_STEP: u32 = 20;

pub static mut CURSOR_X: u32 = 900;
pub static mut CURSOR_Y: u32 = 500;

pub fn input_handler(input_event: InputEvent) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        //println!("{},{},{}", input_event.event_type, input_event.code, input_event.value);
        let SELECT_OLD_X = SELECT_X;
        let SELECT_OLD_Y = SELECT_Y;
        if input_event.event_type == EV_KEY &&input_event.value==1{
            match input_event.code {
                KEY_UP => { if CURSOR_Y > SHORT_STEP { 
                    CURSOR_Y -= SHORT_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_DOWN => { if CURSOR_Y < height - SHORT_STEP { 
                    CURSOR_Y += SHORT_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_LEFT => { if CURSOR_X > SHORT_STEP { 
                    CURSOR_X -= SHORT_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_RIGHT => { if CURSOR_X < width - SHORT_STEP { 
                    CURSOR_X += SHORT_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_PAGEUP => { if CURSOR_Y > LONG_STEP { 
                    CURSOR_Y -= LONG_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_PAGEDOWN => { if CURSOR_Y < height - LONG_STEP { 
                    CURSOR_Y += LONG_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_HOME => { if CURSOR_X > LONG_STEP { 
                    CURSOR_X -= LONG_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_END => { if CURSOR_X < width - LONG_STEP { 
                    CURSOR_X += LONG_STEP;
                    update_cursor(CURSOR_X, CURSOR_Y, false); } 
                }
                KEY_W => { if SELECT_Y >= 75 {   
                    SELECT_Y -= 75;
                    draw_select(SELECT_OLD_X, SELECT_OLD_Y, CYAN);
                    draw_select(SELECT_X, SELECT_Y, RED); } 
                    INPUT_NUMBER = 100;
                }
                KEY_S => { if SELECT_Y < 600 { 
                    SELECT_Y += 75; 
                    draw_select(SELECT_OLD_X, SELECT_OLD_Y, CYAN);
                    draw_select(SELECT_X, SELECT_Y, RED);} 
                    INPUT_NUMBER = 100;
                }
                KEY_A => { if SELECT_X >= 75 { 
                    SELECT_X -= 75;
                    draw_select(SELECT_OLD_X, SELECT_OLD_Y, CYAN);
                    draw_select(SELECT_X, SELECT_Y, RED);} 
                    INPUT_NUMBER = 100;
                }
                KEY_D => { if SELECT_X < 600 { 
                    SELECT_X += 75;
                    draw_select(SELECT_OLD_X, SELECT_OLD_Y, CYAN);
                    draw_select(SELECT_X, SELECT_Y, RED);} 
                    INPUT_NUMBER = 100;
                }   
                KEY_H => { 
                    INPUT_NUMBER = KEY_H;
                } 
                KEY_O => { 
                    INPUT_NUMBER = KEY_O;
                } 
                KEY_1 => { 
                    INPUT_NUMBER = 1;
                } 
                KEY_2 => { 
                    INPUT_NUMBER = 2;
                } 
                KEY_3 => { 
                    INPUT_NUMBER = 3;
                } 
                KEY_4 => { 
                    INPUT_NUMBER = 4;
                } 
                KEY_5 => { 
                    INPUT_NUMBER = 5;
                } 
                KEY_6 => { 
                    INPUT_NUMBER = 6;
                } 
                KEY_7 => { 
                    INPUT_NUMBER = 7;
                } 
                KEY_8 => { 
                    INPUT_NUMBER = 8;
                } 
                KEY_9 => { 
                    INPUT_NUMBER = 9;
                } 
                KEY_BACKSPACE => { 
                    INPUT_NUMBER = 0;
                } 
                _ => {}
            }
        }
    }
}

pub fn input_tracer(_null: *mut u8) {
    unsafe {
        loop {
            let input_event_wrapped = INPUT_DEIVCE.as_mut().unwrap().pop_pending_event();
            match input_event_wrapped {
                Some(input_event) => input_handler(input_event),
                //None => rksched::this_thread::sleep_for(Duration::from_millis(1))
                None => { rksched::this_thread::r#yield() }
            }
        }
    }
}
