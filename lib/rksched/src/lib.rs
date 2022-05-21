#![no_std]

mod thread;
mod wait;

pub use thread::*;
pub use wait::*;
use core::time::Duration;

/// 调度器 sched 的 trait 定义
pub trait RKsched<'a> {
    /// sched start
    fn sched_start(&self);// TODO sched.h 238
    /// sched started
    fn sched_started(&self) -> bool;
    /// yield scheduler
    fn yield_sched(&mut self);
    /// add thread
    fn add_thread(&mut self, t: RKthread<'a>, attr: RKthreadAttr) -> Result<(), &'static str>;
    /// remove thread
    fn remove_thread(&mut self, t: *mut RKthread<'a>) -> Result<(), &'static str>;
    /// block thread
    fn block_thread(&mut self, t: *mut RKthread<'a>);
    /// wake thread
    fn wake_thread(&mut self, t: *mut RKthread<'a>);
    /// let current thread sleep nsec
    fn sleep_thread(&mut self, nsec: Duration);
    /// let current thread exit
    fn exit_thread(&mut self);
    /// set thread priority
    fn set_thread_prio(&mut self, t: *mut RKthread, prio: PrioT) {
        unsafe {
            (*t).set_prio(prio);
        }
    }
    /// get thread priority
    fn get_thread_prio(&self, t: *const RKthread) -> PrioT {
        unsafe {
            (*t).get_prio()
        }
    }
    /// set thread time slice
    fn set_thread_timeslice(&mut self, t: *mut RKthread, tslice: Duration) {
        unsafe {
            (*t).set_timeslice(tslice);
        }
    }
    /// get thread time slice
    fn get_thread_timeslice(&self, t: *const RKthread) -> Duration {
        unsafe {
            (*t).get_timeslice()
        }
    }
}

/// internel functions trait
pub trait RKschedInternelFun {
    /// RKsched 非API 部分
    fn idle_init(&mut self, stack: *mut u8, function: fn(*mut u8));

    fn get_idle(&self) -> *mut RKthread;

    fn thread_create(&mut self, name: *const char, attr: &mut RKthreadAttr, function: fn(*mut u8), arg: *mut u8) -> *mut RKthread;

    fn thread_destroy(&mut self, t: *mut RKthread);

    fn thread_kill(&mut self, t: *mut RKthread);

    fn thread_switch(&mut self, prev: *mut RKthread, next: *mut RKthread);
}


pub struct SchedPrivate<'a> {
    pub thread_list: RKthreadList<'a>,
    pub sleeping_threads: RKthreadList<'a>,
}
