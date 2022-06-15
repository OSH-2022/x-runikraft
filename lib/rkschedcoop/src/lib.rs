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
use rksched::thread::{Prio,Thread,ThreadData};
use rksched::this_thread;
use runikraft::compat_list::Tailq;
use rkplat::spinlock::SpinLock;
use runikraft::errno::Errno;
use core::ptr::{NonNull,addr_of_mut};
use core::time::Duration;
use rksched::sched::destroy_thread;

type ThreadList = Tailq<ThreadData>;

/// 不可抢占式的FCFS调取器
/// 
/// 三种状态的线程：
/// - 就绪：位于`ready_thread`
/// - 等待：
///     - 等待到达某个时刻：位于waiting_thread
///     - 等待某个事件发生：位于该事件的等待队列
/// - 运行：不位于任何队列
pub struct RKschedcoop {
    threads_started: bool,
    ///就绪的线程
    ready_thread: ThreadList,
    ready_thread_size: usize,
    ///等待的线程
    waiting_thread: ThreadList,
    ///已退出的线程
    exit_thread: ThreadList,
    next: Option<&'static mut dyn RKsched>,
    lock: SpinLock,
}

impl RKschedcoop {
    pub fn new() -> Self {
        Self { threads_started: false, next: None, 
            lock: SpinLock::new(),
            ready_thread_size: 0,
            ready_thread: ThreadList::new(),
            waiting_thread: ThreadList::new(),
            exit_thread: ThreadList::new()}
    }
 
    fn schedule(&mut self) {
        loop {
        let current = rksched::this_thread::control_block();
        if current.is_exited() {
            //未分离的线程由创建者负责清理
            if current.attr.get_detachstate() {
                self.exit_thread.push_back(current.as_non_null());
                //防止一个线程被两次加入exit thread（如果当前线程退出了，但是没有新的能执行的线程，会发生这种情况）
                current.attr.set_detachstate(false);
            }
        }
        //把当前线程放入就绪队列（由线程主动调用yield导致）
        else if current.is_runnable() && current.as_node().is_alone() {
            //如果只有一个调度器，则直接放回自己的就绪队列
            //如果有多个调度器而且self的负载大于next的1.5倍，则将current加入下一个hart的调度器
            if *self.next.as_mut().unwrap() as *mut dyn RKsched == self ||
                self.ready_thread_size*2 < self.next.as_mut().unwrap().__workload()*3 
            {
                let _lock = self.lock.lock();
                self.ready_thread.push_back(current.as_non_null());
                self.ready_thread_size+=1;
            }
            else {
                self.next.as_mut().unwrap().add_thread(current.as_non_null()).unwrap();
            }
        }

        //处理退出队列中的线程
        for t in self.exit_thread.iter() {
            if t.element.as_non_null()==current.as_non_null() {continue;}
            t.remove(Some(&mut self.exit_thread));
            destroy_thread(t);
        }

        //处理就绪队列里的线程
        let current_time = rkplat::time::monotonic_clock();
        let mut sleep_until = current_time + Duration::from_secs(10);
        enum Wrapper {
            Ready(NonNull<Thread>),
            Exit(NonNull<Thread>),
            Nothing,
        }
        use Wrapper::*;
        let mut last_removed = Nothing;
        for t in self.waiting_thread.iter() {
            if let Ready(node) = last_removed {
                let _lock = self.lock.lock();
                self.ready_thread.push_back(node);
                self.ready_thread_size+=1;
            }
            else if let Exit(node) = last_removed {
                self.exit_thread.push_back(node);
            }
            if t.element.wakeup_time<=current_time {
                //remove之后不能立即加入新的队列，因为迭代器需要依靠t原来的指针移到下一个位置
                t.remove(Some(&mut self.waiting_thread));
                if t.element.is_exited() { last_removed = Exit(t.element.as_non_null()); }
                else {t.element.set_runnable(); last_removed = Ready(t.element.as_non_null());}
            }
            else if sleep_until>t.element.wakeup_time{
                sleep_until = t.element.wakeup_time;
            }
        }
        //FIXME: 我原本试图用lambda减少一次代码复制，但是Rust的借用检查使这无法实现
        if let Ready(node) = last_removed {
            let _lock = self.lock.lock();
            self.ready_thread.push_back(node);
            self.ready_thread_size+=1;
        }
        else if let Exit(node) = last_removed {
            self.exit_thread.push_back(node);
        }
        
        if !self.ready_thread.is_empty() {
            let lock = self.lock.lock();
            let mut front = self.ready_thread.pop_front().unwrap();
            self.ready_thread_size-=1;
            drop(lock);
            unsafe{front.as_mut().set_alone();}
            if front != current.as_non_null() {
                unsafe{rksched::thread::thread_switch(current, addr_of_mut!(front.as_mut().element))};
            }
            return;
        }
        
        //TODO: 目前时钟中断的处理有些问题
        while rkplat::time::monotonic_clock()<sleep_until {
            rkplat::lcpu::spinwait();
        }
        // rkplat::lcpu::halt_to(sleep_until);
        }
    }

