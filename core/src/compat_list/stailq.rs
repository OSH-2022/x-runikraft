// SPDX-License-Identifier: BSD-3-Clause
// stailq.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use core::ptr::NonNull;

#[repr(C)]
pub struct StailqNode<T> {
    pub element: T,
    pub next: Option<NonNull<StailqNode<T>>>,
}

impl<T> StailqNode<T> {
    pub fn new(element: T) -> Self {
        StailqNode { next: None, element}
    }
}

/// 单尾队列
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - head                  头结点
/// - tail                  尾结点
/// - push_front            头插入
/// - pop_front             弹出头
/// - push_back             尾插入
/// - insert_after          指定位置之后插入
/// - remove_after          删除指定位置之后的元素
#[derive(Default)]
pub struct Stailq<T> {
    head: Option<NonNull<StailqNode<T>>>,
    tail: Option<NonNull<StailqNode<T>>>,
}

impl<T> Stailq<T> {
    pub const fn new() -> Self {
        Self { head: None, tail: None }
    }

    /// 链表是否为空
    #[inline] #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 头结点
    #[inline] #[must_use]
    pub fn head(&self) -> Option<NonNull<StailqNode<T>>> {
        self.head
    }

    /// 尾结点
    #[inline] #[must_use]
    pub fn tail(&self) -> Option<NonNull<StailqNode<T>>> {
        self.tail
    }

    /// 在头部插入新结点
    #[inline]
    pub fn push_front(&mut self, mut node: NonNull<StailqNode<T>>) {
        unsafe{
            if self.head.is_none() {
                debug_assert!(self.tail.is_none());
                self.tail = Some(node);
            }
            node.as_mut().next = self.head;
            self.head = Some(node);
        }
    }

    /// 弹出头部的结点
    pub fn pop_front(&mut self) -> Option<NonNull<StailqNode<T>>> {
        self.head.map(|head| {
            self.head = unsafe{head.as_ref().next};
            if self.head.is_none() {
                debug_assert_eq!(head,self.tail.unwrap());
                self.tail = None;
            }
            head
        })
    }

    /// 在尾部插入新结点
    pub fn push_back(&mut self, mut node: NonNull<StailqNode<T>>) {
        unsafe{
            node.as_mut().next = None;
            if let Some(mut tail) = self.tail {
                tail.as_mut().next = Some(node);
                self.tail = Some(node);
            }
            else {
                debug_assert!(self.head.is_none());
                self.tail = Some(node);
                self.head = Some(node);
            }
        }
    }
}

impl<T> StailqNode<T> {
    /// 在结点之后插入
    pub fn insert_after(&mut self, mut node: NonNull<StailqNode<T>>, owner: Option<&mut Stailq<T>>) {
        if self.next.is_none() {
            owner.unwrap().tail = Some(node);
        }
        unsafe {
            node.as_mut().next = self.next;
            self.next = Some(node);
        }
    }

    /// 在结点之后删除，不修改被删除的结点的next指针
    pub fn remove_after(&mut self, owner: Option<&mut Stailq<T>>) -> Option<NonNull<StailqNode<T>>> {
        unsafe {
            self.next.map(|x| {
                self.next = x.as_ref().next;
                if self.next.is_none() {
                    owner.unwrap().tail = NonNull::new(self);
                }
                x
            })
        }
    }

    pub fn is_tail(&self) -> bool {
        self.next.is_none()
    }
}

impl<T> Drop for Stailq<T> {
    fn drop(&mut self) {
        assert!(self.is_empty())
    }
}

use core::iter::Iterator;

/// 迭代器
pub struct StailqIter<T> {
    pub node: Option<NonNull<StailqNode<T>>>,
}

impl<T> Stailq<T> {
    /// 迭代器
    #[inline]
    pub fn iter(&self) -> StailqIter<T> {
        StailqIter { node: self.head }
    }
}

impl<T: 'static> Iterator for StailqIter<T> {
    type Item = &'static mut StailqNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|mut node| {
            unsafe{ self.node = node.as_mut().next;
            node.as_mut()}
        })
    }
}
