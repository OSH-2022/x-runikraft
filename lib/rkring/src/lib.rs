// 能被并发读写，内部可变性
// TODO: 传入所有权, 返回所有权
#![no_std]

use rkalloc::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Ring {
    pub br_prod_head: u32,
    pub br_prod_tail: u32,
    pub br_prod_size: u32,
    pub br_prod_mask: u32,
    pub br_drops: u64,
    pub br_cons_head: u32,
    pub br_cons_tail: u32,
    pub br_cons_size: u32,
    pub br_cons_mask: u32,
    pub br_ring: Vec<*mut u8>,
}

impl Ring {
    /// `count`: 容量
    /// `alloc`: 分配器
    pub fn new(count: i32,a: &dyn RKalloc)->Ring{
        Ring{}
    }
    pub fn enqueue(&self,buf: *mut u8) -> Result<(),i32> {
        Err(-1)
    }
    pub fn dequeue_mc(&self) -> Option<*mut u8>{
        None
    }
    pub fn dequeue_sc(&mut self) -> Option<*mut u8>{
        let cons_head: u32;
        let cons_next: u32;
        let prod_tail: u32;
        let buf: *mut u8;
        cons_head = self.br_cons_head;
        prod_tail = AtomicU32::new(self.br_prod_tail).load(Ordering::SeqCst);
        cons_next = (cons_head + 1) & self.br_cons_mask as u32;
        if cons_head == prod_tail {
            return None
        }
        self.br_cons_head = cons_next;
        buf = self.br_ring[cons_head as usize];
        self.br_cons_tail = cons_next;
        return Some(buf)
    }
    pub fn advance_sc(&mut self){
        let cons_head = self.br_cons_head;
        let cons_next = (self.br_cons_head + 1)  & self.br_cons_mask as u32;
        let prod_tail = self.br_prod_tail;
        if cons_head == prod_tail {
            return;
        }
        self.br_cons_head = cons_next;
        self.br_cons_tail = cons_next;
    }
    pub fn putback_sc(&mut self, new: *mut u8){
        self.br_ring[self.br_cons_head as usize] = new;
    }
    pub fn peek(&self) -> Option<*mut u8>{
        if self.br_cons_head == self.br_prod_tail {
            None
        }
        else {
            Some(self.br_ring[self.br_cons_head as usize])
        }
    }
    pub fn peek_clear_sc(&self) -> Option<*mut u8>{
        if self.br_cons_head == self.br_prod_tail {
            None
        }
        else {
            Some(self.br_ring[self.br_cons_head as usize])
        }
    }
    pub fn full(&self) -> bool{
        (self.br_prod_head + 1) & self.br_prod_mask == self.br_cons_tail
    }
    pub fn empty(&self) -> bool{
        self.br_cons_head == self.br_prod_tail
    }
    pub fn count(&self) -> i32{
        ((self.br_prod_size + self.br_prod_tail as i32 - self.br_cons_tail as i32) & self.br_prod_mask) as i32
    }
}

impl Drop for Ring {
    fn drop(&mut self){
        
    }
}
