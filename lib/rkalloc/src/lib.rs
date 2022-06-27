// SPDX-License-Identifier: BSD-3-Clause
// rkalloc/lib.rs

// Authors: 张子辰 <zichen350@gmail.com>

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
use core::mem::{align_of,size_of};
use core::ptr::NonNull;
extern crate alloc;
use alloc::alloc::GlobalAlloc;

/// Runikraft的内存分配器API
/// `RKalloc`没有模仿`uk_alloc`，而是模仿了`alloc::alloc::GlobalAlloc`，
/// 因为`GlobalAlloc`要求实现的函数更少，而且`dealloc`函数要求提供分配时
/// 的`size`和`align`，实现起来更容易
///
/// # 安全性
///
/// `self`具有内部可变性，实现应该用自旋锁保证线程安全
pub unsafe trait RKalloc: Sync {
    /// 分配大小为`size`，对齐要求为`align`(必须是2^n)的连续内存空间
    ///
    /// 成功时返回非空指针，失败时返回空指针
    /// 
    /// # 安全性
    /// 
    /// - 返回值可能是空指针
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8;

    /// 解分配内存
    ///
    /// # 安全性
    ///
    /// - `ptr` 必须由同一个分配器分配
    /// - `size`和`align`必须和`alloc`时一致
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize);

    /// 与`alloc`类似，但在分配后清空内存
    unsafe fn alloc_zeroed(&self, size: usize, align: usize) -> *mut u8 {
        let ptr = self.alloc(size, align);
        if !ptr.is_null() {
            ptr.write_bytes(0, size);
        }
        ptr
    }

    /// 重新分配内存，将原有内存区域的数据照原样复制到新的内存区域，
    /// 成功时返回新内存区域的地址，并自动释放原有的空间
    /// 失败时返回空指针，原本的内存空间保持不变
    ///
    /// # 安全性
    ///
    /// - `old_ptr` 必须由同一个分配器分配
    /// - `old_size`和`align`必须和`alloc`时一致
    unsafe fn realloc(&self, old_ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        if new_size == old_size {
            return old_ptr;
        }
        let new_ptr = self.alloc(new_size, align);
        if !new_ptr.is_null() {
            new_ptr.copy_from_nonoverlapping(old_ptr, core::cmp::min(old_size,new_size));
            self.dealloc(old_ptr, old_size, align);
        }
        new_ptr
    }
}

/// 为C接口拓展的内存分配器，相比Rkalloc，它支持运行时不提供size、alloc
pub unsafe trait RKallocExt: RKalloc {
    /// 解分配内存
    ///
    /// # 安全性
    ///
    /// - `ptr` 必须由同一个分配器分配
    unsafe fn dealloc_ext(&self, ptr: *mut u8);

    /// 重新分配内存，将原有内存区域的数据照原样复制到新的内存区域，
    /// 成功时返回新内存区域的地址，并自动释放原有的空间
    /// 失败时返回空指针，原本的内存空间保持不变
    ///
    /// # 安全性
    ///
    /// - `old_ptr` 必须由同一个分配器分配
    unsafe fn realloc_ext(&self, old_ptr: *mut u8, new_size: usize) -> *mut u8;
}

/// 为了方便使用，需要分配器的结构体应该保存静态的分配器引用，然而构造的分配器不一定拥有
/// 静态生命周期，所以定义了这个强制构造静态生命周期的分配器的函数
pub unsafe fn make_static<'a>(a: &'a dyn RKalloc)->&'static dyn RKalloc {
    union Helper<'a> {
        ptr: *const dyn RKalloc,
        ref_: &'a dyn RKalloc,
    }
    Helper{ptr: Helper{ref_: a}.ptr}.ref_
}

pub unsafe fn make_static_ext<'a>(a: &'a dyn RKallocExt)->&'static dyn RKallocExt {
    union Helper<'a> {
        ptr: *const dyn RKallocExt,
        ref_: &'a dyn RKallocExt,
    }
    Helper{ptr: Helper{ref_: a}.ptr}.ref_
}

/// 分配器的状态信息
pub trait RKallocState {
    /// 总空间
    fn total_size(&self) -> usize;
    /// 可用空间
    ///
    /// **注意**: 
    /// - total_size-free_size不等于请求分配的空间的总大小，因为 1. 为了满足对齐要求和分配器
    /// 实现定义的最小分配空间要求，实际分配的空间大于等于请求分配的空间；2. 分配器的元数据需要
    /// 占用内存空间。rk_blkdev_get
    /// - free_size大于等于请求分配的空间时不一定能分配成功，因为可能无法找到足够大的连续内存空间。
    fn free_size(&self) -> usize;
}

/// 分配一段空间，并把T保存在此处
pub unsafe fn alloc_type<T> (alloc: &dyn RKalloc, elem: T) -> *mut T{
    let p = alloc.alloc(size_of::<T>(), align_of::<T>()) as *mut T;
    p.write(elem);
    p
}

/// 释放*T的空间，并调用drop
pub unsafe fn dealloc_type<T> (alloc: &dyn RKalloc, ptr: *mut T) {
    ptr.drop_in_place();
    alloc.dealloc(ptr as *mut u8, size_of::<T>(), align_of::<T>());
}

// FIXME: Unikraft允许注册多个全局分配器，但Runikraft暂时只支持一个
struct RustStyleAlloc {
    a: Option<NonNull<dyn RKalloc>>,
    e: Option<NonNull<dyn RKallocExt>>,
    s: Option<NonNull<dyn RKallocState>>,
}

unsafe impl GlobalAlloc for RustStyleAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.a.unwrap().as_ref().alloc(layout.size(), layout.align())
    }
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.a.unwrap().as_ref().alloc_zeroed(layout.size(), layout.align())
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        self.a.unwrap().as_ref().realloc(ptr, layout.size(), new_size, layout.align())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.a.unwrap().as_ref().dealloc(ptr, layout.size(), layout.align());
    }
}

#[global_allocator]
static mut ALLOCATOR: RustStyleAlloc = RustStyleAlloc{a:None,e:None,s:None};

/// 获取全局默认分配器
pub fn get_default() -> Option<&'static dyn RKalloc> {
    unsafe{
        ALLOCATOR.a.map(|x| x.as_ref())
    }
}

/// 获取全局默认拓展分配器
pub fn get_default_ext() -> Option<&'static dyn RKallocExt> {
    unsafe{
        ALLOCATOR.e.map(|x| x.as_ref())
    }
}

/// 获取全局默认分配器的状态信息
pub fn get_default_state() -> Option<&'static dyn RKallocState> {
    unsafe{
        ALLOCATOR.s.map(|x| x.as_ref())
    }
}

/// 注册全局默认分配器
pub unsafe fn register(a: *const dyn RKalloc) {
    ALLOCATOR.a = NonNull::new(a as *mut _);
}

/// 注册全局默认拓展分配器
pub unsafe fn register_ext(e: *const dyn RKallocExt) {
    ALLOCATOR.e = NonNull::new(e as *mut _);
}

/// 注册全局默认分配器的状态信息
pub unsafe fn register_state(s: *const dyn RKallocState) {
    ALLOCATOR.s = NonNull::new(s as *mut _);
}

//使用feature使它默认不被编译，这样rust-analyzer就不会因为找不到crate __alloc_error_handler报错
#[cfg(feature="__alloc_error_handler")]
extern crate __alloc_error_handler;
