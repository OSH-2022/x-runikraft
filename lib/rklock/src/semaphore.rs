use core::time::Duration;
use core::sync::atomic::{AtomicI32,Ordering::SeqCst};
use core::ptr::{NonNull,addr_of};

use rksched::wait::WaitQ;

pub struct Semaphore {
    ///大于0时表示剩余资源的数量，小于0时表示等待的线程的数量
    count: AtomicI32,
    wait: WaitQ,
}

unsafe impl Sync for Semaphore{}

impl Semaphore {
    pub const fn new(count: i32) -> Self{
        Self { count: AtomicI32::new(count), wait: WaitQ::new() }
    }
    pub fn wait(&self) {
        if self.count.fetch_sub(1, SeqCst)<=0 {
            let current = rksched::this_thread::control_block();
            current.block_for_event(unsafe{NonNull::new_unchecked(addr_of!(self.wait) as *mut WaitQ)});
        }
    }
    #[must_use]
    pub fn try_wait(&self) -> bool {
        let mut current = self.count.load(SeqCst);
        loop {
            if current<=0 {return false;}
            match self.count.compare_exchange(current, current-1, SeqCst, SeqCst) {
                Ok(_) => return true,
                Err(val) => current = val,
            }
        }
    }
    #[must_use]
    pub fn wait_for(&self, _duration: Duration) -> bool {
        todo!("调度器目前还不支持同时等待多个事件")
    }
    #[must_use]
    pub fn wait_until(&self, _until: Duration) -> bool {
        todo!("调度器目前还不支持同时等待多个事件")
    }
    pub fn signal(&self) {
        if self.count.fetch_add(1,SeqCst)<0 {
            self.wait.waitup_first();
        }
    }

    pub fn count(&self) -> i32 {
        self.count.load(SeqCst)
    }
}
