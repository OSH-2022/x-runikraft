// SPDX-License-Identifier: BSD-3-Clause
// wait.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// 等待队列*没有*修改自Unikraft

use crate::thread::ThreadRef;
use rklist::{Stailq,STailqPosMut};
use rkalloc::RKalloc;
use rkplat::spinlock::SpinLock;

/// 等待队列
/// - 资源的等待队列：在资源可用时，等待队列里的第一个线程会被唤醒
/// - 线程等待队列：在线程结束时，等待队列里的线程会被全部唤醒
/// 一个线程至多位于一个等待队列，在线程退出时，它必须主动将自己从等待队列中移除
pub struct WaitQ {
    q: Stailq<ThreadRef>,
    mutex: SpinLock,
}

impl WaitQ {
    pub fn new(alloc: &'static dyn RKalloc)->Self {
        Self { q: Stailq::new(), mutex: SpinLock::new()}
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn add(&mut self, entry: ThreadRef) {
        let _lock = self.mutex.lock();
        self.q.push_back(entry).unwrap();
    }

    pub fn remove(&mut self, entry: ThreadRef) -> Option<ThreadRef>{
        let mut pos = STailqPosMut::default();
        let mut find = false;
        let _lock = self.mutex.lock();
        for i in self.q.iter_mut() {
            if *i == entry {
                find = true;
                break;
            }
            unsafe {pos = STailqPosMut::from_ref(i);}
        }
        if find { unsafe {
            if pos.is_null() { self.q.pop_front() }
            else { self.q.remove_after(pos)}
        }}
        else { None }
    }

    pub fn remove_first(&mut self) -> Option<ThreadRef>{
        let _lock = self.mutex.lock();
        self.q.pop_front()
    }

    pub fn wakeup_all(&mut self) {
        while !self.empty() {
            let mut t = self.remove_first().unwrap();
            t.wake();
        }
    }

    pub fn waitup_first(&mut self) {
        if let Some(t) = self.remove_first().as_mut() {
            t.wake();
        }
    }
}
