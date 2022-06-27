// SPDX-License-Identifier: BSD-3-Clause
// goldfish.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::{RtcDevice, super::Device};
use core::{slice,str};
use core::time::Duration;
use crate::device::ioreg_read32;

const TIME_LOW:  usize = 0x00;      // R: Get current time, then return low-order 32-bits.
const TIME_HIGH: usize = 0x04;      // R: Return high 32-bits, from previous TIME_LOW read.

pub struct GoldfishRtc {
    addr: usize,
    name: [u8;32],
    name_size: usize,
}

impl GoldfishRtc {
    pub fn new(name: &str, addr: usize) -> Self {
        assert!(name.len()<=32);
        let mut name1: [u8;32] = [0;32];
        for i in 0..name.len() {
            name1[i] = name.as_bytes()[i];
        }
        println_bios!("Init goldfish RTC device, name={},addr=0x{:x}.",name,addr);

        Self {
            addr,
            name: name1,
            name_size: name.len()
        }
    }

    #[inline(always)]
    fn reg(&self, r: usize) -> *mut u32{
        (self.addr + r) as *mut u32
    }

    #[inline(always)]
    fn reg_read(&self, r: usize) -> u32 {
        unsafe{ioreg_read32(self.reg(r))}
    }
}

impl Device for GoldfishRtc {
    fn name<'a>(&'a self) -> &'a str {
        unsafe {str::from_utf8_unchecked(slice::from_raw_parts(self.name.as_ptr(), self.name_size))}
    }
}

impl RtcDevice for GoldfishRtc {
    fn time(&self) -> Duration {
        let mut high = self.reg_read(TIME_HIGH);
        let mut low = self.reg_read(TIME_LOW);
        let high2 = self.reg_read(TIME_HIGH);
        if high2 != high {
            low = self.reg_read(TIME_LOW);
            high = high2;
        }
        Duration::from_nanos(((high as u64)<<32) + (low as u64))
    }
}
