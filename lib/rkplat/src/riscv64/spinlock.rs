#[cfg(feature = "has_smp")]
mod inner {
    use core::arch;
    use core::ptr::addr_of;

    /// 自旋锁
    pub struct SpinLock {
        lock: i32,
    }

    impl SpinLock {
        pub fn new() -> SpinLock {
            SpinLock { lock: 0 }
        }
        /// 上锁
        pub fn lock(&self) {
            unsafe {
                arch::asm!(
                r#" 1:  lw t0, ({lock})   #读当前的值
                        beqz t0, 3f   #如果没有被上锁，就尝试上锁
                    2:  .insn i 0x0F,0,x0,x0,0x010  #arch::pause的实现
                        j 1b          #重试
                    3:  li t0, 1
                        amoswap.w.aq t1, t0, ({lock})   #尝试获取锁
                        bnez t1, 2b   #获取失败
                "#,
                    lock = in(reg) addr_of!(self.lock),
                );
            }
        }
        /// 尝试上锁
        pub fn try_lock(&self) -> bool {
            let mut ret = 0;
            unsafe {
                arch::asm!(
                r#" 1:  lw t0, ({lock})   #读当前的值
                        bnez t0, 3f   #已结被上锁，上锁失败
                        li t0, 1
                        amoswap.w.aq t1, t0, ({lock})   #尝试获取锁
                        bnez t1, 3f   #获取失败
                        li {ret}, 1
                    3:  
                "#,
                    lock = in(reg) addr_of!(self.lock),
                    ret = inout(reg) ret
                );
            }
            ret != 0
        }
        /// 解锁
        pub fn unlock(&self) {
            assert!(self.lock != 0);
            unsafe {
                arch::asm!(
                " amoswap.w.rl zero,zero,({lock})",
                lock = in(reg) addr_of!(self.lock))
            }
        }
        /// 已上锁时返回true
        pub fn is_locked(&self) -> bool {
            unsafe { arch::asm!("fence w,r"); }
            self.lock != 0
        }
    }

    unsafe impl Sync for SpinLock{}

    impl Drop for SpinLock {
        fn drop(&mut self) {
            self.unlock();
        }
    }
}

#[cfg(not(feature = "has_smp"))]
mod inner {
    pub struct SpinLock {} //空

    impl SpinLock {
        pub fn new() -> SpinLock {
            SpinLock {}
        }
        #[inline(always)]
        pub fn lock(&self) {}
        #[inline(always)]
        pub fn trylock(&self) -> bool { true }
        #[inline(always)]
        pub fn unlock(&self) {}
        #[inline(always)]
        pub fn is_locked(&self) -> bool { false }
    }
}

pub struct SpinLockGuard<'a> {
    lock: &'a inner::SpinLock,
}

pub struct SpinLock {
    lock: inner::SpinLock,
}

impl SpinLock {
    pub fn new() -> SpinLock {
        SpinLock { lock: inner::SpinLock::new() }
    }

    pub fn lock<'a>(&'a self) -> SpinLockGuard<'a> {
        self.lock.lock();
        SpinLockGuard { lock: &self.lock }
    }

    pub fn try_lock<'a>(&'a self) -> Option<SpinLockGuard<'a>> {
        if self.lock.try_lock() {
            Some(SpinLockGuard{lock: &self.lock})
        }
        else {
            None
        }
    }
}

impl SpinLockGuard<'_> {
    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }
}

impl Drop for SpinLockGuard<'_> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}
