// SPDX-License-Identifier: BSD-3-Clause
// rkschedcoop/lib.rs

// Authors: 张子辰 <zichen350@gmail.com>

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

use rksched::RKsched;
use rksched::thread::{ThreadRef,ThreadAttr,Prio};
use rksched::this_thread;
use rkalloc::RKalloc;
use rklist::{Tailq,TailqPosMut};
use rkplat::spinlock::SpinLock;
use runikraft::errno::Errno;
use core::time::Duration;
use runikraft::config::{STACK_SIZE,THREAD_LOCAL_SIZE};

type ThreadList = Tailq<(ThreadRef,ThreadAttr)>;

pub struct RKschedcoop {
    threads_started: bool,
    allocator: &'static dyn RKalloc,
    ///就绪的线程
    ready_thread: ThreadList,
    // ///已退出的线程
    // exited_thread: ThreadList,
    next: Option<&'static mut dyn RKsched>,
    lock: SpinLock,
}

impl RKschedcoop {
    pub fn new(allocator: &'static dyn RKalloc) -> Self {
        Self { threads_started: false, allocator, next: None, 
            lock: SpinLock::new(),
            ready_thread: ThreadList::new(allocator)}
    }
    //important schedule function
    pub fn schedule(&mut self) {
        todo!()
    }
}

impl RKsched for RKschedcoop {
    fn start(&mut self)->! {
        self.threads_started = true;
        loop {
            self.schedule();
        }
    }

    fn started(&self) -> bool {
        self.threads_started
    }

    fn r#yield(&mut self) {
        self.schedule();
    }

    fn add_thread(&mut self, t: ThreadRef, attr: ThreadAttr) -> Result<(), Errno> {
        let _lock = self.lock.lock();
        if self.ready_thread.push_back((t,attr)).is_err() {
            return Err(Errno::NoMem);
        }
        Ok(())
    }

    fn remove_thread(&mut self, t: ThreadRef) {
        let lock = self.lock.lock();
        let mut pos = None;
        for i in self.ready_thread.iter_mut() {
            if i.0==t {
                unsafe{pos = Some(TailqPosMut::from_ref(i));}
                break;
            }
        }
        let (mut t,_) = unsafe {self.ready_thread.remove(pos.unwrap()).0} ;
        drop(lock);

        if t==this_thread::control_block() {
            panic!("A thread cannot remove itself. name={} base_addr={:?} id={}",t.name(),t.base_addr(),t.id());
        }
        t.exit();
        unsafe {
            t.finish();
            let t_addr = t.base_addr();
            let t_alloc = t.alloc;
            let t_tls = t.tls();
            drop(t);
            t_addr.drop_in_place();
            (*t_alloc).dealloc(t_addr as *mut u8, STACK_SIZE, STACK_SIZE);
            (*t_alloc).dealloc(t_tls, THREAD_LOCAL_SIZE, 16);
        }
    }

    fn thread_blocked(&mut self, t: ThreadRef) {
        let i = self.ready_thread.iter_mut().find(|x| x.0 == t).unwrap();
        
    }

    fn thread_woken(&mut self, t: ThreadRef) {
        todo!()
    }

    fn set_thread_prio(&mut self, t: ThreadRef, prio: Prio) -> Result<(),Errno> {
        todo!()
    }

    fn get_thread_prio(&self, t: ThreadRef) -> Result<Prio,Errno> {
        todo!()
    }

    fn set_thread_timeslice(&mut self, t: ThreadRef, tslice: Duration) -> Result<(),Errno> {
        todo!()
    }

    fn get_thread_timeslice(&self, t: ThreadRef) -> Result<Duration,Errno> {
        todo!()
    }

    unsafe fn __set_next_sheduler(&mut self, sched: &dyn RKsched) {
        union Helper<'a> {
            r: &'a dyn RKsched,
            t: *mut dyn RKsched,
        }
        debug_assert!(self.next.is_none());
        self.next = Some(&mut *Helper{r: sched}.t);
    }
}
