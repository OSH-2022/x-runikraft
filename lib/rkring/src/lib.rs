// 能被并发读写，内部可变性
// TODO: 传入所有权, 返回所有权
#![no_std]

use rkalloc::*;
use core::sync::atomic::{AtomicU32, Ordering};
use core::ptr::{null_mut, drop_in_place};
use core::mem::size_of;

#[inline(always)]
pub fn critical_enter() { rkplat::lcpu::barrier(); }
#[inline(always)]
pub fn critical_exit()  { rkplat::lcpu::barrier(); }
#[inline(always)]
pub fn rmb() { rkplat::lcpu::rmb(); }
#[inline(always)]
pub fn mb() { rkplat::lcpu::mb(); }
#[inline(always)]
pub fn wmb() {rkplat::lcpu::wmb(); }
#[inline(always)]
pub fn spinwait() { rkplat::lcpu::spinwait(); }


#[repr(align(64))]
struct CacheLineAligned<T> {
    data: T,
}

pub struct Ring {
    br_prod_head: u32,
    br_prod_tail: u32,
    br_prod_size: u32,
    br_prod_mask: u32,
    br_drops: u64,
    br_cons_head: CacheLineAligned<u32>,
    br_cons_tail: u32,
    br_cons_size: u32,
    br_cons_mask: u32,
    br_ring: CacheLineAligned<[*mut u8; 64]>,
}

