// SPDX-License-Identifier: BSD-3-Clause
// rkschedpreem/lib.rs

// Authors: 陈建绿 <2512674094@qq.com>

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

use rksched::{RKsched, RKschedInternelFun, RKthread, RKthreadAttr, Prio};
use runikraft::compat_list::{Tailq, TailqPosMut};
use rkalloc::RKalloc;
use core::time::Duration;

pub struct RKschedpreem<'a> {
    threads_started: bool,
    idle: RKthread<'a>,
    exited_threads: Tailq<'a, RKthread<'a>>,
    // plat_ctx_cbs: /* plat context callbacks 类型*/
    allocator: &'a dyn RKalloc,
    next: &'a mut RKschedpreem<'a>,
    prv: *mut u8,
}

impl<'a> RKschedpreem<'a> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<'a> RKsched<'a> for RKschedpreem<'a> {
    fn add_thread(&mut self, t: RKthread<'a>, attr: RKthreadAttr) -> Result<(), &'static str> {
        todo!()
    }
    fn sched_started(&self) -> bool {
        todo!()
    }
    fn sched_start(&self) {
        todo!()
    }
    fn yield_sched(&mut self) {
        todo!()
    }
    fn block_thread(&mut self, t: *mut RKthread<'a>) {
        todo!()
    }
    fn exit_thread(&mut self) {
        todo!()
    }
    fn remove_thread(&mut self, t: *mut RKthread<'a>) -> Result<(), &'static str> {
        todo!()
    }
    fn sleep_thread(&mut self, nsec: Duration) {
        todo!()
    }
    fn wake_thread(&mut self, t: *mut RKthread<'a>) {
        todo!()
    }
}

impl<'a> RKschedInternelFun for RKschedpreem<'a> {
    fn thread_switch(&mut self, prev: *mut RKthread, next: *mut RKthread) {
        todo!()
    }
    fn thread_kill(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_destroy(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_create(&mut self, name: *const char, attr: &mut RKthreadAttr, function: fn(*mut u8), arg: *mut u8) -> *mut RKthread {
        todo!()
    }
    fn idle_init(&mut self, stack: *mut u8, function: fn(*mut u8)) {
        todo!()
    }
    fn get_idle(&self) -> *mut RKthread {
        todo!()
    }
}
