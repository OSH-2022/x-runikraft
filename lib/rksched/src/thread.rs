// SPDX-License-Identifier: BSD-3-Clause
// thread.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::{sched,wait};
use sched::RKsched;
use wait::RKwaitQ;
use rklist::Tailq;
use runikraft::errno::Errno;
use rkalloc::RKalloc;
use core::time::Duration;
use rkplat::thread::Context;
use rkplat::time;

////////////////////////////////////////////////////////////////////////
// 线程属性 thread_attr 的结构体定义
////////////////////////////////////////////////////////////////////////

pub const WAITABLE: bool = false;
pub const DETACHED: bool = true;

pub const PRIO_INVALID: i32 = -1;
pub const PRIO_MIN: i32 = 0;
pub const PRIO_MAX: i32 = 255;
pub const PRIO_DEFAULT: i32 = 127;

pub const TIMESLICE_NIL: Duration = Duration::ZERO;

//优先级类型 prio_t 为 i32
pub type Prio = i32;

#[derive(Clone,Copy)]
pub struct ThreadAttr {
    //True if thread should detach
    detached: bool,
    //Priority
    prio: Prio,
    //Time slice in nanoseconds
    timeslice: Duration,
}

impl Default for ThreadAttr {
    fn default() -> Self {
        Self {
            detached: WAITABLE,
            prio: PRIO_INVALID,
            timeslice: TIMESLICE_NIL,
        }
    }
}

impl ThreadAttr {
    pub fn new(detached: bool, prio: i32, timeslice: Duration) -> Self {
        Self {
            detached,
            prio,
            timeslice,
        }
    }

    pub fn set_detachstate(&mut self, detached: bool) {
        self.detached = detached;
    }

    pub fn get_detachstate(&self) -> bool {
        self.detached
    }

    pub fn set_prio(&mut self, prio: Prio) -> Result<(),Errno>{
        if self.prio >= PRIO_MIN && self.prio <= PRIO_MAX {
            self.prio = prio;
            Ok(())
        }
        else {
            Err(Errno::Inval)
        }
    }

    pub fn get_prio(&self) -> Prio {
        self.prio
    }

    pub fn set_timeslice(&mut self, timeslice: Duration) -> Result<(),Errno>{
        if timeslice.as_nanos() as u64 >= time::TICK_NANOSEC {
            self.timeslice = timeslice;
            Ok(())
        }
        else {
            Err(Errno::Inval)
        }
    }

    pub fn get_timeslice(&self) -> Duration {
        self.timeslice
    }
}

////////////////////////////////////////////////////////////////////////
// 线程 thread 的结构体定义
////////////////////////////////////////////////////////////////////////
pub type ThreadList = Tailq<Thread>;

pub struct Thread {
    pub name: *const str, //Thread和WaitQ需要相互指，所以Thread不能含生命周期注记
    pub stack: usize,
    pub tls: usize,
    pub ctx: *mut Context,
    pub flags: u32,
    pub wakeup_time: Duration,
    pub detached: bool,
    pub waiting_threads: RKwaitQ,
    pub sched: *mut dyn RKsched,
    pub entry: fn(*mut u8),
    pub arg: *mut u8,
    pub prv: *mut u8,
}

impl Thread {
    pub fn new(name: &str, function: fn(*mut u8), data: *mut u8) -> Self {
        todo!();
    }

    pub fn kill(&mut self) {
        todo!();
    }

    pub fn exit(&mut self) {
        todo!();
    }

    pub fn wait(&mut self) {
        todo!();
    }

    pub fn detached(&mut self) {
        todo!();
    }

    pub fn set_prio(&mut self, prio: Prio) -> Result<(),Errno> {
        todo!();
    }

    pub fn get_prio(&self) -> Result<Prio,Errno> {
        todo!();
    }

    pub fn set_timeslice(&mut self, timeslice: Duration) -> Result<(),Errno> {
        todo!();
    }

    pub fn get_timeslice(&self) -> Result<Duration,Errno> {
        todo!();
    }

    //线程初始化
    pub unsafe fn init(&mut self, /*cbs: *mut plat_ctx_callbacks, */allocator: &dyn RKalloc,
                   name:  &'static str, stack: *mut u8, tls: *const char, entry: fn(*mut u8), arg: *mut u8) {
        // TODO
    }
    //线程完成
    pub unsafe fn finish(&mut self, allocator: &dyn RKalloc) {
        // TODO
    }

    pub fn block(&mut self) {
        // TODO
    }

    pub fn block_timeout(&self, nsec: Duration) {
        // TODO
    }

    pub fn wake(&mut self) {
        // TODO
    }

    //后面还有一些仿函数宏未完成(thread.h 145~167)
}


///返回当前线程的控制块
pub fn current() -> &'static mut Thread {
    todo!()//needs the function about stack(operations related to the bottom layer)
}

const RUNNABLE_FLAG: u32  = 0x00000001;
const EXITED_FLAG: u32    = 0x00000002;
const QUEUEABLE_FLAG: u32 = 0x00000004;

impl Thread {
    pub fn is_runnable(&self) -> bool {
        self.flags & RUNNABLE_FLAG !=0
    }
    pub fn set_runnable(&mut self) {
        self.flags |= RUNNABLE_FLAG;
    }
    pub fn clear_runnable(&mut self) {
        self.flags &= !RUNNABLE_FLAG;
    }

    pub fn is_exited(&self) -> bool {
        self.flags & EXITED_FLAG !=0
    }
    pub fn set_exited(&mut self) {
        self.flags |= EXITED_FLAG;
    }

    pub fn is_queueable(&self) -> bool {
        self.flags & QUEUEABLE_FLAG !=0
    }
    pub fn set_queueable(&mut self) {
        self.flags |= QUEUEABLE_FLAG;
    }
    pub fn clear_queueable(&mut self) {
        self.flags &= !QUEUEABLE_FLAG;
    }
}
