use core::{sync::atomic::AtomicBool, time::Duration};

use rksched::wait::WaitQ;

pub struct Semaphore {
    count: isize,
    wait: WaitQ,
}

unsafe impl Sync for Semaphore{}

impl Semaphore {
    pub fn new(count: isize) -> Self{
        todo!()
    }
    pub fn wait(&self) {
        todo!()
    }
    #[must_use]
    pub fn try_wait(&self) -> bool {
        todo!()
    }
    #[must_use]
    pub fn wait_for(&self, duration: Duration) -> bool {
        todo!()
    }
    #[must_use]
    pub fn wait_until(&self, until: Duration) -> bool {
        todo!()
    }
    pub fn signal(&self) {
        todo!()
    }

    pub fn count(&self) -> isize {
        self.count
    }
}
