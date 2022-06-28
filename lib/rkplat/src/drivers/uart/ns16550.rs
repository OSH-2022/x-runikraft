// SPDX-License-Identifier: BSD-3-Clause
// ns16550.rs

// Authors: 张子辰 <zichen350@gmail.com>

// Copyright (c) 2021 OpenSynergy GmbH.
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

#![allow(dead_code)]

use core::{str, slice};
use log::info;

use super::UartDevice;
use crate::drivers::Device;
use crate::device::{ioreg_read8,ioreg_write8};
use crate::lcpu::spinwait;

const THR: usize = 0x00;
const RBR: usize = 0x00;
const IER: usize = 0x01;
const IIR: usize = 0x02;
const FCR: usize = 0x02;
const LCR: usize = 0x03;
const MCR: usize = 0x04;
const LSR: usize = 0x05;
const MSR: usize = 0x06;

const REG_SHIFT: usize = 0x00;

const LCR_DLAB: u8    = 0x80;
const IER_INT_EN: u8  = 0x01;
const FCR_FIFO_EN: u8 = 0x01;
const LSR_RX_READY: u8 = 0x01;
const LSR_TX_VALID: u8 = 0x40;

pub struct Ns16550 {
    addr: usize,  //地址
    irq: usize,     //中断号
    name: [u8;32],
    name_size: usize,
}

impl Ns16550 {
    pub fn new(name: &str, addr: usize, irq: usize) -> Self{
        assert!(name.len()<=32);
        let mut name1: [u8;32] = [0;32];
        for i in 0..name.len() {
            name1[i] = name.as_bytes()[i];
        }
        info!("Init ns16550 device, name={},addr=0x{:x},irq={}.",name,addr,irq);

        Self {
            addr,irq,
            name: name1,
            name_size: name.len()
        }.init()
    }

    #[inline(always)]
    fn reg(&self, r: usize) -> *mut u8{
        (self.addr + (r<<REG_SHIFT)) as *mut u8
    }

    #[inline(always)]
    fn reg_read(&self, r: usize) -> u8 {
        unsafe{ioreg_read8(self.reg(r))}
    }

    #[inline(always)]
    fn reg_write(&self, r: usize, v: u8) {
        unsafe{ ioreg_write8(self.reg(r), v) }
    }

    fn init(self) -> Self {
        //Disable all interrupts
        self.reg_write(IER,self.reg_read(IER) & !IER_INT_EN);
        //Enable FIFOs
        self.reg_write(FCR, self.reg_read(FCR) | FCR_FIFO_EN);
        //8 bit word length
        self.reg_write(LCR, 0x3);
        self
    }

    fn putc_internal(&self, char: u8) {
        //Wait until TX FIFO becomes empty
        while (self.reg_read(LSR)&LSR_TX_VALID) == 0 {
            spinwait();
        }
        //Reset DLAB and write to THR
        // self.reg_write(LCR, self.reg_read(LCR) & !LCR_DLAB);
        self.reg_write(THR, char);
    }

    fn getc_internel(&self) ->Option<u8> {
        //If RX FIFO is empty, return None immediately
        if (self.reg_read(LSR)&LSR_RX_READY) == 0 {
            return None;
        }

        //Reset DLAB and read from RBR
        // self.reg_write(LCR,self.reg_read(LCR) & !LCR_DLAB);
        Some(self.reg_read(RBR) as u8)
    }
}

unsafe impl Sync for Ns16550{}

impl Device for Ns16550 {
    fn name<'a>(&'a self) -> &'a str {
        unsafe {str::from_utf8_unchecked(slice::from_raw_parts(self.name.as_ptr(), self.name_size))}
    }
}

impl UartDevice for Ns16550 {
    fn putc(&self, char: u8) {
        if char =='\n' as u8{
            self.putc_internal('\r' as u8);
        }
        self.putc_internal(char);
    }
    fn getc(&self) ->Option<u8> {
        self.getc_internel().map(|ch|{
            if ch == '\r' as u8 { '\n' as u8 }
            else { ch }
        })
    }
}
