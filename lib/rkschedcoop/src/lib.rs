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

use rkplat::println;
use rksched::RKsched;
use rksched::thread::{Prio,Thread,ThreadData};
use rksched::this_thread;
use runikraft::compat_list::Tailq;
use rkplat::spinlock::SpinLock;
use runikraft::errno::Errno;
use core::ptr::{NonNull,addr_of_mut};
use core::sync::atomic::{AtomicBool,Ordering};
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
    nothing_to_do: AtomicBool,
    ///就绪的线程
    ready_thread: ThreadList,
    ready_thread_size: usize,
    ready_thread_toadd: ThreadList,
    waking_thread_request: [Option<NonNull<Thread>>;16],
    waking_thread_request_size: usize,
    ///等待的线程
    waiting_thread: ThreadList,
    ///已退出的线程
    exit_thread: ThreadList,
    next: Option<&'static mut dyn RKsched>,
    lock: SpinLock,
    lcpuid: rkplat::lcpu::ID,
}

impl RKschedcoop {
    pub fn new(lcpuid: rkplat::lcpu::ID) -> Self {
        rkplat::println!("new({})",lcpuid);
        Self { threads_started: false, next: None, 
            nothing_to_do: AtomicBool::new(false), lcpuid,
            lock: SpinLock::new(),
            ready_thread_toadd: ThreadList::new(),
            ready_thread_size: 0,
            ready_thread: ThreadList::new(),
            waiting_thread: ThreadList::new(),
            waking_thread_request: [None;16],
            waking_thread_request_size: 0,
            exit_thread: ThreadList::new()}
    }
 
    fn schedule(&mut self) {
        debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        //loop {
        let current = rksched::this_thread::control_block();
        if current.is_exited() {
            println!("hart #{}: put current({}) in exit_thread list",self.lcpuid,current.name());
            self.exit_thread.push_back(current.as_non_null());
        }
        //把当前线程放入就绪队列（由线程主动调用yield导致）
        else if current.is_runnable() && current.as_node().is_alone() {
            println!("hart #{}: put current({}) in ready_thread list",self.lcpuid,current.name());
            // //如果只有一个调度器，则直接放回自己的就绪队列
            // 
            // if *self.next.as_mut().unwrap() as *mut dyn RKsched == self ||
            //     self.ready_thread_size*2 <= self.next.as_mut().unwrap().__workload()*3 
            // {
                // let _lock = self.lock.lock();
                self.ready_thread.push_back(current.as_non_null());
                self.ready_thread_size+=1;
            // }
            // else {
            //     self.next.as_mut().unwrap().add_thread(current.as_non_null()).unwrap();
            // }
        }
        else {
            println!("hart #{}: put current({}) in nowhere",self.lcpuid,current.name());
        }

        //处理退出队列中的线程
        for t in self.exit_thread.iter() {
            if t.element.as_non_null()==current.as_non_null() {continue;}
            println!("hart #{}, remove {} in exit_list, current={}",self.lcpuid,t.element.name(),current.name());
            t.remove(Some(&mut self.exit_thread));
            unsafe {t.element.finish();}
            //已分离的线程由调度器负责清理，未分离的线程由创建者负责清理
            if current.attr.get_detachstate() {
                destroy_thread(t);
            }
        }

        //处理异步加入就绪队列里的线程
        {
            let _lock = self.lock.lock();
            while !self.ready_thread_toadd.is_empty() {
                let node = self.ready_thread_toadd.pop_front().unwrap();
                unsafe {
                    debug_assert_eq!(node.as_ref().element.sched,core::ptr::addr_of!(*self) as *const dyn RKsched as *mut dyn RKsched);
                    debug_assert!(node.as_ref().element.is_runnable());
                }
                self.ready_thread.push_back(node);
                self.ready_thread_size+=1;
            }
        }

        //处理异步唤醒的线程
        println!("hart #{}: begin printing waking_thread_request",self.lcpuid);
        {
            let _lock = self.lock.lock();
            for i in 0..self.waking_thread_request_size {
                let mut t = self.waking_thread_request[i].unwrap();
                self.waking_thread_request[i] = None;
                let tt = unsafe {&mut t.as_mut().element};
                println!("hart #{}: {}",self.lcpuid,tt.name());
                if tt.waiting_for.is_some() {
                    tt.waiting_for = None;
                    tt.set_runnable();
                    self.ready_thread.push_back(t);
                    self.ready_thread_size+=1;
                }
                else {
                    tt.wakeup_time = Duration::ZERO;
                }
            }
            self.waking_thread_request_size = 0;
        }
        println!("hart #{}: end printing waking_thread_request",self.lcpuid);

        //处理等待队列里的线程
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
                // let _lock = self.lock.lock();
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
            // let _lock = self.lock.lock();
            self.ready_thread.push_back(node);
            self.ready_thread_size+=1;
        }
        else if let Exit(node) = last_removed {
            self.exit_thread.push_back(node);
        }
        
