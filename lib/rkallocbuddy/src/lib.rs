// SPDX-License-Identifier: BSD-3-Clause
// rkallocbuddy/lib.rs

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

//! 伙伴分配器（buddy allocator）。
//!
//! # 设计
//!
//! 这里不介绍伙伴分配器的定义，只介绍具体的实现。
//!
//! 首先考虑大小为2^n的被管理的内存区域。
//!
//! 最小的块的长度是2^4 bytes，能容纳2个指针；最大的块的大小为2^48 bytes，是AMD64支持的最大内存容量。
//! 用双向链表维护空闲块，链表的结点`Node`储存在它对应的内存区域的开头。
//! 用树状bitset维护所有的内存区块的分配情况（元数据）：
//! - `[0]`是根结点；
//! - `[i*2+1]`是`[i]`的左孩子，它对应的内存区域是`i`二分后的前半段；
//! - `[i*2+2]`是`[i]`的右孩子，它对应的内存区域是`i`二分后的后半段；
//! - 顺序（order）为`k`的结点在这个bitset的索引范围是`[2^(n-k)-1:2^(n-k+1)-2]`；
//! - 顺序为`k`的结点共有2^(n-k)个，每个的大小为2^k
//! - 整个bitset的大小为2^(n+1)/16/8=2^(n-6) bytes
//! - `bitset[i]` = 0 表示结点i：
//!     - i 没有被分配
//!     - i 的父结点已经被分配
//! - `bitset[i]` = 1 表示结点i：
//!     - i 被分配
//!     - i 被二分成了两个子结点
//! 初始时，元数据的所有位都是0，如果一块内存i被分配，则i的孩子一定全是0，i和i的祖先一定全是1。
//!
//! 元数据被储存在被管理的内存区块的末尾，它们需要占用2^(n-10)个最小的块。
//!
//! 当内存区域的大小size不是2的幂时，记n=ceil(log2(size))，则可以将内存区域视为大小为2^n但末尾的一些
//! 结点已经被分配的内存区域。

// TODO: 更好的realloc实现
#![no_std]

use rkalloc::{Alloc, AllocState, AllocExt};
use rkplat::spinlock;
use core::cell::UnsafeCell;
use core::cmp::max;
use core::ptr::null_mut;
use runikraft::config::rkplat::PAGE_SIZE;

//最小的内存块大小
const MIN_SIZE: usize = 1usize << MIN_ORDER;
//最大的内存块大小
const MAX_SIZE: usize = 1usize << MAX_ORDER;
const MIN_ORDER: usize = 4;
const MAX_ORDER: usize = 48;
//页的对齐要求
const PAGE_ALIGNMENT: usize = PAGE_SIZE;

struct Data<'a> {
    //空闲区块列表(双向循环链表)，order-MIN_ORDER才是free_list_head的索引
    //【注意】访问free_list_head时，下标通常是[i-MIN_ORDER]
    free_list_head: [*mut Node; MAX_ORDER - MIN_ORDER + 1],
    max_order: usize,       //order的最大值，等于floor(log2(total))
    root_order: usize,      //根区块的order，等于ceil(log2(total))
    meta_data: Bitset<'a>,
    base: *const u8,        //内存空间的基地址

    //状态信息
    size_data: usize,    
    size_left: usize,       //剩余可用空间大小
    size_total: usize,      //总可用空间大小
}

pub struct AllocBuddy<'a> {
    lock: spinlock::SpinLock,
    
    data: UnsafeCell<Data<'a>>,
}

/// 大小和Node相同的Bitset, 储存空间的分配情况
struct Bitset<'a> {
    data: &'a mut [usize],
}

impl Bitset<'_> {
    unsafe fn new(data: *mut usize, len: usize) -> Self {
        data.write_bytes(0, len);
        Bitset { data: core::slice::from_raw_parts_mut(data, len) }
    }
    fn get(&self, index: usize) -> bool {
        if index / 64 >= self.data.len() {
            false
        }
        else {
            (self.data[index / 64] & (1usize << index % 64)) != 0
        }
    }
    fn set(&mut self, index: usize, data: bool) {
        if data {
            self.data[index / 64] |= 1usize << index % 64;
        }
        else {
            self.data[index / 64] &= !(1usize << index % 64);
        }
    }
}

