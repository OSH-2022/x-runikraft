// SPDX-License-Identifier: BSD-3-Clause
// thread.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::{sched,wait};
use sched::RKsched;
use rklist::Tailq;
use runikraft::errno::Errno;
use rkalloc::RKalloc;
use core::mem::size_of;
use core::ptr::null_mut;
use core::time::Duration;
use rkplat::thread::Context;
use rkplat::time;
use rkplat::constants::STACK_SIZE;

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

extern "Rust" {
    static _rk_thread_inittab_start: *mut InittabEntry;
    static _rk_thread_inittab_end: *mut InittabEntry;
}

pub struct Thread {
    pub name: *const str, //Thread和WaitQ需要相互指，所以Thread不能含生命周期注记
    pub stack: *mut u8,
    pub tls: *mut u8,
    pub ctx: *mut Context,
    pub flags: u32,
    pub wakeup_time: Duration,
    pub detached: bool,
    pub waiting_threads: wait::WaitQ,
    pub sched: *mut dyn RKsched,
    pub entry: unsafe fn(*mut u8)->!,
    pub arg: *mut u8,
    pub prv: *mut u8,
}

fn stack_push(sp: &mut usize, value: usize) {
    *sp -= size_of::<usize>();
    unsafe {(*sp as *mut usize).write(value);}
}

fn init_sp(sp: &mut usize, stack: *mut u8, function: unsafe fn(*mut u8)->!, data: *mut u8) {
    *sp = stack as usize + STACK_SIZE;
    stack_push(sp, function as usize);
    stack_push(sp, data as usize);
}

impl Thread {
    #[inline(always)]
    fn sched_ref(&self) -> &'static mut dyn RKsched {
        unsafe {&mut *self.sched}
    }
}

impl Thread {
    // Thread没有new函数，不能用正常方法构造
    // /// 构造Thread对象，返回值并没有被初始化，需要调用init完成初始化
    // pub fn new(alloc: &'static dyn RKalloc) -> Self {
    //     union Helper {
    //         p: *mut dyn RKsched,
    //         s: usize,
    //     }
    //     Self { 
    //         name: "",
    //         stack: null_mut(),
    //         tls: null_mut(),
    //         ctx: null_mut(),
    //         flags: 0,
    //         wakeup_time: Duration::default(),
    //         detached: false,
    //         waiting_threads: wait::WaitQ::new(alloc),
    //         sched: unsafe{Helper{s:0}.p},
    //         entry: null_entry,
    //         arg: null_mut(),
    //         prv: null_mut() }
    // }

