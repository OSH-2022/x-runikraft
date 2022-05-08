use super::thread::{RKthread, RKthreadAttr, PrioT};
use core::time::Duration;

/// Cooperative scheduler trait
/// The non-preemptive (cooperative) scheduler schedules according to Round Robin algorithm.
pub trait SchedulerCoop {
    /// yield scheduler
    fn yield_sched(&mut self);
    /// add thread
    fn add_thread<'a>(&mut self, t: &'a mut RKthread<'a>, attr: &'a mut RKthreadAttr);
    /// remove thread
    fn remove_thread<'a>(&mut self, t: &'a mut RKthread<'a>);
    /// block thread
    fn block_thread<'a>(&mut self, t: &'a mut RKthread<'a>);
    /// wake thread
    fn wake_thread<'a>(&mut self, t: &'a mut RKthread<'a>);
    /// let current thread sleep nsec
    fn sleep_thread(&self, nsec: Duration);
    /// let current thread exit
    fn exit_thread(&self);
}
/// Preemptive Scheduler extra(based on SchedulerCoop) trait
pub trait SchedulerPreem {
    /// set thread priority
    fn set_thread_prio<'a>(&mut self, t: &'a mut RKthread<'a>, prio: PrioT);
    /// get thread priority
    fn get_thread_prio<'a>(&self, t: &'a RKthread<'a>) -> PrioT;
    /// set thread time slice
    fn set_thread_timeslice<'a>(&mut self, t: &'a mut RKthread<'a>, tslice: Duration);
    /// get thread time slice
    fn get_thread_timeslice<'a>(&self, t: &'a RKthread<'a>) -> Duration;
}