#![no_std]

use rkschedbasis::{SchedulerCoop, RKthread, RKthreadAttr, RKthreadList, SchedPrivate};
use runikraft::list::Tailq;
use rkalloc::RKalloc;
use core::time::Duration;

pub struct RKschedcoop<'a> {
    threads_started: bool,
    idle: RKthread<'a>,
    exited_threads: RKthreadList<'a>,
    // plat_ctx_cbs: /* plat context callbacks 类型*/
    allocator: &'a dyn RKalloc,
    next: &'a mut RKsched<'a>,
    prv: &'a mut SchedPrivate<'a>,
}

impl<'a> RKschedcoop<'a> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<'b> SchedulerCoop for RKschedcoop<'b> {
    fn yield_sched(&mut self) {
        todo!()
    }
    fn add_thread<'a>(&mut self, t: &'a mut RKthread<'a>, attr: &'a mut RKthreadAttr) {
        let mut flags: usize = 0;
        let &mut prv: SchedPrivate<'a> = self.prv;
        t.set_runnable();
        //flags = rkplat_lcpu_save_irqf();
        

    }
    fn remove_thread<'a>(&mut self, t: &'a mut RKthread<'a>) {
        todo!()
    }
    fn block_thread<'a>(&mut self, t: &'a mut RKthread<'a>) {
        todo!()
    }
    fn wake_thread<'a>(&mut self, t: &'a mut RKthread<'a>) {
        todo!()
    }
    fn sleep_thread(&self, nsec: Duration) {
        todo!()
    }
    fn exit_thread(&self) {
        todo!()
    }
}