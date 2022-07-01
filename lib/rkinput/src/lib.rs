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


use core::time::Duration;
use rkgpu::{update_cursor, draw_select};
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

pub static mut SELECT_X: u32 = 1;
pub static mut SELECT_Y: u32 = 1;

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

const SHORT_STEP: u32 = 1;
const LONG_STEP: u32 = 20;

pub static mut CURSOR_X: u32 = 100;
pub static mut CURSOR_Y: u32 = 100;

pub fn input_handler(input_event: InputEvent) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        println!("{},{},{}", input_event.event_type, input_event.code, input_event.value);
        if input_event.event_type == EV_KEY &&input_event.value==1{
            match input_event.code {
                KEY_UP => { if CURSOR_Y > SHORT_STEP { CURSOR_Y -= SHORT_STEP } }
                KEY_DOWN => { if CURSOR_Y < height - SHORT_STEP { CURSOR_Y += SHORT_STEP } }
                KEY_LEFT => { if CURSOR_X > SHORT_STEP { CURSOR_X -= SHORT_STEP } }
                KEY_RIGHT => { if CURSOR_X < width - SHORT_STEP { CURSOR_X += SHORT_STEP } }
                KEY_PAGEUP => { if CURSOR_Y > LONG_STEP { CURSOR_Y -= LONG_STEP } }
                KEY_PAGEDOWN => { if CURSOR_Y < height - LONG_STEP { CURSOR_Y += LONG_STEP } }
                KEY_HOME => { if CURSOR_X > LONG_STEP { CURSOR_X -= LONG_STEP } }
                KEY_END => { if CURSOR_X < width - LONG_STEP { CURSOR_X += LONG_STEP } }
                KEY_W => { if SELECT_Y > 75 { SELECT_Y -= 75} }
                KEY_S => { if SELECT_Y < 600 { SELECT_Y += 75} }
                KEY_A => { if SELECT_X > 75 { SELECT_X -= 75} }
                KEY_D => { if SELECT_X < 600 { SELECT_X += 75} }
                _ => {}
            }
            update_cursor(CURSOR_X, CURSOR_Y, false);
            draw_select(SELECT_X, SELECT_Y);
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
