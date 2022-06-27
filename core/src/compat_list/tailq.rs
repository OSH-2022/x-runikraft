// SPDX-License-Identifier: BSD-3-Clause
// tailq.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use core::ptr::NonNull;

#[repr(C)]
pub struct TailqNode<T> {
    pub element: T,
    pub prev: Option<NonNull<TailqNode<T>>>,
    pub next: Option<NonNull<TailqNode<T>>>,
}

impl<T> TailqNode<T> {
    pub fn new(element: T) -> Self {
        TailqNode { prev: None, next: None, element}
    }
}

/// 双向尾队列
/// 
/// 支持的操作：
/// - new                   创建新链表
/// - is_empty              是否为空
/// - head                  头结点
/// - tail                  尾结点
/// - push_front            头插入
/// - pop_front             弹出头
/// - push_back             尾插入
/// - pop_back              弹出尾
/// - insert_before         在指定位置前插入
/// - insert_after          指定位置之后插入
/// - remove_before         删除指定位置之前的元素
/// - remove                删除指定位置的元素
/// - remove_after          删除指定位置之后的元素
/// - set_alone             清空结点的指针信息，调用后is_alone=true
/// - is_alone              结点是否不位于任何队列
#[derive(Default)]
pub struct Tailq<T> {
    head: Option<NonNull<TailqNode<T>>>,
    tail: Option<NonNull<TailqNode<T>>>,
}

impl<T> Tailq<T> {
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
    pub fn head(&self) -> Option<NonNull<TailqNode<T>>> {
        self.head
    }

    /// 尾结点
    #[inline] #[must_use]
    pub fn tail(&self) -> Option<NonNull<TailqNode<T>>> {
        self.tail
    }

    /// 在头部插入新结点
    #[inline]
    pub fn push_front(&mut self, mut node: NonNull<TailqNode<T>>) {
        unsafe {
            node.as_mut().prev = None;
            node.as_mut().next = self.head;
            if let Some(mut head) = self.head {
                head.as_mut().prev = Some(node);
            }
            else {
                self.tail = Some(node);
            }
            self.head = Some(node);
        }
    }

    /// 弹出头部的结点
    pub fn pop_front(&mut self) -> Option<NonNull<TailqNode<T>>> {
        unsafe {
            self.head.map(|head| {
                self.head = head.as_ref().next;
                if let Some(mut head) = self.head {
                    head.as_mut().prev = None;
                }
                else {
                    self.tail = None;
                }
                head
            })
        }
    }

    /// 在尾部插入新结点
    #[inline]
    pub fn push_back(&mut self, mut node: NonNull<TailqNode<T>>) {
        unsafe{
            node.as_mut().next = None;
            node.as_mut().prev = self.tail;
            if let Some(mut tail) = self.tail {
                tail.as_mut().next = Some(node);
            }
            else {
                debug_assert!(self.head.is_none());
                self.head = Some(node);
            }
            self.tail = Some(node);
        }
    }

    /// 弹出头部的结点
    pub fn pop_back(&mut self) -> Option<NonNull<TailqNode<T>>> {
        unsafe {
            self.tail.map(|tail| {
                self.tail = tail.as_ref().prev;
                if let Some(mut tail) = self.tail {
                    tail.as_mut().next = None;
                }
                tail
            })
        }
    }
}

impl<T> TailqNode<T> {
    /// 在结点之前插入
    pub fn insert_before(&mut self, mut node: NonNull<TailqNode<T>>, owner: Option<&mut Tailq<T>>) {
        unsafe {
            node.as_mut().prev = self.prev;
            node.as_mut().next = NonNull::new(self);
            if let Some(mut prev) = self.prev {
                prev.as_mut().next = Some(node);
            }
            else {
                owner.unwrap().head = Some(node);
            }
            self.prev = Some(node);
        }
    }

    /// 在结点之后插入
    pub fn insert_after(&mut self, mut node: NonNull<TailqNode<T>>, owner: Option<&mut Tailq<T>>) {
        unsafe {
            node.as_mut().next = self.next;
            node.as_mut().prev = NonNull::new(self);
            if let Some(mut next) = self.next {
                next.as_mut().prev = Some(node);
            }
            else {
                owner.unwrap().tail = Some(node);
            }
            self.next = Some(node);
        }
    }

    /// 在结点之前删除，不修改被删除的结点的prev和next指针
    pub fn remove_before(&mut self, owner: Option<&mut Tailq<T>>) -> Option<NonNull<TailqNode<T>>> {
        unsafe {
            if let Some(prev) = self.prev {
                self.prev = prev.as_ref().prev;
                if let Some(mut prev_prev) = self.prev {
                    prev_prev.as_mut().next = NonNull::new(self);
                }
                else {
                    owner.unwrap().head = NonNull::new(self);
                }
                Some(prev)
            }
            //self是头结点
            else {None}
        }
    }

    /// 将self从链表中删除，不修改self的prev和next指针
    pub fn remove(&mut self, mut owner: Option<&mut Tailq<T>>) {
        unsafe {
            if let Some(mut prev) = self.prev {
                prev.as_mut().next = self.next;
            }
            else {
                owner.as_mut().unwrap().head = self.next;
            }
            if let Some(mut next) = self.next {
                next.as_mut().prev = self.prev;
            }
            else {
                owner.unwrap().tail = self.prev;
            }
        }
    }

    /// 在迭代器指向的位置之后删除
    pub fn remove_after(&mut self, owner: Option<&mut Tailq<T>>) -> Option<NonNull<TailqNode<T>>> {
        unsafe {
            if let Some(next) = self.next {
                self.next = next.as_ref().next;
                if let Some(mut next_next) = self.next {
                    next_next.as_mut().prev = NonNull::new(self);
                }
                else {
                    owner.unwrap().tail = NonNull::new(self);
                }
                Some(next)
            }
            //self是尾结点
            else {None}
        }
    }

    pub fn is_tail(&self) -> bool {
        self.next.is_none()
    }

    pub fn is_head(&self) -> bool {
        self.prev.is_none()
    }

    pub fn is_alone(&self) -> bool {
        self.prev.is_none() && self.next.is_none()
    }

    pub fn set_alone(&mut self) {
        self.prev = None;
        self.next = None;
    }
}


impl<'a,T> Drop for Tailq<T> {
    fn drop(&mut self) {
        assert!(self.is_empty());
    }
}

use core::iter::Iterator;

/// 迭代器
pub struct TailqIter<T> {
    pub node: Option<NonNull<TailqNode<T>>>,
}

impl<T> Tailq<T> {
    /// 迭代器
    #[inline]
    pub fn iter(&self) -> TailqIter<T> {
        TailqIter { node: self.head }
    }
}

impl<T:'static> Iterator for TailqIter<T> {
    type Item = &'static mut TailqNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|mut node| {
            unsafe{self.node = node.as_mut().next;
            node.as_mut()}
        })
    }
}
