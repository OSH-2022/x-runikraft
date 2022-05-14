#![no_std]

use rkschedbasis::{SchedulerCoop, SchedulerPreem, RKthread, RKthreadAttr, PrioT};
use runikraft::list::{Tailq, TailqPosMut};
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

impl<'b> SchedulerCoop for RKschedpreem<'b> {
    fn yield_sched(&mut self) {
        todo!()
    }
    fn add_thread<'a>(&mut self, t: &'a mut RKthread<'a>, attr: &'a mut RKthreadAttr) -> Result<(), &'static str> {
        todo!()
    }
    fn remove_thread<'a>(&mut self, t_pos: TailqPosMut<RKthread>) -> Result<(), &'static str> {
        todo!()
    }
    fn block_thread<'a>(&mut self, t_pos: TailqPosMut<RKthread>) {
        todo!()
    }
    fn wake_thread<'a>(&mut self, t_pos: TailqPosMut<RKthread>) {
        todo!()
    }
    fn sleep_thread(&self, nsec: Duration) {
        todo!()
    }
    fn exit_thread(&self) {
        todo!()
    }
}

impl<'b> SchedulerPreem for RKschedpreem<'b> {
    fn set_thread_prio<'a>(&mut self, t: &'a mut RKthread<'a>, prio: PrioT) {
        todo!()
    }
    fn get_thread_prio<'a>(&self, t: &'a RKthread<'a>) -> PrioT {
        todo!()
    }
    fn set_thread_timeslice<'a>(&mut self, t: &'a mut RKthread<'a>, tslice: Duration) {
        todo!()
    }
    fn get_thread_timeslice<'a>(&self, t: &'a RKthread<'a>) -> Duration {
        todo!()
    }
}