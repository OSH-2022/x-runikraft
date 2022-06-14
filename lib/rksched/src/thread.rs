// SPDX-License-Identifier: BSD-3-Clause
// thread.rs
// Authors: Rolf Neugebauer
//          Grzegorz Milos
//          Costin Lupu <costin.lupu@cs.pub.ro>
//          陈建绿 <2512674094@qq.com>
//          张子辰 <zichen350@gmail.com>
// Copyright (c) 2019, NEC Europe Ltd., NEC Corporation.
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.  All rights reserved.

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


// Copyright (c) 2003-2005, Intel Research Cambridge
// Copyright (c) 2017, NEC Europe Ltd., NEC Corporation. All rights reserved.

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// Thread definitions
//  Ported from Mini-OS

extern crate alloc;

use crate::wait::WaitQ;

use super::{sched,wait};
use sched::RKsched;
use rklist::TailqNode;
use runikraft::errno::Errno;
use rkalloc::RKalloc;
use core::mem::size_of;
use core::ops::{Deref, DerefMut};
use core::ptr::{null_mut,addr_of_mut,addr_of,NonNull};
use core::time::Duration;
use core::sync::atomic::{AtomicI32,AtomicU32,Ordering};
use rkplat::thread::Context;
use rkplat::time;
use runikraft::config::STACK_SIZE;
use alloc::string::String;

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
extern "Rust" {
    static _rk_thread_inittab_start: *mut InittabEntry;
    static _rk_thread_inittab_end: *mut InittabEntry;
}

pub type Thread = TailqNode<ThreadData>;

/// 线程的控制块
/// 
/// 线程的生命周期：
/// 1. 分配线程栈空间（stack）和线程本地存储空间（tls）。
///    栈空间必须满足对齐要求STACK_SIZE (默认是65536)。
/// 2. 在栈的低地址调用`init`（`unsafe{*(stack as *mut Thread).init(...)}`，初始化控制块。
/// 3. 用`add_thread`把线程加入调度器。
/// 4. （调度器执行线程）
/// 5. 当线程执行完毕或被kill时，调度器调用`exit`。
/// 6. 调用`finish`。
/// 7. 释放线程栈空间（stack）和线程本地存储空间（tls）。
pub struct ThreadData {
    name: String,
    id: u32,
    stack: *mut u8,
    tls: *mut u8,
    ctx: *mut Context,
    flags: u32,
    pub wakeup_time: Duration,
    pub detached: bool,
    /// 等待self结束的线程
    pub waiting_threads: wait::WaitQ,
    /// self所在的等待队列
    pub waiting_for: Option<NonNull<wait::WaitQ>>,
    pub sched: *mut dyn RKsched,
    entry: unsafe fn(*mut u8)->!,
    arg: *mut u8,
    // prv: *mut u8,
    ref_cnt: AtomicI32,
    pub alloc: *const dyn RKalloc,
    pub attr: ThreadAttr,
    _pinned_marker: core::marker::PhantomPinned,
}

#[allow(unused)]
fn stack_push(sp: &mut usize, value: usize) {
    *sp -= size_of::<usize>();
    unsafe {(*sp as *mut usize).write(value);}
}

#[allow(unused_variables)]
fn init_sp(sp: &mut usize, stack: *mut u8, function: unsafe fn(*mut u8)->!, data: *mut u8) {
    *sp = stack as usize + STACK_SIZE;
    // stack_push(sp, function as usize);
    // stack_push(sp, data as usize);
}

impl ThreadData {
    #[inline(always)]
    fn sched_ref(&self) -> &'static mut dyn RKsched {
        unsafe {&mut *self.sched}
    }
}

static THREAD_ID: AtomicU32 = AtomicU32::new(0);

impl ThreadData {
    // Thread没有new函数，不能用正常方法构造

