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
    unix_time: Duration,
}

impl TimePoint {
    pub fn from_unix_time(unix_time: Duration) -> Self {
        let unix_time_secs = unix_time.as_secs();
        let unix_time_nanos = unix_time.as_nanos() - (unix_time_secs * 1000000000) as u128;
        let one_day_secs = 86400;
        let mut total_day = unix_time_secs / one_day_secs;
        let day_in_week = (total_day + 4) % 7;
        let secs_remain = unix_time_secs - total_day * one_day_secs;
        let hour = secs_remain / 3600;
        let min = (secs_remain - (hour as u64 * 3600)) / 60;
        let sec = secs_remain - hour * 3600 - min * 60;
        let mut begin_year: u32 = 1970;
        loop {
            if is_leap_year(begin_year) {
                if total_day >= 366 {
                    total_day -= 366;
                    begin_year += 1;
                } else {
                    break;
                }
            } else {
                if total_day >= 365 {
                    total_day -= 365;
                    begin_year += 1;
                } else {
                    break;
                }
            }
        }
        let day_in_year = total_day;
        let mut month: u8 = 0;
        for mon in 0..=11 {
            if total_day >= day_in_month(mon, begin_year) as u64 {
                total_day = total_day - day_in_month(mon, begin_year) as u64;
            } else {
                month = mon;
                break;
            }
        }
        TimePoint {
            year: begin_year,
            mon: month,
            day: (total_day + 1) as u8,
	        hour: hour as u8,
	        min: min as u8,
	        sec: sec as u8,
            nanosec: unix_time_nanos as u32,
            day_in_week: day_in_week as u8,
            day_in_year: day_in_year as u16,
            unix_time: unix_time
        }
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

pub fn is_leap_year (year: u32) -> bool {
    if year % 400 == 0 {
        return true;
    } else if year % 100 == 0 {
        return false;
    } else if year % 4 == 0 {
        return true;
    } else {
        return false;
    }
}

pub fn day_in_month(month: u8, year: u32) -> u32 {
    let day_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) {
        if month == 2 {
            return day_in_month[month as usize] + 1;
        } else {
            return day_in_month[month as usize];
        }
    } else {
        return day_in_month[month as usize];
    }
}