#[inline(always)] fn lchild(i: usize) -> usize {i*2+1}
#[inline(always)] fn rchild(i: usize) -> usize {i*2+2}
#[inline(always)] fn parent(i: usize) -> usize {(i-1)/2}
#[inline(always)] fn sibling(i: usize) -> usize {((i-1)^1)+1}

#[derive(Clone, Copy)]
struct Node {
    pub pre: *mut Node,     //前驱结点
    pub next: *mut Node,    //后继结点
}

impl Node {
    ///初始化循环双链表（pre和next都指向自己）
    fn init(&mut self) {
        self.pre = self;
        self.next = self;
    }
}

impl Data<'_> {
    ///在结点head之后插入结点node
    unsafe fn insert_node(&mut self, order: usize, node: *mut Node) {
        debug_assert!(!node.is_null());
        if self.free_list_head[order - MIN_ORDER].is_null() {
            (*node).init();
            self.free_list_head[order - MIN_ORDER] = node;
        }
        else {
            let head = self.free_list_head[order - MIN_ORDER];
            (*node).next = (*head).next;
            (*head).next = node;
            debug_assert!(!(*node).next.is_null());
            (*(*node).next).pre = node;
            (*node).pre = head;
        }
    }

    ///把一个结点移出链表
    unsafe fn remove_node(&mut self, order: usize, node: *mut Node) {
        debug_assert!(!node.is_null());
        debug_assert!(!(*node).pre.is_null());
        debug_assert!(!(*node).next.is_null());
        if (*node).next == node {
            self.free_list_head[order - MIN_ORDER] = null_mut();
        }
        else {
            (*(*node).pre).next = (*node).next;
            (*(*node).next).pre = (*node).pre;
            self.free_list_head[order - MIN_ORDER] = (*node).pre;
        }
    }
}

const fn log2_usize(mut x: usize) -> usize {
    let mut y = 0_usize;
    if x>=4294967296 {y+=32;x>>=32;}
    if x>=65536 {y+=16;x>>=16;}
    if x>=256 {y+=8;x>>=8;}
    if x>=16 {y+=4;x>>=4;}
    if x>=4 {y+=2;x>>=2;}
    if x>=2 {y+=1;}
    y
}

const fn min_power2(x: usize) -> usize {
    let y = log2_usize(x);
    if 1<<y == x {x}
    else {1<<(y+1)}
}

/// 通过二分查找确定元数据块的数量
/// - `t`: 数据块总数
/// - `m`: 元数据块数
/// - `d`: t-m
/// m满足 m>= ceil( (2^ceil(log2(d)) -2 + d +1)/128 )
///         = floor((2^ceil(log2(d)) -2 + d)/128) + 1
/// 其中的(2^ceil(log2(d)) -2 + d)是最大的可分配数据块在元数据对应的bitset的索引
/// 可以初步估计 floor(t/65) <= m <= ceil(t/43)
fn find_n_meta(t: usize) -> usize {
    let (mut l, mut r) = (t / 65, (t + 42) / 43);
    let ok = |m: usize| {
        let d = t - m;
        m >= (min_power2(d) - 2 + d) / 128 + 1
    };
    while l != r {
        let mid = l + r >> 1;
        if ok(mid) {
            r = mid;
        }
        else {
            l = mid + 1;
        }
    }
    l
}

