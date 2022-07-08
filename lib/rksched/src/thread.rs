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
use sched::Sched;
use runikraft::compat_list::{TailqNode, StailqNode};
use runikraft::errno::Errno;
use rkalloc::Alloc;
use core::ptr::{null_mut,addr_of_mut,NonNull};
use core::time::Duration;
use core::sync::atomic::{AtomicU32,Ordering};
use rkplat::thread::Context;
use rkplat::time;
use alloc::string::String;

////////////////////////////////////////////////////////////////////////
// 线程属性 thread_attr 的结构体定义
////////////////////////////////////////////////////////////////////////

pub const WAITABLE: bool = false;
pub const DETACHED: bool = true;

pub const PRIO_INVALID: i32 = -1;
pub const PRIO_HIGHEST: i32 = 0;
pub const PRIO_LOWEST: i32 = 255;
pub const PRIO_DEFAULT: i32 = 127;
/// 空线程的优先级
/// 
/// 空线程是特殊的线程，一个调度器内有且只有一个空线程，在调度器内有
/// 其他就绪线程时，空线程不得被执行。
/// 空线程的存在是为了使调度器始终能找到一个可执行的线程，它不得终止，必须在死循环内不断调用yield。
pub const PRIO_EMPTY: i32 = i32::MAX;

pub type Prio = i32;

#[derive(Clone,Copy)]
pub struct ThreadAttr {
    detached: bool,
    prio: Prio,
    timeslice: Duration,
    deadline: Duration,
    stack_size: usize,
    tls_size: usize,
    ///线程绑定到特定调度器
    pinned: bool,
}

impl Default for ThreadAttr {
    fn default() -> Self {
        Self {
            detached: WAITABLE,
            prio: PRIO_DEFAULT,
            //FIXME: 太小的时间片会导致无法启动GPU device
            timeslice: Duration::from_millis(500),
            deadline: Duration::MAX,
            stack_size: runikraft::config::rksched::STACK_SIZE,
            tls_size: 0,
            pinned: false,
        }
    }
}

impl ThreadAttr {
    pub fn new(detached: bool,pinned: bool, prio: i32, timeslice: Duration, deadline: Duration, stack_size: usize, tls_size: usize) -> Self {
        Self {
            detached,
            prio,
            pinned,
            timeslice,
            deadline,
            stack_size,
            tls_size,
        }
    }

    pub fn set_detachstate(&mut self, detached: bool) {
        self.detached = detached;
    }

    pub fn get_detachstate(&self) -> bool {
        self.detached
    }

