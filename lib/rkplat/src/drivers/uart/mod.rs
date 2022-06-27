// SPDX-License-Identifier: BSD-3-Clause
// uart/mod.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::Device;

pub trait UartDevice: Device {
    /// 无缓冲输出
    fn putc(&self, char: u8);
    /// 无阻塞输入
    fn getc(&self)->Option<u8>;
}

#[cfg(feature="driver_ns16550")]
pub mod ns16550;
