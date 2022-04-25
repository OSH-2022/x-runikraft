// 能被并发读写，内部可变性
// TODO: 传入所有权, 返回所有权
#![no_std]

use rkalloc::*;

pub struct Ring {

}

impl Ring {
    /// `count`: 容量
    /// `alloc`: 分配器
    fn new(count: i32,a: &dyn RKalloc)->Ring{

    }
    fn enqueue(&self,buf: *mut u8) -> Result<(),i32> {

    }
    fn dequeue_mc(&self) -> Option<*mut u8>{

    }
    fn dequeue_sc(&self) -> Option<*mut u8>{

    }
    fn advance_sc(&self){

    }
    fn putback_sc(&self, new: *mut u8){
        
    }
    fn peek(&self) -> Option<*mut u8>{

    }
    fn peek_clear_sc(&self) -> Option<*mut u8>{

    }
    fn full(&self)->bool{

    }
    fn empty(&self)->bool{

    }
    fn count(&self)->usize{

    }
}

impl Drop for Ring {
    fn drop(&mut self){
        
    }
}