impl Ring {
    /// `count`: 容量
    /// `alloc`: 分配器
    pub fn new(count: i32, a: &dyn RKalloc) -> *mut Ring {
        let br: *mut Ring;
        unsafe {
            br = a.alloc(size_of::<Ring>(), count as usize * size_of::<*mut u8>()) as *mut Ring;
        }
        if br == null_mut() {
            return null_mut();
        }
        unsafe {
            (*br).br_prod_size = count as u32;
            (*br).br_cons_size = count as u32;
            (*br).br_prod_mask = (count - 1) as u32;
            (*br).br_cons_mask = (count - 1) as u32;
            (*br).br_prod_head = 0;
            (*br).br_cons_head.data = 0;
            (*br).br_prod_tail = 0;
            (*br).br_cons_tail = 0;
        }
        return br;
    }
    pub fn enqueue(&mut self, buf: *mut u8) -> Result<(), i32> {
        let mut prod_head: u32;
        let mut prod_next: u32;
        let mut cons_tail: u32;

        if cfg!(DEBUG_BUFRING) {
            let mut i: u32 = self.br_cons_head.data;
            loop {
                if i == self.br_prod_head {
                    break;
                }
                if self.br_ring.data[i as usize] == buf {
                    panic!("buf already enqueue at {} prod={} cons={}", i, self.br_prod_tail, self.br_cons_tail);
                    // UK_CRASH("buf=%p already enqueue at %d prod=%d cons=%d",
					//buf, i, br->br_prod_tail, br->br_cons_tail);
                }
                i = (i + 1) & self.br_cons_mask;
            }
        }
        critical_enter();
        // critical_enter()
        //__asm__ __volatile__("" : : : "memory")
        loop {
            prod_head = self.br_prod_head;
            prod_next = (prod_head + 1) & self.br_prod_mask;
            cons_tail = self.br_cons_tail;
            
            if prod_next == cons_tail {
                rmb();
                //rmb()
                //__asm__ __volatile__ ("lfence" : : : "memory")
                if prod_head == self.br_prod_head && cons_tail == self.br_cons_tail {
                    self.br_drops = self.br_drops + 1;
                    critical_exit();
                    //critical_exit()
                    //__asm__ __volatile__("" : : : "memory")
                    return Err(-105);
                }
                continue;
            }

            match AtomicU32::new(self.br_prod_head).compare_exchange(prod_head, prod_next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(success) => success,
                Err(_) => { break },
            };
        }

        if cfg!(DEBUG_BUFRING) {
            if self.br_ring.data[prod_head as usize] != null_mut() {
                panic!("dangling value in enqueue");
                //UK_CRASH("dangling value in enqueue");
            }
        }

        self.br_ring.data[prod_head as usize] = buf;
        loop {
            if self.br_prod_tail != prod_head {
                spinwait();
                //ukarch_spinwait();
                //__asm__ __volatile__("pause" : : : "memory");         //lcpu.rs
            }
            else {
                break;
            }
        }
        AtomicU32::new(self.br_prod_tail).store(prod_next, Ordering::SeqCst);
        critical_exit();
        //critical_exit()
        //__asm__ __volatile__("" : : : "memory")
        return Ok(());
    }
    pub fn dequeue_mc(&mut self) -> Option<*mut u8>{
        let mut cons_head: u32;
        let mut cons_next: u32;
        let buf: *mut u8;

        critical_enter();
        // critical_enter()
        //__asm__ __volatile__("" : : : "memory")
        loop {
            cons_head = self.br_cons_head.data;
            cons_next = (cons_head + 1) & self.br_cons_mask;
            if cons_head == self.br_prod_tail {
                critical_exit();
                // critical_exit()
                return None;
            }
            match AtomicU32::new(self.br_cons_head.data).compare_exchange(cons_head, cons_next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(success) => success,
                Err(_) => { break },
            };
        }
        buf = self.br_ring.data[cons_head as usize];

        if cfg!(DEBUG_BUFRING) {
            self.br_ring.data[cons_head as usize] = null_mut();
        }

        loop {
            if self.br_cons_tail != cons_next {
                spinwait();
                //ukarch_spinwait();
                //__asm__ __volatile__("pause" : : : "memory");         //lcpu.rs
            }
            else {
                break;
            }
        }
        AtomicU32::new(self.br_cons_tail).store(cons_next, Ordering::SeqCst);
        critical_exit();
        //critical_exit()
        //__asm__ __volatile__("" : : : "memory")
        return Some(buf);
    }
    pub fn dequeue_sc(&mut self) -> Option<*mut u8> {
        let cons_head: u32;
        let cons_next: u32;
        let prod_tail: u32;
        let buf: *mut u8;
        if cfg!(CONFIG_ARCH_ARM_32) || cfg!(CONFIG_ARCH_ARM_64) {
            cons_head = AtomicU32::new(self.br_cons_head.data).load(Ordering::SeqCst);
        } 
        else {
            cons_head = self.br_cons_head.data;
        }

        prod_tail = AtomicU32::new(self.br_prod_tail).load(Ordering::SeqCst);
        cons_next = (cons_head + 1) & self.br_cons_mask;
        if cons_head == prod_tail {
            return None;
        }

        //if cfg!(PREFETCH_DEFINED) {
        //    let cons_next_next: u32 = (cons_head + 2) & self.br_cons_mask;
        //    if cons_next != prod_tail {
        //        
        //    }
        //}

        self.br_cons_head.data = cons_next;
        buf = self.br_ring.data[cons_head as usize];

        if cfg!(DEBUG_BUFRING) {
            self.br_ring.data[cons_head as usize] = null_mut();
            //if (!uk_mutex_is_locked(br->br_lock))
		        //UK_CRASH("lock not held on single consumer dequeue: %d", br->br_lock->lock_count);
            if self.br_cons_tail != cons_head {
                panic!("inconsistent list cons_tail={} cons_head={}", self.br_cons_tail, cons_head);
                //UK_CRASH("inconsistent list cons_tail=%d cons_head=%d",
				//br->br_cons_tail, cons_head);
            }
        }

        self.br_cons_tail = cons_next;
        return Some(buf);
    }
    pub fn advance_sc(&mut self) {
        let cons_head: u32 = self.br_cons_head.data;
        let cons_next: u32 = (self.br_cons_head.data + 1) & self.br_cons_mask;
        let prod_tail: u32 = self.br_prod_tail;

        if cons_head == prod_tail {
            return;
        }
        self.br_cons_head.data = cons_next;

        if cfg!(DEBUG_BUFRING) {
            self.br_ring.data[cons_head as usize] = null_mut();
        }

        self.br_cons_tail = cons_next;
    }
    pub fn putback_sc(&mut self, new: *mut u8) {
        self.br_ring.data[self.br_cons_head.data as usize] = new;
    }
    pub fn peek(&self) -> Option<*mut u8> {
        if cfg!(DEBUG_BUFRING) {
            //if (!uk_mutex_is_locked(br->br_lock))
		    //UK_CRASH("lock not held on single consumer dequeue");
        }

        if self.br_cons_head.data == self.br_prod_tail {
            None
        } else {
            Some(self.br_ring.data[self.br_cons_head.data as usize])
        }
    }
    pub fn peek_clear_sc(&mut self) -> Option<*mut u8> {
        if cfg!(DEBUG_BUFRING) {
            //if (!uk_mutex_is_locked(br->br_lock))
		    //UK_CRASH("lock not held on single consumer dequeue");
        }

        if self.br_cons_head.data == self.br_prod_tail {
            return None
        }

        if cfg!(CONFIG_ARCH_ARM_32) || cfg!(CONFIG_ARCH_ARM_64) {
            panic!("unsupported: atomic_thread_fence_acq()");
        }

        if cfg!(DEBUG_BUFRING) {
            let ret: *mut u8 = self.br_ring.data[self.br_cons_head.data as usize];
            self.br_ring.data[self.br_cons_head.data as usize] = null_mut();
            return Some(ret);
        }

        return Some(self.br_ring.data[self.br_cons_head.data as usize])
    }
    pub fn full(&self) -> bool {
        (self.br_prod_head + 1) & self.br_prod_mask == self.br_cons_tail
    }
    pub fn empty(&self) -> bool {
        self.br_cons_head.data == self.br_prod_tail
    }
    pub fn count(&self) -> i32 {
        ((self.br_prod_size + self.br_prod_tail - self.br_cons_tail) & self.br_prod_mask) as i32
    }
}

impl Drop for Ring {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self);
        }
    }
}