impl Data<'_> {
    ///确定一个结点的在meta_data中的索引
    #[inline(always)]
    fn index(&self, addr: *const Node, order: usize) -> usize {
        let addr = addr as *const u8;
        debug_assert!(order >= MIN_ORDER);
        debug_assert!(order <= self.max_order);
        debug_assert!(addr >= self.base);
        (1 << (self.root_order - order)) - 1 + unsafe { addr.offset_from(self.base) as usize } / (1 << order)
    }

    /// 创建伙伴分配器示例
    /// - `base`: 内存区域的基地址，必须4k对齐
    /// - `size`: 内存区域的大小，不必是2^n，但必须是16的倍数
    /// # 安全性
    /// - base..base+size范围的地址不能有其他用途
    unsafe fn new(base: *mut u8, size: usize) -> Self {
        debug_assert!(!base.is_null());
        debug_assert!(size % MIN_SIZE == 0);
        debug_assert!(base as usize % PAGE_ALIGNMENT == 0);
        debug_assert!(size <= MAX_SIZE);

        //总的16B-块数
        let n_blocks = size / MIN_SIZE;
        //用来存元数据的16B-块数
        let n_meta_blocks = find_n_meta(n_blocks);
        //用来存能被分配出去的数据的16B-块数
        let n_data_blocks = n_blocks - n_meta_blocks;
        debug_assert!(n_meta_blocks >= n_data_blocks / 64);
        //let meta_size = n_meta_blocks*MIN_SIZE;
        let data_size = n_data_blocks * MIN_SIZE;

        let max_order = log2_usize(data_size);
        let root_order = if 1 << max_order == data_size { max_order } else { max_order + 1 };
        let mut free_list_head = [null_mut(); MAX_ORDER - MIN_ORDER + 1];
        //debug_assert!((1<<root_order-MIN_ORDER+1)/64 < n_meta_blocks*2);

        //将空闲结点加入空闲结点链表
        {
            let mut size = data_size;
            let mut base = base;
            while size > 0 {
                let i = log2_usize(size);
                let node = base as *mut Node;
                (*node).init();
                free_list_head[i - MIN_ORDER] = node;
                base = base.offset(1 << i);
                size -= 1 << i;
            }
        }

        Data {
            free_list_head,
            max_order,
            root_order,
            meta_data: Bitset::new(base.add(data_size) as *mut usize, n_meta_blocks * 2),
            base,
            size_data: data_size,
            size_left: data_size,
            size_total: size,
        }
    }

    /// 这里的size就是实际上要分配的结点的大小，由buddy allocator的特点，size正确就一定能满足对其要求
    /// 调用前要对self加锁
    unsafe fn alloc_mut(&mut self, size: usize) -> *mut u8 {
        let log2size = log2_usize(size);
        let mut i = log2size;
        //从log2size开始，找到大小为2^i的空闲的块
        while i <= self.max_order && self.free_list_head[i - MIN_ORDER].is_null() {
            i += 1;
        }
        //找不到大小足够的块
        if i > self.max_order { return null_mut(); }

        let ptr = self.free_list_head[i - MIN_ORDER];
        debug_assert!(!ptr.is_null());
        self.remove_node(i, ptr);

        while i != log2size {
            i -= 1;
            self.split(ptr, i);
        }
        debug_assert!(self.meta_data.get(self.index(ptr, i)) == false);
        self.meta_data.set(self.index(ptr, i), true);

        // 清空元数据
        (*ptr).pre = null_mut();
        (*ptr).next = null_mut();
        // 更新统计信息
        self.size_left -= size;
        ptr as *mut u8
    }

    unsafe fn dealloc_mut(&mut self, ptr: *mut u8, size: usize) {
        let mut ptr = ptr as *mut Node;
        let mut order = log2_usize(size);
        let mut i = self.index(ptr, order);
        self.meta_data.set(i, false);
        //i的伙伴是空闲的，将i与i的伙伴合并
        while i != 0 && !self.meta_data.get(sibling(i)) && self.meta_data.get(parent(i)) {
            ptr = self.merge(ptr, order, i);
            order += 1;
            i = parent(i);
        }
        self.insert_node(order, ptr);
        self.size_left += size;
    }

    /// 将一个大的内存块分割成两个小的，将地址较大的一段插入free_list_head[order-MIN_ORDER]处
    /// 【注意】order是ptr拆分后的order，而不是ptr本身的order
    unsafe fn split(&mut self, mut ptr: *mut Node, order: usize) {
        let i = self.index(ptr, order + 1);
        //把它标记为被分裂了
        debug_assert!(self.meta_data.get(i) == false);
        self.meta_data.set(i, true);
        //它的分裂产生的两个子结点应该处在可用状态
        debug_assert!(self.meta_data.get(lchild(i)) == false);
        debug_assert!(self.meta_data.get(rchild(i)) == false);
        ptr = (ptr as *mut u8).offset(1 << order) as *mut Node; //现在ptr指向分裂后的结点的右孩子
        //找不到更小的块才会去尝试拆分更大的块, 所以order层的可用结点列表一定是空的
        debug_assert!(self.free_list_head[order - MIN_ORDER].is_null());
        (*ptr).init();
        self.free_list_head[order - MIN_ORDER] = ptr;
    }

    /// 把位于`free_list`之外的`ptr`与位于`free_list`之内的伙伴内存块合并，返回合并后的内存块的地址
    /// 合并后的内存块**不**被拆入`free_list`中，`order`是ptr自身的order, `i`是`ptr`在meta_data中的索引
    unsafe fn merge(&mut self, ptr: *mut Node, order: usize, i: usize) -> *mut Node {
        debug_assert_eq!(i, self.index(ptr, order));
        //i左孩子，i的伙伴的起始地址是ptr+(1<<order-MIN_ORDER)
        if i % 2 == 1 {
            let buddy = ptr.add(1 << order - MIN_ORDER);
            self.remove_node(order, buddy);
            debug_assert!(self.meta_data.get(parent(i)));
            self.meta_data.set(parent(i), false);
            ptr
        }
        else {
            let buddy = ptr.sub(1 << order - MIN_ORDER);
            self.remove_node(order, buddy);
            debug_assert!(self.meta_data.get(parent(i)));
            self.meta_data.set(parent(i), false);
            buddy
        }
    }

    /// 获取指针指向的内存空间在分配时使用的size
    fn find_size_when_alloc(&self, ptr: *mut u8) -> usize {
        if ptr as usize % 16 != 0 {
            panic!("{:?} is not correctly aligned", ptr);
        }
        let ptr = ptr as *mut Node;
        for order in (4..self.root_order).rev() {
            if ptr as usize % (1 << order) != 0 { continue; }
            if ptr as usize - self.base as usize + (1 << order) > self.size_data { continue; }
            if self.meta_data.get(self.index(ptr, order)) == false {
                return 1 << order + 1;
            }
        }
        MIN_SIZE
    }
}