    ///线程初始化
    pub unsafe fn init(&mut self,
            allocator: &'static dyn RKalloc,
            name:  &str, stack: *mut u8, tls: *mut u8,
            function: unsafe fn(*mut u8)->!, arg: *mut u8) -> Result<(),Errno>{
        assert!(!stack.is_null());
        assert!(!tls.is_null());

        // Save pointer to the thread on the stack to get current thread
        (stack as *mut usize).write(self as *mut ThreadData as usize);

        // Allocate thread context
        let ctx = rkalloc::alloc_type(allocator, Context::default());
        if ctx.is_null() {
            return Err(Errno::NoMem);
        }

        self.ctx = ctx;
        self.name = String::from(name);
        self.id = THREAD_ID.fetch_add(1, Ordering::SeqCst);
        self.stack = stack;
        self.tls = tls;
        self.entry = function;
        self.arg = arg;

        self.flags = 0;
        self.wakeup_time = Duration::ZERO;
        self.detached = false;
        self.attr = ThreadAttr::default();
        addr_of_mut!(self.waiting_threads).write(wait::WaitQ::new(allocator));
        addr_of_mut!(self.sched).write_bytes(0, 1);
        addr_of_mut!(self.waiting_for).write(None);

        let mut itr = _rk_thread_inittab_start;
        while itr != _rk_thread_inittab_end {
            if (*itr).init as usize == 0 {
                continue;
            }

            if let Err(errno) = ((*itr).init)(self) {
                itr = itr.sub(1);
                loop {
                    ((*itr).finish)(self);
                    if itr == _rk_thread_inittab_start {break;}
                    itr = itr.sub(1);
                }
                rkalloc::dealloc_type(allocator, ctx);
                self.ctx = null_mut();
                return Err(errno);
            }
            itr = itr.add(1);
        }

        // Prepare stack and TLS
        // NOTE: In case the function pointer was changed by a thread init
        //       function (e.g., encapsulation), we prepare the stack here
        //       with the final setup
        let mut sp: usize = 0;
        init_sp(&mut sp, stack, self.entry, self.arg);

        //Platform specific context initialization
        //FIXME: ukarch_tls_pointer(tls)
        rkplat::thread::init(self.ctx, sp, self.tls as usize, self.entry, arg);

        addr_of_mut!(self.ref_cnt).write(AtomicI32::new(0));
        self.alloc = allocator;
        Ok(())
    }

    ///线程完成
    pub unsafe fn finish(&mut self) {
        let mut itr = _rk_thread_inittab_start;
        while itr!=_rk_thread_inittab_end {
            if (*itr).finish as usize == 0 {
                continue;
            }
            ((*itr).finish)(self);
            itr = itr.add(1);
        }
        rkalloc::dealloc_type(&*self.alloc, self.ctx);
        self.ctx = null_mut();
    }

    /// 阻塞到特定的时刻
    pub fn block_until(&mut self, until: Duration) {
        assert!(self.is_runnable());
        let flag = rkplat::lcpu::save_irqf();
        self.wakeup_time = until;
        self.clear_runnable();
        self.sched_ref().thread_blocked(self.as_non_null());
        rkplat::lcpu::restore_irqf(flag);
    }

    /// 阻塞一段时间
    pub fn block_timeout(&mut self, duration: Duration) {
        self.block_until(rkplat::time::monotonic_clock()+duration);
    }

    /// 阻塞不确定的时间，它的含义是把线程放入等待队列，但是并不实际等待任何事件
    pub fn block(&mut self) {
        self.block_until(Duration::ZERO);
    }

    /// 等待，直到某个事件发生
    pub fn block_for_event(&mut self, mut event: NonNull<WaitQ>) {
        assert!(self.waiting_for.is_none());
        self.waiting_for = Some(event);
        unsafe {
            let flag = rkplat::lcpu::save_irqf();
            event.as_mut().add(self.as_ref());
            rkplat::lcpu::restore_irqf(flag);
        }
        self.block();
    }

    /// 等待，直到某个线程终止
    pub fn block_for_thread(&mut self, thread: ThreadRef) {
        let event = NonNull::new( addr_of!(thread.waiting_threads) as *mut WaitQ);
        self.block_for_event(event.unwrap());
    }

    pub fn wake(&mut self) {
        let flag = rkplat::lcpu::save_irqf();
        if !self.is_runnable() {
            self.sched_ref().thread_woken(self.as_non_null());
            self.wakeup_time = Duration::ZERO;
            self.set_runnable();
        }
        rkplat::lcpu::restore_irqf(flag);
    }

    pub fn kill(&mut self) {
        self.sched_ref().remove_thread(self.as_non_null());
    }

    pub fn exit(&mut self) {
        self.set_exited();
        if let Some(waitq) = self.waiting_for.as_mut() {
            unsafe {waitq.as_mut().remove(self.as_ref());}
            self.waiting_for = None;
        }
        if !self.detached {
            self.waiting_threads.wakeup_all();
        }
        else {
            debug_assert!(self.waiting_threads.empty());
        }
    }

    pub fn detach(&mut self) {
        assert!(!self.detached);
        self.waiting_threads.wakeup_all();
        self.detached = true;
    }

    pub fn set_prio(&mut self, prio: Prio) -> Result<(), Errno>{
        self.sched_ref().set_thread_prio(self.as_non_null(), prio)
    }

    pub fn get_prio(&self) -> Prio {
        self.attr.prio
    }

    pub fn set_timeslice(&mut self, timeslice: Duration) -> Result<(),Errno> {
        self.sched_ref().set_thread_timeslice(self.as_non_null(), timeslice)
    }

