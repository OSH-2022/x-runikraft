use super::thread::{RKthread, RKthreadAttr, PrioT, RKthreadList};
use runikraft::list::{TailqPosMut};
use core::time::Duration;
use core::result::Result;

/// Cooperative scheduler trait
/// The non-preemptive (cooperative) scheduler schedules according to Round Robin algorithm.
pub trait SchedulerCoop {
    /// yield scheduler
    fn yield_sched(&mut self);
    /// add thread
    fn add_thread<'a>(&mut self, t: *mut RKthread, attr: &'a mut RKthreadAttr) -> Result<(), &'static str>;
    /// remove thread
    fn remove_thread<'a>(&mut self, t: *mut RKthread) -> Result<(), &'static str>;
    /// block thread
    fn block_thread<'a>(&mut self, t: *mut RKthread);
    /// wake thread
    fn wake_thread<'a>(&mut self, t: *mut RKthread);
    /// let current thread sleep nsec
    fn sleep_thread(&self, nsec: Duration);
    /// let current thread exit
    fn exit_thread(&self);
}
/// Preemptive Scheduler extra(based on SchedulerCoop) trait
pub trait SchedulerPreem {
    /// set thread priority
    fn set_thread_prio<'a>(&mut self, t: *mut RKthread, prio: PrioT);
    /// get thread priority
    fn get_thread_prio<'a>(&self, t: *mut RKthread) -> PrioT;
    /// set thread time slice
    fn set_thread_timeslice<'a>(&mut self, t: *mut RKthread, tslice: Duration);
    /// get thread time slice
    fn get_thread_timeslice<'a>(&self, t: *mut RKthread) -> Duration;
}

pub struct SchedPrivate<'a> {
    pub thread_list: RKthreadList<'a>,
    pub sleeping_threads: RKthreadList<'a>,
}