unsafe impl Sync for AllocBuddy<'_>{}

impl AllocBuddy<'_> {
    pub unsafe fn new(base: *mut u8, size: usize) -> Self {
        Self { lock: spinlock::SpinLock::new(), data: UnsafeCell::new(Data::new(base,size)) }
    }
}

unsafe impl Alloc for AllocBuddy<'_> {
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        debug_assert!(align.is_power_of_two());
        debug_assert!(align <= PAGE_ALIGNMENT);
        //实际上需要分配的内存大小
        let size = min_power2(max(max(size, align), MIN_SIZE));
        //剩余空间不足
        if (*self.data.get()).size_left < size {
            return null_mut();
        }
        let _lock = self.lock.lock();
        return (*self.data.get()).alloc_mut(size);
    }
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize) {
        if ptr.is_null() { return; }
        debug_assert!(align.is_power_of_two());
        // debug_assert!(align <= PAGE_ALIGNMENT);
        let size = min_power2(max(max(size, align), MIN_SIZE));
        let _lock = self.lock.lock();
        return (*self.data.get()).dealloc_mut(ptr, size);
    }
}

unsafe impl AllocExt for AllocBuddy<'_> {
    unsafe fn dealloc_ext(&self, ptr: *mut u8) {
        if ptr.is_null() { return; }
        let size = (*self.data.get()).find_size_when_alloc(ptr);
        let _lock = self.lock.lock();
        (*self.data.get()).dealloc_mut(ptr, size);
    }

    unsafe fn realloc_ext(&self, old_ptr: *mut u8, new_size: usize) -> *mut u8 {
        if old_ptr.is_null() { return self.alloc(new_size, 16); }
        let old_size = (*self.data.get()).find_size_when_alloc(old_ptr);
        self.realloc(old_ptr, old_size, new_size, 16)
    }
}


impl AllocState for AllocBuddy<'_> {
    fn total_size(&self) -> usize { unsafe{(*self.data.get()).size_total} }
    fn free_size(&self) -> usize { unsafe{(*self.data.get()).size_left} }
}

// mod debug;