        #[cfg(debug_assertions)]
        {
            // let _lock = self.lock.lock();
            println!("hart #{}: begin print ready_thread",self.lcpuid);
            for i in self.ready_thread.iter() {
                println!("hart #{}: ready_thread[i]={}",self.lcpuid,i.element.name());
            }
            println!("hart #{}: end print ready_thread",self.lcpuid);
            println!("hart #{}: ready_thread_size={}",self.lcpuid,self.ready_thread_size);
            assert!(!self.ready_thread.is_empty());
            assert!(self.ready_thread_size!=0);
        }
        {
            //TODO
            //简陋的负载均衡
            //如果有多个调度器而且self的负载大于next的1.5倍，则将ready_thread的线程加入下一个hart的调度器
            //下面的实现会引起饥饿
            // if *self.next.as_mut().unwrap() as *mut dyn RKsched != self {
            //     //不可被移动的线程的数量，引入这个变量可以避免死循环
            //     let mut unmovable_cnt = 0;
            //     while self.ready_thread_size>unmovable_cnt+1 && self.ready_thread_size*2 > self.next.as_mut().unwrap().__workload()*3 {
            //         let lock = self.lock.lock();
            //         let front = self.ready_thread.pop_front().unwrap();
            //         if unsafe{front==current.as_non_null() || front.as_ref().element.attr.pinned()} {
            //             self.ready_thread.push_back(front);
            //             unmovable_cnt+=1;
            //             continue;
            //         }
            //         self.ready_thread_size-=1;
            //         drop(lock);
            //         self.next.as_mut().unwrap().add_thread(front).unwrap();
            //     }
            // }

            // let lock = self.lock.lock();
            let mut front = self.ready_thread.pop_front().unwrap();
            //除非别无选择，不调度优先级是empty的线程
            if self.ready_thread_size!=1 && unsafe{front.as_ref().element.attr.get_prio()==rksched::thread::PRIO_EMPTY} {
                self.ready_thread.push_back(front);
                front = self.ready_thread.pop_front().unwrap();
            }
            self.ready_thread_size-=1;
            // drop(lock);
            unsafe{front.as_mut().set_alone();}
            if front != current.as_non_null() {
                rkplat::println!("hart #{}: switch thread, prev={}, next={}",self.lcpuid,current.name(),unsafe{front.as_ref()}.element.name());
                unsafe{rksched::thread::thread_switch(current, addr_of_mut!(front.as_mut().element))};
            }
            else if current.attr.get_prio()==rksched::thread::PRIO_EMPTY {
                //再次检查ready_thread_toadd，防止刚刚加入新的待就绪线程
                let lock = self.lock.lock();
                if self.ready_thread_toadd.is_empty() && self.waking_thread_request_size==0 {
                    self.nothing_to_do.store(true, Ordering::SeqCst);
                    drop(lock);
                    rkplat::lcpu::halt_to(sleep_until);
                    self.nothing_to_do.store(false, Ordering::SeqCst);
                }
            }
            return;
        }
        // self.nothing_to_do.store(true, Ordering::SeqCst);
        // rkplat::lcpu::halt_to(sleep_until);
        // self.nothing_to_do.store(false, Ordering::SeqCst);
        //}
    }

    /// 把线程从就绪队列中移除，但不清理
    fn remove_thread_ready(&mut self, mut t: NonNull<Thread>) {
        unsafe {
            // let lock = self.lock.lock();
            if t.as_ref().next.is_none() && t.as_ref().prev.is_none() {
                return;
            }
            t.as_mut().remove(Some(&mut self.ready_thread));
            self.ready_thread_size-=1;
            // drop(lock);
            t.as_mut().set_alone();
        }
    }

    /// 把线程从等待队列中移除，但不清理
    fn remove_thread_waiting(&mut self, mut t: NonNull<Thread>) {
        unsafe {
            // let lock = self.lock.lock();
            if t.as_ref().next.is_none() && t.as_ref().prev.is_none() {
                return;
            }
            t.as_mut().remove(Some(&mut self.waiting_thread));
            // drop(lock);
            t.as_mut().set_alone();
        }
    }

    fn add_thread_syn(&mut self, mut t: NonNull<Thread>) -> Result<(), Errno> {
        // debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        let tt = unsafe {&mut t.as_mut().element};
        tt.sched = self;
        tt.set_runnable();
        self.ready_thread.push_back(t);
        self.ready_thread_size+=1;
        Ok(())
    }

    fn thread_woken_sync(&mut self, mut t: NonNull<Thread>) {
        // debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        let tt = unsafe {&mut t.as_mut().element};
        println!("hart #{}, id={}: thread_woken({})",self.lcpuid,rkplat::lcpu::id(),tt.name());
        if let Some(_waitq) = tt.waiting_for.as_mut() {
            // unsafe {
            //     waitq.as_mut().remove(tt.as_ref());
            // }
            tt.waiting_for = None;
        }
        else {
            self.remove_thread_waiting(t);
        }
        tt.set_runnable();
        // let lock = self.lock.lock();
        self.ready_thread.push_back(t);
        self.ready_thread_size+=1;
        // drop(lock);
        println!("hart #{}: thread_woken({}) done!",self.lcpuid,tt.name());
    }
}