    /// 原子地把线程从就绪队列中移除，但不清理
    fn remove_thread_ready(&mut self, mut t: NonNull<Thread>) {
        unsafe {
            let lock = self.lock.lock();
            if t.as_ref().next.is_none() && t.as_ref().prev.is_none() {
                return;
            }
            t.as_mut().remove(Some(&mut self.ready_thread));
            self.ready_thread_size-=1;
            drop(lock);
            t.as_mut().set_alone();
        }
    }

    /// 原子地把线程从等待队列中移除，但不清理
    fn remove_thread_waiting(&mut self, mut t: NonNull<Thread>) {
        unsafe {
            let lock = self.lock.lock();
            if t.as_ref().next.is_none() && t.as_ref().prev.is_none() {
                return;
            }
            t.as_mut().remove(Some(&mut self.waiting_thread));
            drop(lock);
            t.as_mut().set_alone();
        }
    }
}

impl RKsched for RKschedcoop {
    fn start(&mut self)->! {
        self.threads_started = true;
        let mut t = self.ready_thread.pop_front().unwrap();
        self.ready_thread_size-=1;
        unsafe {
            rksched::thread::thread_start(&mut t.as_mut().element);
        }
    }

    fn started(&self) -> bool {
        self.threads_started
    }

    fn r#yield(&mut self) {
        self.schedule();
    }

    fn add_thread(&mut self, mut t: NonNull<Thread>) -> Result<(), Errno> {
        let tt;
        unsafe {tt = &mut t.as_mut().element;}
        tt.sched = self;
        tt.attr.set_prio(0).unwrap();
        tt.attr.set_timeslice(Duration::MAX).unwrap();
        tt.set_runnable();
        let _lock = self.lock.lock();
        self.ready_thread.push_back(t);
        self.ready_thread_size+=1;
        Ok(())
    }

    fn remove_thread(&mut self, mut t: NonNull<Thread>) {
        let tt = unsafe {&mut t.as_mut().element};
        if tt.waiting_for.is_none() {
            if tt.is_runnable() {
                self.remove_thread_ready(t);
            }
            else {
                self.remove_thread_waiting(t);
            }
        }
        
        if t==this_thread::control_block().as_non_null() {
            panic!("A thread cannot remove itself. name={} base_addr={:?} id={}",tt.name(),tt.base_addr(),tt.id());
        }
        tt.exit();
        unsafe {destroy_thread(t.as_mut());}
    }

    fn thread_blocked(&mut self, mut t: NonNull<Thread>) {
        self.remove_thread_ready(t);
        let tt = unsafe {&mut t.as_mut().element};
        if tt.waiting_for.is_none() {
            self.waiting_thread.push_back(t);
        }
        else {
            tt.as_node().set_alone();
        }
        if t==this_thread::control_block().as_non_null() {
            self.schedule();
        }
    }

    fn thread_woken(&mut self, mut t: NonNull<Thread>) {
        let tt = unsafe {&mut t.as_mut().element};
        if let Some(waitq) = tt.waiting_for.as_mut() {
            unsafe {
                waitq.as_mut().remove(tt.as_ref());
            }
            tt.waiting_for = None;
        }
        else {
            self.remove_thread_waiting(t);
        }
        self.ready_thread.push_back(t);
        self.ready_thread_size+=1;
    }

    fn set_thread_prio(&mut self, _t: NonNull<Thread>, _prio: Prio) -> Result<(),Errno> {
        Err(Errno::NotSup)//本调度器不支持线程优先级
    }

    fn set_thread_timeslice(&mut self, _t: NonNull<Thread>, _tslice: Duration) -> Result<(),Errno> {
        Err(Errno::NotSup)//本调度器不支持线程时间片
    }

    unsafe fn __set_next_sheduler(&mut self, sched: *const dyn RKsched) {
        debug_assert!(self.next.is_none());
        self.next = Some(&mut *(sched as *mut dyn RKsched));
    }

    fn __workload(&self) -> usize {
        self.ready_thread_size
    }
}
