use core::time::Duration;
use core::ptr::{NonNull,addr_of};
use core::sync::atomic::{AtomicU32,Ordering::SeqCst,Ordering::Relaxed};
use rksched::{thread::Thread,wait::WaitQ};
use core::cell::UnsafeCell;

/// Mutex that relies on a scheduler
/// uses wait queues for threads
pub struct Mutex {
    lock_count: AtomicU32,
    owner: UnsafeCell<Option<NonNull<Thread>>>,
    wait: WaitQ,
}

pub struct MutexGuard {
    owner: NonNull<Mutex>,
}

unsafe impl Sync for Mutex{}

impl Mutex {
    pub const fn new() -> Self {
        Self {lock_count: AtomicU32::new(0), owner: UnsafeCell::new(None), wait: WaitQ::new()}
    }
    #[must_use]
    pub fn lock(&self) -> MutexGuard {
        let current = rksched::this_thread::control_block();
        loop {
            if let Some(guard) = self.try_lock() {
                return guard;
            }
            current.block_for_event(unsafe{NonNull::new_unchecked(addr_of!(self.wait) as *mut WaitQ)});
        }
    }
    #[must_use]
    pub fn try_lock(&self) -> Option<MutexGuard> {
        let current = rksched::this_thread::control_block();
        if self.lock_count.compare_exchange(0, 1, SeqCst, SeqCst).is_ok() {
            unsafe{*self.owner.get() = Some(current.as_non_null());}
            return Some(MutexGuard::new(self));
        }
        else if let Some(owner) = unsafe{*self.owner.get()} {
            if current.as_non_null() == owner {
                self.lock_count.fetch_add(1, SeqCst);
                return Some(MutexGuard::new(self));
            }
        }
        return None;
    }
    #[must_use]
    pub fn lock_for(&self, _duration: Duration) -> Option<MutexGuard> {
        todo!("调度器目前还不支持同时等待多个事件")
    }
    #[must_use]
    pub fn lock_until(&self, _until: Duration) -> Option<MutexGuard> {
        todo!("调度器目前还不支持同时等待多个事件")
    }
    pub fn is_locked(&self) -> bool {
        unsafe{(*self.owner.get()).is_some()}
    }
    fn unlock(&self) {
        assert_eq!(unsafe{*self.owner.get()},Some(rksched::this_thread::control_block().as_non_null()));
        if self.lock_count.fetch_sub(1, Relaxed)==1 {
            unsafe {*self.owner.get() = None;}
            self.wait.waitup_first();
        }
    }
}

impl MutexGuard {
    fn new(mutex: &Mutex) -> Self {
        unsafe{return Self{owner: NonNull::new_unchecked(mutex as *const Mutex as *mut Mutex)};}
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        if self.owner.get_mut().is_some() {
            panic!("Attempt to drop Mutex when it is still locked.");
        }
        self.wait.wakeup_final();
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            self.owner.as_ref().unlock();
        }
    }
}
