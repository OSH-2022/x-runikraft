// SPDX-License-Identifier: BSD-3-Clause
// rtc/mod.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::Device;
use core::time::Duration;

pub trait RtcDevice: Device {
    /// 获取系统时间（通常是UNIX时间）
    fn time(&self) -> Duration;
}

#[cfg(feature="driver_goldfish_rtc")]
pub mod goldfish;
