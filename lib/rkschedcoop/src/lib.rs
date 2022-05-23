// SPDX-License-Identifier: BSD-3-Clause
// rkschedcoop/lib.rs

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

use rksched::{RKsched, RKschedInternelFun, RKthread, RKthreadAttr, RKthreadList, SchedPrivate};
use rklist::{TailqPosMut};
use rkalloc::RKalloc;
use rkplat::{lcpu, thread};
use core::time::Duration;

pub struct RKschedcoop<'a> {
    threads_started: bool,
    idle: RKthread<'a>,
    exited_threads: RKthreadList<'a>,
    allocator: &'a dyn RKalloc,
    next: *mut RKschedcoop<'a>,
    prv: &'a mut SchedPrivate<'a>,
}

impl<'a> RKschedcoop<'a> {
    pub fn new() -> Self {
        todo!()
    }
    //important schedule function
    pub fn schedcoop_schedule(&mut self) {
        todo!()
    }
}

impl<'a> RKsched<'a> for RKschedcoop<'a> {
    fn sched_start(&self) {
        unsafe {
            thread::start(self.idle.ctx);
        }
    }
    fn sched_started(&self) -> bool {
        self.threads_started
    }
    fn yield_sched(&mut self) {
        self.schedcoop_schedule();
    }
    fn add_thread(&mut self, mut t: RKthread<'a>, attr: RKthreadAttr) -> Result<(), &'static str> {
        let mut flags: usize = 0;
        t.set_runnable();

        flags = lcpu::save_irqf();

        self.prv.sleeping_threads.push_back(t);

        lcpu::restore_irqf(flags);

        Ok(())
    }
    fn remove_thread(&mut self, t: *mut RKthread<'a>) -> Result<(), &'static str> {
        let mut flags: usize = 0;

        flags = lcpu::save_irqf();
        unsafe {
            let t_pos = TailqPosMut::from_ptr(t);
            if t != rksched::thread_current() {
                //Remove from the thread list
                let mut thread = self.prv.thread_list.remove(t_pos).0;
                thread.clear_runnable();
                thread.exit();
                // Put onto exited list
                self.exited_threads.push_front(thread);
                lcpu::restore_irqf(flags);
            }
            else {
                //Remove from the thread list
                let mut thread = self.prv.thread_list.remove(t_pos).0;
                thread.clear_runnable();
                thread.exit();
                // Put onto exited list
                self.exited_threads.push_front(thread);
                lcpu::restore_irqf(flags);
                // Schedule only if current thread is exiting
                self.schedcoop_schedule();
                //TODO：here need translate `rk_pr_warn("schedule() returned! Trying again\n");`
            }
        }
        Ok(())
    }
    fn block_thread(&mut self, t: *mut RKthread<'a>) {
        debug_assert!(lcpu::irqs_disabled());

        unsafe {
            let t_pos = TailqPosMut::from_ptr(t);
            if t != rksched::thread_current() {
                let mut thread = self.prv.thread_list.remove(t_pos).0;
                if !thread.wakeup_time.is_zero() {
                    self.prv.sleeping_threads.push_back(thread);
                }
            }
        }
    }
    fn wake_thread(&mut self, t: *mut RKthread<'a>) {
        debug_assert!(lcpu::irqs_disabled());

        unsafe {
            let t_pos = TailqPosMut::from_ptr(t);
            if !(*t).wakeup_time.is_zero() {
                let mut thread = self.prv.sleeping_threads.remove(t_pos).0;
                if t != rksched::thread_current() || thread.is_queueable() {
                    thread.clear_queueable();
                    self.prv.thread_list.push_back(thread);
                }
            }
        }
    }
    fn sleep_thread(&mut self, nsec: Duration) {
        let t = rksched::thread_current();
        unsafe {
            (*t).block_timeout(nsec);
        }
        self.yield_sched();
    }
    fn exit_thread(&mut self) {
        let t = rksched::thread_current();
        self.remove_thread(t);
        //TODO: need to translate `RK_CRASH("Failed to stop the thread\n");`
    }
}

impl<'a> RKschedInternelFun for RKschedcoop<'a> {
    fn get_idle(&self) -> *mut RKthread {
        todo!()
    }
    fn idle_init(&mut self, stack: *mut u8, function: fn(*mut u8)) {
        todo!()
    }
    fn thread_create(&mut self, name: *const char, attr: &mut RKthreadAttr, function: fn(*mut u8), arg: *mut u8) -> *mut RKthread {
        todo!()
    }
    fn thread_destroy(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_kill(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_switch(&mut self, prev: *mut RKthread, next: *mut RKthread) {
        todo!()
    }
}
