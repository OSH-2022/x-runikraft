// SPDX-License-Identifier: BSD-3-Clause
// slist.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use core::ptr::NonNull;

#[repr(C)]
pub struct SlistNode<T> {
    pub element: T,     //为了pop_front方法能获取element
    pub next: Option<NonNull<SlistNode<T>>>,
}

impl<T> SlistNode<T> {
    pub fn new(element: T) -> Self {
        SlistNode { next: None, element}
    }
}

/// 单链表
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - head                  头结点
/// - push_front            头插入
/// - pop_front             弹出头
/// - insert_after          指定位置之后插入
/// - remove_after          删除指定位置之后的元素
#[derive(Default)]
pub struct Slist<T> {
    head: Option<NonNull<SlistNode<T>>>,
}

impl<T> Slist<T> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// 链表是否为空
    #[inline] #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 头结点
    #[inline] #[must_use]
    pub fn head(&self) -> Option<NonNull<SlistNode<T>>> {
        self.head
    }

    /// 在头部插入新结点
    pub fn push_front(&mut self, mut node: NonNull<SlistNode<T>>){
        unsafe{
            node.as_mut().next = self.head;
            self.head = Some(node);
        }
    }

    /// 弹出头部的结点
    pub fn pop_front(&mut self) -> Option<NonNull<SlistNode<T>>> {
        self.head.map(|x| {
            self.head = unsafe{x.as_ref().next};
            x
        })
    }
}

impl<T> SlistNode<T> {
    /// 在结点之后插入
    pub fn insert_after(&mut self, mut node: NonNull<SlistNode<T>>) {
        unsafe {
            node.as_mut().next = self.next;
            self.next = Some(node)
        }
    }

    /// 在结点之后删除，不修改被删除的结点的next指针
    pub fn remove_after(&mut self) -> Option<NonNull<SlistNode<T>>> {
        unsafe {
            self.next.map(|x| {
                self.next = x.as_ref().next;
                x
            })
        }
    }

    pub fn is_tail(&self) -> bool{
        self.next.is_none()
    }
}

impl<T> Drop for Slist<T> {
    fn drop(&mut self) {
        assert!(self.is_empty());
    }
}

use core::iter::Iterator;

/// 迭代器
pub struct SlistIter<T> {
    pub node: Option<NonNull<SlistNode<T>>>,
}

impl<T> Slist<T> {
    /// 迭代器
    #[inline]
    pub fn iter(&self) -> SlistIter<T> {
        SlistIter { node: self.head }
    }
}

impl<T: 'static> Iterator for SlistIter<T> {
    type Item = &'static mut SlistNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|mut node| {
            unsafe{self.node = node.as_mut().next;
            node.as_mut()}
        })
    }
}
