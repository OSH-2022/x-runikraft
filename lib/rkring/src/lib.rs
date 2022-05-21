// SPDX-License-Identifier: BSD-3-Clause
// rkring/lib.rs

// Authors: Kip Macy <kmacy@freebsd.org>
//          蓝俊玮 <ljw13@mail.ustc.edu.cn>
//          张子辰 <zichen350@gmail.com>

// Copyright (c) 2007-2009 Kip Macy <kmacy@freebsd.org>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#![no_std]

use rkalloc::*;
use core::slice;
use core::sync::atomic::{AtomicU32, Ordering};
use core::mem::{size_of};

use rkplat::lcpu::{rmb,spinwait};

#[inline(always)]
fn critical_enter() { rkplat::lcpu::barrier(); }
#[inline(always)]
fn critical_exit()  { rkplat::lcpu::barrier(); }

/// 无锁的环形缓冲队列，能被并发读写
pub struct Ring<'a,T> {
    br_prod_head: AtomicU32,
    br_prod_tail: AtomicU32,
    br_prod_size: u32,
    br_prod_mask: u32,
    br_drops: u64,
    br_cons_head: AtomicU32,
    br_cons_tail: AtomicU32,
    //br_cons_size: u32,
    br_cons_mask: u32,
    a: &'a dyn RKalloc,
    br_ring: &'a mut [Option<T>],
}

unsafe impl<T: Copy> Sync for Ring<'_,T>{}

