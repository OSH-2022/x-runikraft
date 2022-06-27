use core::{ptr::NonNull, time::Duration};

use rksched::{thread::Thread,wait::WaitQ};

/// Mutex that relies on a scheduler
/// uses wait queues for threads
pub struct Mutex {
    lock_count: i32,
    owner: Option<&'static Thread>,
    wait: WaitQ,
}

pub struct MutexGuard {
    owner: NonNull<Mutex>,
}

unsafe impl Sync for Mutex{}

impl Mutex {
    pub fn new() -> Self {
        todo!()
    }
    #[must_use]
    pub fn lock(&self) -> MutexGuard {
        todo!()
    }
    #[must_use]
    pub fn try_lock(&self) -> Option<MutexGuard> {
        todo!()
    }
    #[must_use]
    pub fn lock_for(&self, duration: Duration) -> Option<MutexGuard> {
        todo!()
    }
    #[must_use]
    pub fn lock_until(&self, until: Duration) -> Option<MutexGuard> {
        todo!()
    }
    pub fn is_locked(&self) -> bool {
        todo!()
    }
    fn unlock(&self) {
        todo!()
    }
}

impl MutexGuard {
    pub fn unlock(&self) {
        unsafe{self.owner.as_ref().unlock();}
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        todo!()
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            if self.owner.as_ref().is_locked() {
                self.owner.as_ref().unlock();
            }
        }
    }
}
