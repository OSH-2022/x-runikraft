// SPDX-License-Identifier: BSD-3-Clause
// rktimeconv/lib.rs

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

/// 时间点
pub struct TimePoint {
    //Unikraft的year可以小于0，但core::time::Duration不能表示负的时间间隔，
    //所以在Runikraft中，year必须大于0
    year: u32, 
	mon: u8,
	day: u8,
	hour: u8,
	min: u8,
	sec: u8,
    nanosec: u32,
    day_in_week: u8,
    day_in_year: u16,
    week: u8,
    unix_time: Duration,
}

impl TimePoint {
    pub fn from_unix_time(unix_time: Duration) -> Self {
        todo!()
    }
    pub fn year(&self) -> u32 {
        self.year
    }
    pub fn month(&self) -> u32 {
        self.mon as u32
    }
    pub fn day(&self) -> u32 {
        self.day as u32
    }
    pub fn week(&self) -> u32 {
        self.week as u32
    }
    pub fn hour(&self) -> u32 {
        self.hour as u32
    }
    pub fn min(&self) -> u32 {
        self.min as u32
    }
    pub fn second(&self) -> u32 {
        self.sec as u32
    }
    pub fn nanosec(&self) -> u32 {
        self.nanosec as u32
    }
    pub fn day_in_year(&self) -> u32 {
        self.day_in_year as u32
    }
    pub fn day_in_week(&self) -> u32 {
        self.day_in_week as u32
    }
    pub fn to_unix_time(&self) -> Duration {
        self.unix_time
    }
}
