// SPDX-License-Identifier: BSD-3-Clause
// console.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

//控制台输入输出

use super::sbi::*;

fn putchar_bios(ch: usize) -> bool {
    if let Err(_) = sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0) {
        return false;
    }
    true
}

// fn getchar_bios() -> Result<usize, usize> {
//     sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
// }

#[cfg(feature="driver_uart")]
mod uart_based_io
{
    use crate::drivers::uart::UartDevice;
    pub(crate) static mut UART_DEIVCE: Option<&dyn UartDevice> = None;

    /// 向内核控制台输出字符串
    /// 注意字符串不必是合法的UTF-8，也不会因null终止
    /// - `buf`: 字符串缓冲区
    /// - 返回值: 输出的字符数
    pub fn coutk(buf: &[u8]) -> Option<usize> {
        unsafe {
            UART_DEIVCE.map(|uart|{
                for i in buf {
                    uart.putc(*i);
                }
                buf.len()
            })
        }
    }

    // Unikraft的`coutd`的实现和`coutk`相同，所以Runikraft删去了冗余的API

    /// 从控制台读入字符
    /// - `buf`: 目标缓冲区
    /// - 返回值 读入的字符数
    pub fn cink(buf: &mut [u8]) -> Option<usize> {
        unsafe {
            if let Some(uart) = UART_DEIVCE {
                let mut num = 0;
                    
                while num < buf.len() {
                    if let Some(c) = uart.getc() {
                        buf[num] = c;
                        num+=1;
                    }
                    else {break;}
                }
                
                if num!=0 {Some(num)}
                else {None}
            }
            else {None}
        }
    }
}

#[cfg(not(any(feature="driver_uart")))]
mod bios_io {
    use super::*;
    fn putchar(ch: usize) -> bool {
        if let Err(_) = sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0) {
            return false;
        }
        true
    }
    
    fn getchar() -> Result<usize, usize> {
        sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
    }

    pub fn coutk(buf: &[u8]) -> Option<usize> {
        for i in buf {
            if !putchar(*i as usize) {
                return None;
            }
        }
        Some(buf.len())
    }
    pub fn cink(buf: &mut [u8]) -> Option<usize> {
        let mut cnt: usize = 0;
        for i in buf {
            match getchar() {
                Ok(ch) => {
                    *i = ch as u8;
                    cnt = cnt + 1;
                }
                Err(_) => { return None; }
            }
        }
        Some(cnt)
    }
}

#[cfg(feature="driver_uart")]
pub use uart_based_io::*;

#[cfg(not(any(feature="driver_uart")))]
pub use bios_io::*;

///////////////////
//Rust风格的输出

use core::fmt::{self, Write};

struct RustStyleOutputBIOS;
struct RustStyleOutput;

static LOCK: super::spinlock::SpinLock = super::spinlock::SpinLock::new();

pub(crate) fn __print_bios(args: fmt::Arguments) {
    let flag = super::lcpu::save_irqf();
    let _lock = LOCK.lock();
    RustStyleOutputBIOS.write_fmt(args).unwrap();
    super::lcpu::restore_irqf(flag);
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
    let flag = super::lcpu::save_irqf();
    let _lock = LOCK.lock();
    RustStyleOutput.write_fmt(args).unwrap();
    super::lcpu::restore_irqf(flag);
}

impl Write for RustStyleOutputBIOS {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.as_bytes() {
            putchar_bios(*ch as usize);
        }
        Ok(())
    }
}

impl Write for RustStyleOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let None = coutk(s.as_bytes()) {
            panic!("Attempt to use coutk before initializing uart device. str={}",s);
        }
        Ok(())
    }
}

macro_rules! print_bios {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print_bios(format_args!($fmt $(, $($arg)+)?))
    }
}

macro_rules! println_bios {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print_bios(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print(format_args!(concat!($fmt, "\n")  $(, $($arg)+)?))
    }
}