impl RKsched for RKschedcoop {
    fn start(&mut self)->! {
        debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        self.threads_started = true;
        // let lock = self.lock.lock();
        let mut t = self.ready_thread.pop_front().unwrap();
        self.ready_thread_size-=1;
        // drop(lock);
        unsafe{t.as_mut().set_alone();}
        rkplat::println!("hart #{}: start thread",self.lcpuid);
        unsafe {
            rksched::thread::thread_start(&mut t.as_mut().element);
        }
    }

    fn started(&self) -> bool {
        self.threads_started
    }

    fn r#yield(&mut self) {
        debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        self.schedule();
    }

    fn add_thread(&mut self, mut t: NonNull<Thread>) -> Result<(), Errno> {
        if self.lcpuid==rkplat::lcpu::id() || !self.threads_started {
            self.add_thread_syn(t)
        }
        else {
            unsafe {
                t.as_mut().element.sched = self;
                t.as_mut().element.set_runnable();
            }
            let lock = self.lock.lock();
            self.ready_thread_toadd.push_back(t);
            drop(lock);
            while self.nothing_to_do.load(Ordering::SeqCst) {
                rkplat::lcpu::wakeup(self.lcpuid).unwrap();
                rkplat::lcpu::spinwait();
            }
            // if self.nothing_to_do.swap(false, Ordering::SeqCst) {
            //     rkplat::lcpu::wakeup(self.lcpuid).unwrap();
            // }
            Ok(())
        }
    }

    fn remove_thread(&mut self, mut t: NonNull<Thread>) {
        debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
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
        debug_assert_eq!(self.lcpuid,rkplat::lcpu::id());
        unsafe {
            if !t.as_ref().is_alone() {
                t.as_mut().remove(Some(&mut self.ready_thread));
                self.ready_thread_size-=1;
                t.as_mut().set_alone();
            }
        }
        
        let tt = unsafe {&mut t.as_mut().element};
        debug_assert!(!tt.is_runnable());
        println!("hart #{}: thread_blocked({})",self.lcpuid,tt.name());
        if tt.waiting_for.is_none() {
            println!("hart #{}: waiting_for nothing",self.lcpuid);
            self.waiting_thread.push_back(t);
        }
        else {
            println!("hart #{}: waiting_for sth",self.lcpuid);
            tt.as_node().set_alone();
        }
        if t==this_thread::control_block().as_non_null() {
            self.schedule();
        }
    }

    //这个函数不应该被用户直接调用，而应该由thread::wake()调用
    fn thread_woken(&mut self, t: NonNull<Thread>) {
        println!("thread_woken({}), hart={}, current lcpuid={}",unsafe{t.as_ref().element.name()},self.lcpuid,rkplat::lcpu::id());
        debug_assert!(self.threads_started);
        if self.lcpuid==rkplat::lcpu::id() {
            self.thread_woken_sync(t);
        }
        else {
            //等待唤醒列表中有空位
            let mut lock;
            loop {
                rkplat::lcpu::rmb();
                while self.waking_thread_request_size >= 16 {
                    rkplat::lcpu::spinwait();
                    rkplat::lcpu::rmb();
                }
                lock = self.lock.lock();
                if self.waking_thread_request_size < 16 {
                    break;
                }
                drop(lock);
            }
            self.waking_thread_request[self.waking_thread_request_size] = Some(t);
            self.waking_thread_request_size+=1;
            
            drop(lock);
            while self.nothing_to_do.load(Ordering::SeqCst) {
                rkplat::lcpu::wakeup(self.lcpuid).unwrap();
                rkplat::lcpu::spinwait();
            }
            // if self.nothing_to_do.swap(false, Ordering::SeqCst) {
            //     rkplat::lcpu::wakeup(self.lcpuid).unwrap();
            // }
        }
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
        rkplat::println!("next={:?}",sched);
    }

    fn __workload(&self) -> usize {
        self.ready_thread_size
    }
}