impl<'a,T> Ring<'a,T> {
    /// 为 Ring 分配内存
    /// `count`: 容量
    /// `alloc`: 分配器
    pub fn new(count: usize, a: &'a dyn RKalloc) -> Option<Ring<'a,T>> {
        assert!(count.is_power_of_two());
        let br_ring_data = unsafe{a.alloc_zeroed(count*size_of::<T>(),64) as *mut Option<T>};
        if br_ring_data.is_null() {
            return None;
        }
        let br_ring = unsafe{slice::from_raw_parts_mut(br_ring_data, count)};
        let count = count as u32;
        
        Some(Self{
            br_prod_size: count,
            //br_cons_size: count,
            br_prod_mask: count - 1,
            br_cons_mask: count - 1,
            br_prod_head: AtomicU32::new(0),
            br_cons_head: AtomicU32::new(0),
            br_prod_tail: AtomicU32::new(0),
            br_cons_tail: AtomicU32::new(0),
            br_drops: 0,
            a,
            br_ring
        })
    }

    /// 安全的多生产者环形缓冲队列入队
    pub fn enqueue(&self, buf: T) -> Result<(),i32> {
        let mut_self = unsafe{(self as *const Self as *mut Self).as_mut().unwrap()};
        mut_self.enqueue_mut(buf)
    }

    fn enqueue_mut(&mut self, buf: T) -> Result<(), i32> {
        let mut prod_head: u32;
        let mut prod_next: u32;
        let mut cons_tail: u32;

        critical_enter();
        // critical_enter()
        //__asm__ __volatile__("" : : : "memory")

        loop {
            prod_head = self.br_prod_head.load(Ordering::Relaxed);
            prod_next = (prod_head + 1) & self.br_prod_mask;
            cons_tail = self.br_cons_tail.load(Ordering::Relaxed);
            
            if prod_next == cons_tail {
                rmb();
                if prod_head == self.br_prod_head.load(Ordering::Relaxed) && 
                cons_tail == self.br_cons_tail.load(Ordering::Relaxed) {
                    self.br_drops += 1;
                    critical_exit();
                    return Err(-105);
                }
                continue;
            }
            if let Ok(_)=self.br_prod_head.compare_exchange(prod_head, prod_next, Ordering::SeqCst, Ordering::SeqCst){
                break;
            }
        }

        self.br_ring[prod_head as usize] = Some(buf);
        
        /*
        如果有其他入队操作在进程中更早发生, 需要等待它们完成操作
        */
        while self.br_prod_tail.load(Ordering::Relaxed) != prod_head {
            spinwait();
        }
        self.br_prod_tail.store(prod_next, Ordering::SeqCst);
        critical_exit();
        return Ok(());
    }

    /// 安全的多消费者队列出队
    pub fn dequeue_mc(&self) -> Option<T>{
        let mut_self = unsafe{(self as *const Self as *mut Self).as_mut().unwrap()};
        mut_self.dequeue_mc_mut()
    }
    fn dequeue_mc_mut(&mut self) -> Option<T>{
        let mut cons_head: u32;
        let mut cons_next: u32;

        critical_enter();
        loop {
            cons_head = self.br_cons_head.load(Ordering::Relaxed);
            cons_next = (cons_head + 1) & self.br_cons_mask;
            if cons_head == self.br_prod_tail.load(Ordering::Relaxed) {
                critical_exit();
                return None;
            }
            if let Ok(_)=self.br_cons_head.compare_exchange(cons_head, cons_next, Ordering::SeqCst, Ordering::SeqCst){
                break;
            }
        }
        let buf = self.br_ring[cons_head as usize].take();
        
        /*
        如果有其他入队操作在进程中更早发生, 需要等待它们完成操作
        */
        while self.br_cons_tail.load(Ordering::Relaxed) != cons_head {
            spinwait();
        }
        self.br_cons_tail.store(cons_next, Ordering::SeqCst);
        return buf;
    }

    /// 单消费者队列出队
    /// 
    /// use where dequeue is protected by a lock
    /// e.g. a network driver's tx queue lock
    pub fn dequeue_sc(&self) -> Option<T> {
        let mut_self = unsafe{(self as *const Self as *mut Self).as_mut().unwrap()};
        mut_self.dequeue_sc_mut()
    }
    fn dequeue_sc_mut(&mut self) -> Option<T> {
        
        let cons_head = self.br_cons_head.load(Ordering::Acquire);
        let prod_tail = self.br_prod_tail.load(Ordering::SeqCst);

        let cons_next = (cons_head + 1) & self.br_cons_mask;
        //TODO: prefetch support

        if cons_head == prod_tail {
            return None;
        }

        self.br_cons_head.store(cons_next, Ordering::Relaxed);
        let buf = self.br_ring[cons_head as usize].take();

        self.br_cons_tail.store(cons_next, Ordering::Relaxed);
        return buf;
    }

    /// single-consumer advance after a peek
    /// use where it is protected by a lock
    /// e.g. a network driver's tx queue lock
    pub fn advance_sc(&mut self) {
        let cons_head: u32 = self.br_cons_head.load(Ordering::Relaxed);
        let prod_tail: u32 = self.br_prod_tail.load(Ordering::Relaxed);
        let cons_next: u32 = (cons_head + 1) & self.br_cons_mask;
        

        if cons_head == prod_tail {
            return;
        }
        self.br_cons_head.store(cons_next, Ordering::Relaxed);

        self.br_cons_tail.store(cons_next,Ordering::Relaxed);
    }

    /// Used to return a buffer (most likely already there)
    /// to the top of the ring. The caller should *not*
    /// have used any dequeue to pull it out of the ring
    /// but instead should have used the peek() function.
    /// This is normally used where the transmit queue
    /// of a driver is full, and an mbuf must be returned.
    /// Most likely whats in the ring-buffer is what
    /// is being put back (since it was not removed), but
    /// sometimes the lower transmit function may have
    /// done a pullup or other function that will have
    /// changed it. As an optimization we always put it
    /// back (since jhb says the store is probably cheaper),
    /// if we have to do a multi-queue version we will need
    /// the compare and an atomic.
    pub fn putback_sc(&mut self, new: T) {
        assert_ne!(self.br_cons_head.load(Ordering::Relaxed),self.br_cons_tail.load(Ordering::Relaxed));
        self.br_ring[self.br_cons_head.load(Ordering::Relaxed) as usize] = Some(new);
    }

    /// 在没有修改的情况下返回环形队列的第一个条目的指针
    /// 
    /// 如果环形队列为空时返回空指针
    /// return a pointer to the first entry in the ring
    /// without modifying it, or NULL if the ring is empty
    /// race-prone if not protected by a lock
    pub fn peek(&self) -> Option<T> {
        let mut_self = unsafe{(self as *const Self as *mut Self).as_mut().unwrap()};
        mut_self.peek_mut()
    }
    fn peek_mut(&mut self) -> Option<T> {
        if self.br_cons_head.load(Ordering::Relaxed) == self.br_prod_tail.load(Ordering::Relaxed) {
            None
        } else {
            self.br_ring[self.br_cons_head.load(Ordering::Relaxed) as usize].take()
        }
    }
    
    /// 
    pub fn peek_clear_sc(&self) -> Option<T> {
        let mut_self = unsafe{(self as *const Self as *mut Self).as_mut().unwrap()};
        mut_self.peek_clear_sc_mut()
    }
    fn peek_clear_sc_mut(&mut self) -> Option<T> {
        if self.br_cons_head.load(Ordering::Relaxed) == self.br_prod_tail.load(Ordering::Relaxed) {
            return None;
        }
        return self.br_ring[self.br_cons_head.load(Ordering::Relaxed) as usize].take();
    }

    /// 判断缓冲队列是否为满
    pub fn full(&self) -> bool {
        (self.br_prod_head.load(Ordering::Relaxed) + 1) & self.br_prod_mask == self.br_cons_tail.load(Ordering::Relaxed)
    }

    /// 判断缓冲队列是否为空
    pub fn empty(&self) -> bool {
        self.br_cons_head.load(Ordering::Relaxed) == self.br_prod_tail.load(Ordering::Relaxed)
    }

    pub fn count(&self) -> usize {
        ((self.br_prod_size + self.br_prod_tail.load(Ordering::Relaxed) - self.br_cons_tail.load(Ordering::Relaxed)) & self.br_prod_mask) as usize
    }
}

impl<T> Drop for Ring<'_,T> {
    fn drop(&mut self) {
        unsafe{
            self.a.dealloc(self.br_ring.as_mut_ptr() as *mut u8, self.br_ring.len()*size_of::<T>(), 64);
        }
    }
}
