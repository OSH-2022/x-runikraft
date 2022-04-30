#![no_std]
use rkalloc::RKalloc;
use core::ptr::null_mut;
/// 不带头结点链表的 trait 声明
pub trait RKlist<T> {
    /// 获取第一个结点，从 Ok() 中获取
    fn get_head(&self) -> Result<&mut T, i32>;
    /// 获取尾部结点，从 Ok() 中获取
    fn get_tail(&self) -> Result<&mut T, i32>;
    /// 获取某结点后的结点，从 Ok() 中获取
    fn get_after(&self, base_elem: &T) -> Result<&mut T, i32>;
    /// 获取某结点前的结点，从 Ok() 中获取
    fn get_prev(&self, base_elem: &T) -> Result<&mut T, i32>;
    /// 获取链表长度
    fn get_length(&self) -> u32;
    /// 判断链表是否为空
    fn is_empty(&self) -> bool;
    /// 在链表头部插入
    fn insert_head(&mut self, elem: T) -> Result<(), i32>;
    /// 在链表中某结点后插入
    fn insert_after(&mut self, elem: T, base_elem: &T) -> Result<(), i32>;
    /// 在链表尾部插入
    fn insert_tail(&mut self, elem: T) -> Result<(), i32>;
    /// 删除链表第一个结点，删除的结点在 Ok() 中获取
    fn remove_first(&mut self) -> Result<&mut T, i32>;
    /// 删除某结点后的结点，删除的结点在 Ok() 中获取
    fn remove_after(&mut self, base_elem: &T) -> Result<&mut T, i32>;
    /// 删除链表尾部结点
    fn remove_tail(&mut self) -> Result<&mut T, i32>;
    /// 交换链表中某两个元素
    fn swap(&mut self, elem1: &T, elem2: &T);
}

/// 单向尾部队列类型 条目 结构体
pub struct STailQEntry<T> {
    entry: T,
    next: *mut STailQEntry<T>,
}

impl<T> STailQEntry<T> {
    pub fn new(entry: T, next: *mut STailQEntry<T>) -> STailQEntry<T> {
        STailQEntry {
            entry,
            next,
        }
    }
}

/// 单向尾部队列类型结构体
pub struct STailQ<'a, T> {
    head: *mut STailQEntry<T>,
    length: u32,
    allocator: &'a dyn RKalloc,
}

impl<'a, T> STailQ<'a, T> {
    pub fn new(allocator: &'a dyn RKalloc) -> STailQ<'a, T> {
        STailQ {
            head: null_mut(),
            length: 0,
            allocator,
        }
    }
}

/// 为单向尾队列实现 trait RKlist
impl<'a, T> RKlist<STailQEntry<T>> for STailQ<'a, T> {
    fn get_head(&self) -> Result<&mut STailQEntry<T>, i32> {
        if !self.head.is_null() {
            Result::Ok(&mut self.head)
        }
        else {
            Result::Err(-1)
        }
    }
    //TODO
}

/// 双向尾部队列类型 条目 结构体
pub struct TailQEntry<T> {
    entry: T,
    prev: *mut TailQEntry<T>,
    next: *mut TailQEntry<T>,
}

impl<T> TailQEntry<T> {
    pub fn new(entry: T, prev: *mut TailQEntry<T>, next: *mut TailQEntry<T>) {
        TailQEntry {
            entry,
            prev,
            next,
        }
    }
}

/// 双向尾部队列类型结构体
pub struct TailQ<'a, T> {
    head: *mut TailQEntry<T>,
    length: u32,
    allocator: &'a dyn RKalloc,
}

impl<'a, T> TailQ<'a, T> {
    pub fn new(allocator: &'a dyn RKalloc) -> TailQ<'a, T> {
        TailQ {
            head: null_mut(),
            length: 0,
            allocator,
        }
    }
}

/// 为双向尾部队列实现 trait RKlist
impl<'a, T> RKlist<TailQEntry<T>> for TailQ<'a, T> {
    fn get_head(&self) -> Result<&mut TailQEntry<T>, i32> {
        if !self.head.is_null() {
            Result::Ok(&mut self.head)
        }
        else {
            Result::Err(-1)
        }
    }
    //TODO
}