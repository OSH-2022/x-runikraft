// 能被并发读写，内部可变性
// TODO: 传入所有权, 返回所有权
#![no_std]

use rkalloc::*;

pub struct Ring {

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
    pub fn dequeue_sc(&self) -> Option<*mut u8>{
        None
    }
    pub fn advance_sc(&self){

    }
    pub fn putback_sc(&self, new: *mut u8){
        
    }
    pub fn peek(&self) -> Option<*mut u8>{
        None
    }
    pub fn peek_clear_sc(&self) -> Option<*mut u8>{
        None
    }
    pub fn full(&self)->bool{
        false
    }
    pub fn empty(&self)->bool{
        false
    }
    pub fn count(&self)->usize{
        0
    }
}

impl Drop for Ring {
    fn drop(&mut self){
        
    }
}
