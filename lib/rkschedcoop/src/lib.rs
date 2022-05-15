#![no_std]

use rkschedbasis::{SchedulerCoop, RKthread, RKthreadAttr, RKthreadList, SchedPrivate};
use runikraft::list::{Tailq, TailqPosMut};
use rkalloc::RKalloc;
use rkplat::lcpu;
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

impl<'a> RKschedcoop<'a> {
    //important schedule function
    pub fn schedcoop_schedule(&mut self) {
        todo!()
    }
}

impl<'b> SchedulerCoop for RKschedcoop<'b> {
    fn yield_sched(&mut self) {
        self.schedcoop_schedule();
    }

    fn add_thread<'a>(&mut self, mut t: RKthread, attr: &'a mut RKthreadAttr) -> Result<(), &'static str> {
        let mut flags: usize = 0;
        let prv = self.prv;
        t.set_runnable();

        flags = lcpu::save_irqf();

        self.prv.sleeping_threads.push_back(t);

        lcpu::restore_irqf(flags);

        Ok(())
    }

    fn remove_thread<'a>(&mut self, t: *mut RKthread) -> Result<(), &'static str> {
        let mut flags: usize = 0;
        let prv = self.prv;

        flags = lcpu::save_irqf();
        unsafe {
            let t_pos = TailqPosMut::from_ptr(t);
            if t != rkschedbasis::thread_current() {
                let mut thread = prv.thread_list.remove(t_pos).0;
                thread.clear_runnable();
                thread.exit();
                self.exited_threads.push_front(thread);
            }
            else {
                let mut thread = prv.thread_list.remove(t_pos).0;
                thread.clear_runnable();
                thread.exit();
                self.exited_threads.push_front(thread);
                self.schedcoop_schedule();
            }
        }


        /* Put onto exited list */
        self.exited_threads.push_back(t);

        lcpu::restore_irqf(flags);

        /* Schedule only if current thread is exiting */
        //TODO: here need judge if t == rk_thread_current(), then
        {
            self.schedcoop_schedule();
            //TODO: here need translate "uk_pr_warn("schedule() returned! Trying again\n")"
        }

        Ok(())
    }

    unsafe fn block_thread<'a>(&mut self, t: &'a mut RKthread<'a>) {

        let prv = self.prv;

        debug_assert!(/*TODO: rkplat_lcpu_irqs_disabled()*/);

        // let mut t = *t_pos;

        //TODO: here need judge if t != rk_thread_current(), then
        let mut t = prv.thread_list.remove(t_pos).0;

        if !t.wakeup_time.is_zero() {
            prv.sleeping_threads.push_front(t);
        }

    }
    unsafe fn wake_thread<'a>(&mut self, t: &'a mut RKthread<'a>) {
        let prv = self.prv;

        debug_assert!(/*TODO: rkplat_lcpu_irqs_disabled()*/);

        //TODO: transform the type [RKthread<'a>] to the type [TailqPosMut<RKthread>]

        if t.wakeup_time > 0 {
            let t = &mut prv.sleeping_threads.remove(t_pos).0;
        }

    }
    fn sleep_thread(&self, nsec: Duration) {
        todo!()
    }
    fn exit_thread(&self) {
        todo!()
    }
}