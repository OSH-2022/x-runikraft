use core::time::Duration;
use runikraft::errno::Errno;
use crate::thread::{Thread,ThreadAttr,Prio};

// extern "C" {
//     static mut __tls_start: *mut u8;
//     static mut __tls_end: *mut u8;
// }

// pub(crate) fn have_tls_area() -> bool {
//     unsafe {__tls_end.offset_from(__tls_start) != 0 }
// }

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
    fn thread_blocked(&mut self, t: *const Thread);
    /// wake thread
    fn thread_woken(&mut self, t: *const Thread);
    /// let current thread sleep nsec
    fn sleep_thread(&mut self, duration: Duration);
    /// let current thread exit
    fn exit_thread(&mut self);
    /// set thread priority
    fn set_thread_prio(&mut self, t: *mut Thread, prio: Prio) -> Result<(),Errno>;
    /// get thread priority
    fn get_thread_prio(&self, t: *const Thread) -> Result<Prio,Errno>;
    /// set thread time slice
    fn set_thread_timeslice(&mut self, t: *mut Thread, tslice: Duration) -> Result<(),Errno>;
    /// get thread time slice
    fn get_thread_timeslice(&self, t: *const Thread) -> Result<Duration,Errno>;

    //内部使用：

    unsafe fn __thread_create(&mut self, name: &str, attr: ThreadAttr, function: fn(*mut u8), arg: *mut u8)-> *const Thread;
    unsafe fn __thread_destroy(&mut self,thread: *mut Thread);
    unsafe fn __thread_kill(&mut self,thread: *mut Thread);
    unsafe fn __thread_switch(&mut self, prev: *mut Thread, next: *mut Thread) {
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
