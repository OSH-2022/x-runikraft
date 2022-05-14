#![no_std]
use rkschedbasis::{RKthread, RKthreadAttr};
use rkschedcoop::RKschedcoop;
use rkschedpreem::RKschedpreem;
use rkalloc::RKalloc;

/// scheduler kind definition
pub enum SchedKind {
    Coop,   // Cooperative
    Preem,  // Preemptive
}

/// 调度器 sched 的结构体定义
pub struct RKsched<'a> {
    sched_type: SchedKind,
    coop_sched: RKschedcoop<'a>,
    preem_sched: RKschedpreem<'a>,
}

impl<'a> RKsched<'a> {
    /// RKsched API 部分
    /// create a sched
    pub fn sched_create(sched_type: SchedKind, allocator: &'a dyn RKalloc, prv_size: usize) -> Self {
        // TODO
        panic!();
    }
    /// sched start
    pub fn sched_start(&self) {
        // TODO sched.h 238
    }
    /// sched started
    pub fn sched_started(&self) -> bool {
        todo!()
    }

    /// RKsched 非API 部分

    pub fn idle_init(&mut self, stack: *mut u8, function: fn(*mut u8)) {}

    pub fn get_idle(&'a self) -> &'a mut RKthread<'a> {
        let idle: &'a mut RKthread<'a> = unsafe { &mut *(&self.idle as *const RKthread<'a> as *mut RKthread<'a>) };
        idle
    }

    pub fn thread_create(&mut self, name: *const char, attr: &'a mut RKthreadAttr, function: fn(*mut u8), arg: *mut u8) -> *mut RKthread {
        todo!()
    }

    pub fn thread_destroy(&mut self, t: &'a mut RKthread<'a>) {
        // TODO
    }

    pub fn thread_kill(&mut self, t: &'a mut RKthread<'a>) {
        // TODO
    }

    pub fn thread_switch(&mut self, prev: &'a mut RKthread<'a>, next: &'a mut RKthread<'a>) {
        // TODO
    }
}