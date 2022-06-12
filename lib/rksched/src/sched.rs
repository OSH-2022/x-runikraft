// SPDX-License-Identifier: BSD-3-Clause
// sched.rs
// Authors: Costin Lupu <costin.lupu@cs.pub.ro>
//          陈建绿 <2512674094@qq.com>
//          张子辰 <zichen350@gmail.com>
// Copyright (c) 2017, NEC Europe Ltd., NEC Corporation.
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿. All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// 
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

use core::time::Duration;
use runikraft::errno::Errno;
use crate::thread::{Thread,ThreadAttr,Prio};

// extern "C" {
//     static mut __tls_start: *mut u8;
//     static mut __tls_end: *mut u8;
// }

// pub(crate) fn have_tls_area() -> bool {
//     unsafe {__tls_end.offset_from(__tls_start) != 0 }
// }

/// 调度器 sched 的 trait 定义
pub trait RKsched {
    /// sched start
    fn start(&mut self)->!;
    /// sched started
    fn started(&self) -> bool;
    /// yield scheduler
    fn r#yield(&mut self);
    /// add thread
    fn add_thread(&mut self, t: Thread, attr: ThreadAttr) -> Result<(), Errno>;
    /// remove thread
    fn remove_thread(&mut self, t: *const Thread);
    /// block thread
    fn thread_blocked(&mut self, t: *const Thread);
    /// wake thread
    fn thread_woken(&mut self, t: *const Thread);
    /// set thread priority
    fn set_thread_prio(&mut self, t: *mut Thread, prio: Prio) -> Result<(),Errno>;
    /// get thread priority
    fn get_thread_prio(&self, t: *const Thread) -> Result<Prio,Errno>;
    /// set thread time slice
    fn set_thread_timeslice(&mut self, t: *mut Thread, tslice: Duration) -> Result<(),Errno>;
    /// get thread time slice
    fn get_thread_timeslice(&self, t: *const Thread) -> Result<Duration,Errno>;

    //内部使用：

    unsafe fn __thread_create(&mut self, name: &str, attr: ThreadAttr, function: fn(*mut u8), arg: *mut u8)-> *const Thread;
    unsafe fn __thread_destroy(&mut self,thread: *mut Thread);
    unsafe fn __thread_kill(&mut self,thread: *mut Thread);
    unsafe fn __thread_switch(&mut self, prev: *mut Thread, next: *mut Thread) {
        rkplat::thread::switch((*prev).ctx, (*next).ctx);
    }
}