    pub fn get_timeslice(&self) -> Duration {
        self.attr.timeslice
    }
}

impl ThreadData {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn base_addr(&self) -> *mut ThreadData {
        self as *const ThreadData as *mut ThreadData
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn tls(&self) -> *mut u8 {
        self.tls
    }
}

/// 安全性：必须在启动调度器时调用，用来启动一个硬件线程上的第一个内核线程
pub unsafe fn thread_start(init_thread: *mut ThreadData) ->! {
    rkplat::thread::start((*init_thread).ctx);
}

pub unsafe fn thread_switch(prev: *mut ThreadData, next: *mut ThreadData) {
    rkplat::thread::switch((*prev).ctx, (*next).ctx);
}

pub type InitFunc = fn(&mut ThreadData)->Result<(),Errno>;
pub type FinishFunc = fn(&mut ThreadData);
pub struct InittabEntry {
    pub init: InitFunc,
    pub finish: FinishFunc,
}

/// Registers a thread initialization function that is
/// called during thread creation
/// 
/// - `fn`
///   initialization function to be called (uk_thread_init_func_t)
/// - `prio`
///   Priority level (0 (earliest) to 9 (latest))
///   Use the UK_PRIO_AFTER() helper macro for computing priority dependencies.
///   Note: Any other value for level will be ignored
/// 
/// FIXME: Rust的宏不支持标识符拼接，所以使用者必须提供唯一的标识符，比如__rkthread_inittab_<uuid>
#[macro_export]
macro_rules! inittab_entry_prio {
    ($init_fn:ident, $finish_fn:ident, $prio:literal, $unique_ident:ident) => {
        #[no_mangle]
        #[link_section = concat!(".text.thread_inittab.",$prio)]
        static $unique_ident : $crate::thread::InittabEntry =
            $crate::thread::InittabEntry{init: $init_fn, finish: $finish_fn};
    };
}

#[macro_export]
macro_rules! inittab_entry {
    ($init_fn:ident, $finish_fn:ident, $prio:literal, $unique_ident:ident) => {
        #[no_mangle]
        #[link_section = ".text.thread_inittab.9"]
        static $unique_ident : $crate::thread::InittabEntry =
            $crate::thread::InittabEntry{init: $init_fn, finish: $finish_fn};
    };
}

const RUNNABLE_FLAG: u32  = 0x00000001;
const EXITED_FLAG: u32    = 0x00000002;
const QUEUEABLE_FLAG: u32 = 0x00000004;

impl ThreadData {
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

/// 线程的引用，用在等待队列
pub struct ThreadRef {
    ptr: *mut ThreadData,
}

impl Default for ThreadRef {
    fn default() -> Self {
        Self { ptr: null_mut() }
    }
}

impl Deref for ThreadRef {
    type Target = ThreadData;
    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null());
        unsafe {&*(self.ptr)}
    }
}

impl DerefMut for ThreadRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null());
        unsafe {&mut *(self.ptr)}
    }
}

impl Clone for ThreadRef {
    fn clone(&self) -> Self {
        if self.ptr.is_null() {
            Self {ptr: null_mut()}
        }
        else {
            let old_ref = self.ref_cnt.fetch_add(1, Ordering::SeqCst);
            if old_ref == -1 {
                panic!("Attempt to clone ThreadRef when thread is dropping.");
            }
            Self { ptr: self.ptr }
        }
    }
}

impl Drop for ThreadRef {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.ref_cnt.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

impl PartialEq for ThreadRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
    fn ne(&self, other: &Self) -> bool {
        self.ptr != other.ptr
    }
}

impl ThreadData {
    pub fn as_ref(&self) -> ThreadRef {
        let old_ref = self.ref_cnt.fetch_add(1, Ordering::SeqCst);
        if old_ref == -1 {
            panic!("Attempt to create ThreadRef when thread is dropping.");
        }
        ThreadRef { ptr: self as *const ThreadData as *mut ThreadData}
    }

    pub fn as_non_null(&self) -> NonNull<Thread> {
        NonNull::new(self as *const ThreadData as *mut Thread).unwrap()
    }

    pub fn as_node(&self) -> &mut Thread {
        unsafe {self.as_non_null().as_mut()}
    }
}

impl Drop for ThreadData{
    fn drop(&mut self) {
        assert!(self.is_exited());
        debug_assert!(self.waiting_for.is_none());
        debug_assert!(self.waiting_threads.empty());
        let old_ref = self.ref_cnt.swap(-1, Ordering::SeqCst);
        if old_ref != 0 {
            panic!("Attempt to drop thread when it is still referenced.");
        }
        assert!(self.sched.is_null());
    }
}
