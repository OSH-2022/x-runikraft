#![no_std]

use rkalloc::RKalloc;

pub struct RKallocEmpty {
    base: *mut u8,
    top: *mut u8,
}

impl RKallocEmpty {
    pub fn new(base: *mut u8, size: usize)->RKallocEmpty{
        RKallocEmpty{base, top: unsafe{base.add(size)}}
    }
}

impl RKalloc for RKallocEmpty {
    // 分配大小为size的连续内存空间
    unsafe fn malloc(&mut self, size: usize)->*mut u8 {
        let p = self.base;
        if p>=self.top {return core::ptr::null_mut();}
        self.base = self.base.add(size);
        p
    }
    // 分配足以容纳n个大小为size的对象的连续内存空间
    unsafe fn calloc(&mut self, nmemb: usize, size: usize)->*mut u8 {
        self.malloc(nmemb*size)
    }
    // 分配满足对齐要求的内存空间
    unsafe fn memalign(&mut self, _align: usize, size: usize)->*mut u8 {
        self.malloc(size)
    }
    // 调整内存空间大小，并复制原有内容
    unsafe fn realloc(&mut self, ptr: *const u8, _size: usize)->*mut u8 {
        ptr as *mut u8
    }
    // 释放内存空间
    unsafe fn free(&mut self, _ptr: *const u8) {

    }
}
