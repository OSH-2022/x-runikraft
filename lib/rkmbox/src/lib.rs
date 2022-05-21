#![no_std]

use rkplat::{lcpu, println};
use core::ptr::{null_mut};
use core::mem::{size_of, align_of};
use rkalloc::*;

pub struct Mbox {
    pub len: u128,
    // readsem: Semaphore,
    pub readpos: i64,
    // writesem: Semaphore,
    pub writepos: i64,
    pub msgs: [*mut u8; 128],
}

impl Mbox {
    pub fn new( size: u128, a: &dyn RKalloc) -> *mut Mbox {
        let m: *mut Mbox;
        assert!(size <= u128::MAX);

        unsafe {
            m = a.alloc(size_of::<Mbox>() + size_of::<*mut u8>() * (size + 1) as usize, align_of::<Mbox>()) as *mut Mbox;
        }

        if m == null_mut() {
            return null_mut();
        }

        unsafe {
            (*m).len = size + 1;

            // uk_semaphore_init(&m->readsem, 0);
            (*m).readpos = 0;
    
            // uk_semaphore_init(&m->writesem, (long) size);
            (*m).writepos = 0;
        }
        
        println!("Created mailbox {:?}\n", m);
        return m;
    }

    pub fn do_mbox_recv(&mut self) -> Option<*mut u8> {
        let irqf: usize;
        let ret: *mut u8;

        // println!("Receive message from mailbox {:?}\n", self);
        irqf = lcpu::save_irqf();
        assert!(self.readpos != self.writepos);
        ret = self.msgs[self.readpos as usize];
        self.readpos = (self.readpos + 1) % self.len as i64;
        lcpu::restore_irqf(irqf);

        // uk_semaphore_up(&m->writesem);

        return Some(ret);
    }

    pub fn do_mbox_post(&mut self, msg: *mut u8) {
        let irqf: usize;

        irqf = lcpu::save_irqf();
        self.msgs[self.writepos as usize] = msg;
        self.writepos = (self.writepos + 1) % self.len as i64;
        assert!(self.readpos != self.writepos);
        lcpu::restore_irqf(irqf);
        // println!("Posted message {} to mailbox {}\n", msg, self);

        // uk_semaphore_up(&m->readsem);
    }

    pub fn mbox_post(&mut self, msg: *mut u8) {
        // uk_semaphore_down(&m->writesem);
        self.do_mbox_post(msg);
    }

    pub fn mbox_post_try(&mut self, msg: *mut u8) -> i32 {
        // if (!uk_semaphore_down_try(&m->writesem))
        // return -ENOBUFS;

        self.do_mbox_post(msg);
        return 0;
    }

    pub fn mbox_post_to(&mut self, msg: *mut u8, timeout: u64) -> u64 {
        let ret: u64 = 0;
        // ret = uk_semaphore_down_to(&m->writesem, timeout);

        if ret != u64::MAX {
            self.do_mbox_post(msg);
        }

        return ret;
    }

    pub fn mbox_recv(&mut self, msg: *mut *mut u8) {
        let rmsg: *mut u8;

        // uk_semaphore_down(&m->readsem);
        rmsg = match self.do_mbox_recv() {
            None => null_mut(),
            Some(x) => x,
        };
        
        unsafe {
            if msg != null_mut() {
                *msg = rmsg;
            }
        }
    }

    pub fn mbox_recv_try(&mut self, msg: *mut *mut u8) -> i32 {
        let rmsg: *mut u8;
        // if (!uk_semaphore_down_try(&m->readsem))
        // return -ENOBUFS;

        rmsg = match self.do_mbox_recv() {
            None => null_mut(),
            Some(x) => x,
        };

        unsafe {
            if msg != null_mut() {
                *msg = rmsg;
            }
        }
        
        return 0;
    }

    pub fn mbox_recv_to(&mut self, msg: *mut *mut u8, timeout: u64) -> u64 {
        let mut rmsg: *mut u8 = null_mut();
        let ret: u64 = 0;

        // ret = uk_semaphore_down_to(&m->readsem, timeout);
        if ret != u64::MAX {
            rmsg = match self.do_mbox_recv() {
                None => null_mut(),
                Some(x) => x,
            };
        }

        unsafe {
            if msg != null_mut() {
                *msg = rmsg;
            }
        }
        
        return ret;
    }
}

impl Drop for Mbox {
    fn drop(&mut self) {
        unsafe {
            // 
        }
    }
}