use core::time::Duration;
use runikraft::errno::Errno;
use crate::thread::{Thread,ThreadAttr,Prio};

/// 调度器 sched 的 trait 定义
pub trait RKsched {
    /// sched start
    fn start(&mut self)->!;
    /// sched started
    fn started(&self) -> bool;
    /// yield scheduler
    fn r#yield(&mut self);
    /// add thread
    fn add_thread(&mut self, t: Thread, attr: ThreadAttr) -> Result<(), Errno>;
    /// remove thread
    fn remove_thread(&mut self, t: *const Thread);
    /// block thread
    fn block_thread(&mut self, t: *const Thread);
    /// wake thread
    fn wake_thread(&mut self, t: *const Thread);
    /// let current thread sleep nsec
    fn sleep_thread(&mut self, duration: Duration);
    /// let current thread exit
    fn exit_thread(&mut self);
    /// set thread priority
    fn set_thread_prio(&mut self, t: *mut Thread, prio: Prio) -> Result<(),Errno> {
        unsafe {
            (*t).set_prio(prio)
        }
    }
    /// get thread priority
    fn get_thread_prio(&self, t: *const Thread) -> Result<Prio,Errno> {
        unsafe {
            (*t).get_prio()
        }
    }
    /// set thread time slice
    fn set_thread_timeslice(&mut self, t: *mut Thread, tslice: Duration) -> Result<(),Errno>{
        unsafe {
            (*t).set_timeslice(tslice)
        }
    }
    /// get thread time slice
    fn get_thread_timeslice(&self, t: *const Thread) -> Result<Duration,Errno> {
        unsafe {
            (*t).get_timeslice()
        }
    }

    //内部使用：

    unsafe fn thread_create(&mut self, name: &str, attr: ThreadAttr, function: fn(*mut u8), arg: *mut u8)-> *const Thread;
    unsafe fn thread_destroy(&mut self,thread: *mut Thread);
    unsafe fn thread_kill(&mut self,thread: *mut Thread);
    unsafe fn thread_switch(&mut self, prev: *mut Thread, next: *mut Thread) {
        rkplat::thread::switch((*prev).ctx, (*next).ctx);
    }
}

/// 针对当前线程的操作
pub mod this_thread {
    use core::time::Duration;

    pub fn r#yield() {
        let current = crate::thread::current();
        unsafe {
            let s=current.sched;
            assert!(!s.is_null());
            (*s).r#yield();
        }
    }
    pub fn sleep_for(duration: Duration) {
        todo!();
    }
    pub fn exit()->! {
        todo!();
    }
}
