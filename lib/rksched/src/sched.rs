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
use core::ptr::NonNull;
use runikraft::errno::Errno;
use crate::thread::{ThreadData,ThreadAttr,Prio,Thread};
use rkalloc::RKalloc;
use runikraft::config::{STACK_SIZE,THREAD_LOCAL_SIZE};
use core::sync::atomic::{AtomicU32,Ordering::SeqCst};

// extern "C" {
//     static mut __tls_start: *mut u8;
//     static mut __tls_end: *mut u8;
// }

// pub(crate) fn have_tls_area() -> bool {
//     unsafe {__tls_end.offset_from(__tls_start) != 0 }
// }

/// 调度器的接口
/// 
/// 调度器负责管理线程，使它们一定的顺序运行。每个硬件线程（hart）都拥有
/// 一个调度器，这些调取器以单向循环链表的形式相连。一个内核线程在一个hart上
/// 执行后，这个hart的调度器会把它交给下一个hart的调取器。等待队列独立于调取器，
/// 在线程被放入等待队列时，`Thread::sched`会被指向随机的调取器（随机数的产生方式由
/// 调度器的实现决定）。在等待的事件发送后，等待队列里的线程会被放入`Thread::sched`指向
/// 的调度器。
/// 
/// 在系统启动时，只有一个hart活跃（这里称之为boot hart），而其他hart被挂起。
/// boot hart负责初始化所有的调度器，构建调度器循环链表，启动其他harts，然后调用自己的
/// 调度器的`start`。其他harts的初始化函数最终也会调用`start`。
pub trait RKsched {
    /// 把当前hart的控制权转交给调度器
    fn start(&mut self)->!;
    /// 调度器是否已启动
    fn started(&self) -> bool;
    /// 挂起当前系统线程，要求调度器执行新线程
    fn r#yield(&mut self);
    /// 把线程加入调度器
    fn add_thread(&mut self, t: NonNull<Thread>) -> Result<(), Errno>;
    /// 把线程从调取器中移除
    fn remove_thread(&mut self, t: NonNull<Thread>);
    /// 把线程的状态设置为不可执行
    fn thread_blocked(&mut self, t: NonNull<Thread>);
    /// 把线程的状态设置为可以执行
    fn thread_woken(&mut self, t: NonNull<Thread>);
    /// 设置线程的优先级
    fn set_thread_prio(&mut self, t: NonNull<Thread>, prio: Prio) -> Result<(),Errno>;
    /// 设置线程的时间片
    fn set_thread_timeslice(&mut self, t: NonNull<Thread>, tslice: Duration) -> Result<(),Errno>;

    //内部使用：
    //它们本应该被隐藏，但是Rust不支持protected和friend，所以只能把它们设置成公开接口

    ///**安全性**：只能在初始化调度器的环形链表时使用
    unsafe fn __set_next_sheduler(&mut self, sched: &dyn RKsched);

    /// 调度器的负载程度，一般是就绪队列的大小，用于负载均衡。
    /// 目前，如果self.workload()*2>=next.workload()*3，则将线程加入下一个调取器
    /// TODO: 使这个参数可配置
    fn __workload(&self) -> usize;
}

///TODO: 在合并后应该改成rkplat::LCPU_MAXCOUNT
static mut SCHED_LIST: [Option<NonNull<dyn RKsched>>;16] = [None;16];
static mut SCHED_CNT: usize = 0;
static ADD_NEW_THEAD_TO: AtomicU32 = AtomicU32::new(0);

///注册调度器
pub unsafe fn register(sched: &mut dyn RKsched) -> usize {
    SCHED_LIST[SCHED_CNT] = NonNull::new(sched as *const dyn RKsched as *mut dyn RKsched);
    SCHED_CNT += 1;
    SCHED_CNT-1
}

#[repr(C)]
struct EntryData {
    function: fn(*mut u8),
    arg: *mut u8,
}

unsafe fn thread_entry_point(arg: *mut u8) -> ! {
    let data = arg as *const EntryData;
    ((*data).function)((*data).arg);
    super::this_thread::control_block().exit();
    (*super::this_thread::control_block().sched).r#yield();
    panic!("should exit");
}

/// 创建新线程，并且把它添加到调度器
pub fn create_thread_on_sched(name: &str, alloc: &'static dyn RKalloc,sched_id: usize, attr: ThreadAttr, function: fn(*mut u8), arg: *mut u8) -> Result<(),Errno> {
    unsafe {
        assert!(SCHED_CNT!=0);
        let stack = alloc.alloc(STACK_SIZE, STACK_SIZE);
        if stack.is_null() {
            return Err(Errno::NoMem);
        }
        //栈空间有严格的对齐要求，但线程本地空间没有
        let tls = alloc.alloc(THREAD_LOCAL_SIZE, 16);
        if tls.is_null() {
            alloc.dealloc(stack, STACK_SIZE, STACK_SIZE);
            return Err(Errno::NoMem);
        }

        let thread_addr = stack as *mut ThreadData;
        let entry_data = tls as *mut EntryData;
        (*entry_data).function = function;
        (*entry_data).arg = arg;
        if let Err(errno) = (*thread_addr).init(alloc, name, stack, tls, thread_entry_point, entry_data as *mut u8) {
            alloc.dealloc(stack, STACK_SIZE, STACK_SIZE);
            alloc.dealloc(tls, THREAD_LOCAL_SIZE, 16);
            return Err(errno);
        }
        (*thread_addr).attr = attr;

        if let Err(errno) = SCHED_LIST[sched_id].unwrap().
            as_mut().add_thread((*thread_addr).as_non_null()) {
            alloc.dealloc(stack, STACK_SIZE, STACK_SIZE);
            alloc.dealloc(tls, THREAD_LOCAL_SIZE, 16);
            (*thread_addr).finish();
            return Err(errno);
        }

    }
    Ok(())
}

/// 创建新线程，并且把它添加到调度器
pub fn create_thread(name: &str, alloc: &'static dyn RKalloc, attr: ThreadAttr, function: fn(*mut u8), arg: *mut u8) -> Result<(),Errno> {
    create_thread_on_sched(name, alloc, ADD_NEW_THEAD_TO.fetch_update(SeqCst, SeqCst, 
        |x| {
            Some(if x+1 == unsafe{SCHED_CNT} as u32 {0}
            else {x+1})
        }).unwrap() as usize, attr, function, arg)
}