    ///线程初始化
    pub unsafe fn init(&mut self,
            allocator: &'static dyn RKalloc,
            name:  &'static str, stack: *mut u8, tls: *mut u8,
            function: unsafe fn(*mut u8)->!, arg: *mut u8) -> Result<(),()>{
        assert!(!stack.is_null());
        assert!(!tls.is_null());

        // Save pointer to the thread on the stack to get current thread
        (stack as *mut usize).write(self as *mut Thread as usize);

        // Allocate thread context
        let ctx = rkalloc::alloc_type(allocator, Context::default());
        if ctx.is_null() {
            return Err(());
        }

        self.ctx = ctx;
        self.name = name;
        self.stack = stack;
        self.tls = tls;
        self.entry = function;
        self.arg = arg;

        self.flags = 0;
        self.wakeup_time = Duration::ZERO;
        self.detached = false;
        core::ptr::addr_of_mut!(self.waiting_threads).write(wait::WaitQ::new(allocator));
        core::ptr::addr_of_mut!(self.sched).write_bytes(0, size_of::<*mut dyn RKsched>());
        self.prv = null_mut();

        let mut itr = _rk_thread_inittab_start;
        while itr != _rk_thread_inittab_end {
            if (*itr).init as usize == 0 {
                continue;
            }

            if !((*itr).init)(self) {
                itr = itr.sub(1);
                loop {
                    ((*itr).finish)(self);
                    if itr == _rk_thread_inittab_start {break;}
                    itr = itr.sub(1);
                }
                rkalloc::dealloc_type(allocator, ctx);
                self.ctx = null_mut();
                return Err(());
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
        Ok(())
    }

    ///线程完成
    pub unsafe fn finish(&mut self, allocator: &dyn RKalloc) {
        let mut itr = _rk_thread_inittab_start;
        while itr!=_rk_thread_inittab_end {
            if (*itr).finish as usize == 0 {
                continue;
            }
            ((*itr).finish)(self);
            itr = itr.add(1);
        }
        rkalloc::dealloc_type(allocator, self.ctx);
        self.ctx = null_mut();
    }

    pub fn block_until(&mut self, until: Duration) {
        let flag = rkplat::lcpu::save_irqf();
        self.wakeup_time = until;
        self.clear_runnable();
        self.sched_ref().thread_blocked(self);
        rkplat::lcpu::restore_irqf(flag);
    }

    pub fn block_timeout(&mut self, duration: Duration) {
        self.block_until(rkplat::time::monotonic_clock()+duration);
    }

    pub fn block(&mut self) {
        self.block_until(Duration::ZERO);
    }

    pub fn wake(&mut self) {
        let flag = rkplat::lcpu::save_irqf();
        if !self.is_runnable() {
            self.sched_ref().thread_woken(self);
            self.wakeup_time = Duration::ZERO;
            self.set_runnable();
        }
        rkplat::lcpu::restore_irqf(flag);
    }

    pub fn kill(&mut self) {
        unsafe {(*self.sched).__thread_kill(self);}
    }

    pub fn exit(&mut self) {
        self.set_exited();
        if !self.detached {
            self.waiting_threads.wake_up();
        }
    }

    pub fn wait(&mut self) -> Result<(),Errno>{
        // TODO critical region
        
        if self.detached {
            return Err(Errno::Inval);
        }

        self.waiting_threads.wait_event(self.is_exited());
        
        self.detached = true;

        unsafe {(*self.sched).__thread_destroy(self); }

        Ok(())
    }

    pub fn detach(&mut self) {
        self.detached = true;
    }

    pub fn set_prio(&mut self, prio: Prio) -> Result<(), Errno>{
        self.sched_ref().set_thread_prio(self, prio)
    }

    pub fn get_prio(&self) -> Result<Prio,Errno> {
        self.sched_ref().get_thread_prio(self)
    }

    pub fn set_timeslice(&mut self, timeslice: Duration) -> Result<(),Errno> {
        self.sched_ref().set_thread_timeslice(self, timeslice)
    }

    pub fn get_timeslice(&self) -> Result<Duration,Errno> {
        self.sched_ref().get_thread_timeslice(self)
    }
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
pub type InitFunc = extern fn(&mut Thread)->bool;
pub type FinishFunc = extern fn(&mut Thread);
pub struct InittabEntry {
    pub init: InitFunc,
    pub finish: FinishFunc,
}

//TODO:
// #define __UK_THREAD_INITTAB_ENTRY(init_fn, fini_fn, prio)		\
// 	static const struct uk_thread_inittab_entry			\
// 	__used __section(".uk_thread_inittab" # prio) __align(8)	\
// 		__uk_thread_inittab ## prio ## _ ## init_fn ## _ ## fini_fn = {\
// 		.init = (init_fn),					\
// 		.fini = (fini_fn)					\
// 	}

// #define _UK_THREAD_INITTAB_ENTRY(init_fn, fini_fn, prio)		\
// 	__UK_THREAD_INITTAB_ENTRY(init_fn, fini_fn, prio)

// #define UK_THREAD_INIT_PRIO(init_fn, fini_fn, prio)			\
// 	_UK_THREAD_INITTAB_ENTRY(init_fn, fini_fn, prio)

// #define UK_THREAD_INIT(init_fn, fini_fn)				\
// 	_UK_THREAD_INITTAB_ENTRY(init_fn, fini_fn, UK_PRIO_LATEST)

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
