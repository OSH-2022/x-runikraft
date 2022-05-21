#![no_std]

use rksched::{RKsched, RKschedInternelFun, RKthread, RKthreadAttr, PrioT};
use rklist::{Tailq, TailqPosMut};
use rkalloc::RKalloc;
use core::time::Duration;

pub struct RKschedpreem<'a> {
    threads_started: bool,
    idle: RKthread<'a>,
    exited_threads: Tailq<'a, RKthread<'a>>,
    // plat_ctx_cbs: /* plat context callbacks 类型*/
    allocator: &'a dyn RKalloc,
    next: &'a mut RKschedpreem<'a>,
    prv: *mut u8,
}

impl<'a> RKschedpreem<'a> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<'a> RKsched<'a> for RKschedpreem<'a> {
    fn add_thread(&mut self, t: RKthread<'a>, attr: RKthreadAttr) -> Result<(), &'static str> {
        todo!()
    }
    fn sched_started(&self) -> bool {
        todo!()
    }
    fn sched_start(&self) {
        todo!()
    }
    fn yield_sched(&mut self) {
        todo!()
    }
    fn block_thread(&mut self, t: *mut RKthread<'a>) {
        todo!()
    }
    fn exit_thread(&mut self) {
        todo!()
    }
    fn remove_thread(&mut self, t: *mut RKthread<'a>) -> Result<(), &'static str> {
        todo!()
    }
    fn sleep_thread(&mut self, nsec: Duration) {
        todo!()
    }
    fn wake_thread(&mut self, t: *mut RKthread<'a>) {
        todo!()
    }
}

impl<'a> RKschedInternelFun for RKschedpreem<'a> {
    fn thread_switch(&mut self, prev: *mut RKthread, next: *mut RKthread) {
        todo!()
    }
    fn thread_kill(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_destroy(&mut self, t: *mut RKthread) {
        todo!()
    }
    fn thread_create(&mut self, name: *const char, attr: &mut RKthreadAttr, function: fn(*mut u8), arg: *mut u8) -> *mut RKthread {
        todo!()
    }
    fn idle_init(&mut self, stack: *mut u8, function: fn(*mut u8)) {
        todo!()
    }
    fn get_idle(&self) -> *mut RKthread {
        todo!()
    }
}