    pub fn set_prio(&mut self, prio: Prio) -> Result<(),Errno>{
        if prio >= PRIO_HIGHEST && prio <= PRIO_LOWEST {
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

    pub fn set_deadline(&mut self, deadline: Duration) -> Result<(),Errno> {
        if deadline > rkplat::time::monotonic_clock() {
            self.deadline = deadline;
            Ok(())
        }
        else {
            Err(Errno::Inval)
        }
    }

    pub fn get_deadline(&self) -> Duration {
        self.deadline
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack_size
    }

    pub fn get_tls_size(&self) -> usize {
        self.tls_size
    }

    pub fn pinned(&self) -> bool {
        self.pinned
    }
}

pub struct ThreadProfile {
    /// 等待时间
    pub time_waiting: Duration,
    /// 运行时间，即占用的CPU时间
    pub time_running: Duration,
}

impl Default for ThreadProfile {
    fn default() -> Self {
        Self { time_waiting: Duration::ZERO, time_running: Duration::ZERO }
    }
}

pub struct ThreadLimit {
    memory_size: usize,
    open_files: usize,
    pipe_size: usize,
    cpu_time: Duration,
}

impl Default for ThreadLimit {
    fn default() -> Self {
        use runikraft::config::rksched::limit::*;
        Self {
            memory_size: MEMORY_SIZE,
            open_files: OPEN_FILES,
            pipe_size: PIPE_SIZE,
            cpu_time: CPU_TIME,
        }
    }
}

impl ThreadLimit {
    pub fn new(memory_size: usize, open_files: usize, pipe_size: usize, cpu_time: Duration) -> Self {
        Self { memory_size, open_files, pipe_size, cpu_time}
    }

    pub fn memory_size(&self) -> usize {
        self.memory_size
    }

    pub fn open_files(&self) -> usize {
        self.open_files
    }

    pub fn pipe_size(&self) -> usize {
        self.pipe_size
    }

    pub fn cpu_time(&self) -> Duration {
        self.cpu_time
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
/// 5. 当线程执行完毕或被kill时，调用`exit`。
/// 6. 切换到其他线程
/// 7. 调度器调用`finish`。
/// 8. 释放线程栈空间（stack）和线程本地存储空间（tls）。
#[repr(align(16))]
pub struct ThreadData {
    name: String,
    id: u32,
    stack: *mut u8,
    tls: *mut u8,
    ctx: *mut Context,
    flags: u32,
    pub wakeup_time: Duration,
    /// 等待self结束的线程
    pub waiting_threads: wait::WaitQ,
    /// self所在的等待队列
    pub waiting_for: Option<NonNull<wait::WaitQ>>,
    pub sched: *mut dyn Sched,
    entry: unsafe fn(*mut u8)->!,
    arg: *mut u8,
    // prv: *mut u8,
    // ref_cnt: AtomicI32,
    pub alloc: *const dyn Alloc,
    pub attr: ThreadAttr,
    pub profile: ThreadProfile,
    pub limit: ThreadLimit,
    _pinned_marker: core::marker::PhantomPinned,
}

impl ThreadData {
    #[inline(always)]
    fn sched_ref(&self) -> &'static mut dyn Sched {
        unsafe {&mut *self.sched}
    }
}

static THREAD_ID: AtomicU32 = AtomicU32::new(0);

impl ThreadData {
    // Thread没有new函数，不能用正常方法构造

    ///线程初始化
    pub unsafe fn init(&mut self,
            allocator: &'static dyn Alloc,
            name:  &str, stack: *mut u8, tls: *mut u8,
            attr: ThreadAttr, limit: ThreadLimit,
            function: unsafe fn(*mut u8)->!, arg: *mut u8) -> Result<(),Errno>{
        assert!(!stack.is_null());
        assert!(!tls.is_null());

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
        addr_of_mut!(self.attr).write(attr);
        addr_of_mut!(self.profile).write(ThreadProfile::default());
        addr_of_mut!(self.limit).write(limit);
        addr_of_mut!(self.waiting_threads).write(wait::WaitQ::new());
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
        let sp: usize = stack as usize + attr.stack_size;

        //Platform specific context initialization
        //FIXME: ukarch_tls_pointer(tls)
        rkplat::thread::init(self.ctx, sp, self.tls as usize, self.entry, arg);

        self.alloc = allocator;
        Ok(())
    }

    ///线程完成，安全性：不得对current_thread调用finish
    pub unsafe fn finish(&mut self) {
        self.waiting_threads.wakeup_final();
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
        //wait_entry被分配在了栈上，这是安全的，因为在线程被唤醒前，thread_blocked不会返回
        let mut wait_entry = StailqNode::new(self.as_non_null());
        unsafe {
            let flag = rkplat::lcpu::save_irqf();
            if !event.as_mut().add(NonNull::new_unchecked(&mut wait_entry)) {
                self.waiting_for=None;
                rkplat::lcpu::restore_irqf(flag);
                return;
            }
            rkplat::lcpu::restore_irqf(flag);
        }
        self.block();
    }

    /// 等待，直到某个线程终止
    pub fn block_for_thread(&mut self, mut thread: NonNull<Thread>) {
        self.block_for_event(unsafe{NonNull::new_unchecked( addr_of_mut!(thread.as_mut().element.waiting_threads))});
    }

    pub fn wake(&mut self) {
        let flag = rkplat::lcpu::save_irqf();
        if !self.is_runnable() {
            self.sched_ref().thread_woken(self.as_non_null());
            self.wakeup_time = Duration::ZERO;
            // debug_assert!(self.is_runnable());
            //self.set_runnable();
        }
        rkplat::lcpu::restore_irqf(flag);
    }

    pub fn kill(&mut self) {
        self.sched_ref().remove_thread(self.as_non_null());
    }

    pub fn exit(&mut self) {
        self.set_exited();
        if let Some(waitq) = self.waiting_for.as_mut() {
            unsafe {waitq.as_mut().remove(self.as_non_null());}
            self.waiting_for = None;
        }
        //此时不能唤醒线程，否则被唤醒的线程可能调用drop(self)，而此时线程所在的调度器还没来得及切换到其他线程
        //finish时才可以安全的唤醒等待的线程
        // if !self.attr.detached {
        //     self.waiting_threads.wakeup_all();
        // }
        // else {
        //     debug_assert!(self.waiting_threads.empty());
        // }
    }

    pub fn detach(&mut self) {
        assert!(!self.attr.detached);
        self.waiting_threads.wakeup_all();
        self.attr.detached = true;
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

impl ThreadData {
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
    }
}
