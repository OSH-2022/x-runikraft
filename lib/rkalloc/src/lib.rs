#![no_std]

//extern crate alloc;
//use alloc::alloc::{GlobalAlloc,Layout};

//TODO: 暂时不支持分页，所有线程在同一个地址空间下执行
pub trait RKalloc {
    /// 分配大小为size的连续内存空间
    unsafe fn malloc(&mut self, size: usize)->*mut u8;
    /// 分配足以容纳n个大小为size的对象的连续内存空间
    unsafe fn calloc(&mut self, nmemb: usize, size: usize)->*mut u8;
    /// 分配满足对齐要求的内存空间
    unsafe fn memalign(&mut self, align: usize, size: usize)->*mut u8;
    /// 调整内存空间大小，并复制原有内容
    unsafe fn realloc(&mut self, ptr: *const u8, size: usize)->*mut u8;
    /// 释放内存空间
    unsafe fn free(&mut self, ptr: *const u8);
    //unsafe fn palloc(&mut self, num_pages: usize)->*mut c_void;
    //unsafe fn pfree(&mut self, ptr: *c_void, num_pages: usize);
    /// [可选] 把可用内存加入分配器
    unsafe fn addmem(&mut self, _base: *const u8, _size: usize)->i32 {-1}
    // TODO: maxalloc availmem pmaxalloc pavailmem
}

//把指针和usize强制绑定为一个union很丑陋，但这是我找到的唯一的创建全局多态指针的方法
union AllocWrapper{
    p: *mut dyn RKalloc,
    s: usize
}

//#[global_allocator]
static mut ALLOC: AllocWrapper = AllocWrapper{s:0};

// #[alloc_error_handler]
// fn allocate_fail(layout: Layout) -> ! {
//     panic!("Allocation fail: layout = {:?}", layout);
// }

// unsafe impl GlobalAlloc for AllocWrapper {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         assert!(self.s!=0);
//         (*self.p).calloc(layout.align(),layout.size())
//     }
//     unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout){
//         assert!(self.s!=0);
//         (*self.p).free(ptr);
//     }
// }

/// 注册全局分配器
pub unsafe fn register_alloc(alloc: *mut dyn RKalloc){
    ALLOC.p = alloc;
}

pub unsafe fn malloc(size: usize)->*mut u8{
    assert!(ALLOC.s!=0);
    (*ALLOC.p).malloc(size)
}

pub unsafe fn calloc(nmemb: usize, size: usize)->*mut u8{
    assert!(ALLOC.s!=0);
    (*ALLOC.p).calloc(nmemb,size)
}

pub unsafe fn memalign(align: usize, size: usize)->*mut u8{
    assert!(ALLOC.s!=0);
    (*ALLOC.p).calloc(align,size)
}

pub unsafe fn realloc(ptr: *const u8, size: usize)->*mut u8{
    assert!(ALLOC.s!=0);
    (*ALLOC.p).realloc(ptr,size)
}

pub unsafe fn free(ptr: *const u8){
    assert!(ALLOC.s!=0);
    (*ALLOC.p).free(ptr)
}

pub unsafe fn addmem(base: *const u8, size: usize)->i32{
    assert!(ALLOC.s!=0);
    (*ALLOC.p).addmem(base,size)
}


//Rust 风格的分配器
