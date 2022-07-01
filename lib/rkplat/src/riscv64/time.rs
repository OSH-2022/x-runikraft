// SPDX-License-Identifier: BSD-3-Clause
// time.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::{lcpu,sbi};
use core::time::Duration;
#[cfg(feature="driver_rtc")]
use crate::drivers::rtc::RtcDevice;

#[cfg(feature="driver_rtc")]
pub(crate) static mut RTC_DEVICE: Option<&dyn RtcDevice> = None;

/// 1秒
pub const SEC: Duration = Duration::new(1, 0);
/// 1纳秒
pub const NSEC: Duration = Duration::new(0, 1);

use core::{arch, ptr::null_mut};

//1tick的长度
pub const TICK_NANOSEC: u64 = 100;

//初始化时的time寄存器的值
static mut INIT_TIME: u64 = 0;

fn timer_irq_handler(_: *mut u8) -> bool {
    super::irq::ack_irq(0x5);
    true
}

/// 初始化时钟和时钟中断
pub fn init() {
    unsafe {
        INIT_TIME = get_time_counter();
        super::irq::register(get_irq(), timer_irq_handler, null_mut()).unwrap();
    }
}

//获取时钟中断号
pub const fn get_irq() -> usize {
    0x5
}

fn get_time_counter() -> u64 {
    let time: u64;
    unsafe {
        arch::asm!("rdtime {r}",
        r=out(reg)time);
    }
    time
}

/// CPU内部的计时器的值
pub fn get_ticks() -> Duration {
    Duration::from_nanos(TICK_NANOSEC * get_time_counter())
}

/// 获取自时钟初始化以来的时间
pub fn monotonic_clock() -> Duration {
    Duration::from_nanos(unsafe { TICK_NANOSEC * (get_time_counter() - INIT_TIME) })
}

/// 获取UNIX时间
pub fn wall_clock() -> Duration {
    #[cfg(feature="driver_rtc")]
    {
        unsafe{RTC_DEVICE.unwrap().time()}
    }
    #[cfg(not(feature="driver_rtc"))]
    {
        unimplemented!("Feature `driver_rtc` is disabled.");
    }
}

pub(crate) fn block(until: Duration) {
    assert!(lcpu::irqs_disabled());
    let time_now = monotonic_clock();
    if until <= time_now {return;}
    //Set Timer
    set_timer(until);
    lcpu::halt_irq();
}

/// 暂停当前处理器核，直到`until`时刻 
pub fn sleep_until(until: Duration) {
    let flag = lcpu::save_irqf();
    loop {
        block(until);
        if monotonic_clock() >= until {break;}
    }
    lcpu::restore_irqf(flag);
}

/// 在until时刻触发时钟中断
pub fn set_timer(until: Duration) {
    sbi::sbi_call(0x54494D45, 0, ((until.as_nanos() as u64 + unsafe{INIT_TIME})/TICK_NANOSEC) as usize, 0, 0).unwrap();
}
