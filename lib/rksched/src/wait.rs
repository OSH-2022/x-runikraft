// SPDX-License-Identifier: BSD-3-Clause
// wait.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use crate::thread::Thread;
use rklist::STailq;
use rkalloc::RKalloc;

/// 等待队列条目结构体
pub struct WaitQEntry {
    waiting: bool,
    thread: *mut Thread,
}

impl WaitQEntry {
    /// 等待队列条目初始化
    pub fn new(&mut self, thread: *mut Thread) -> Self {
        Self {
            waiting: false,
            thread,
        }
    }
}

/// 等待队列头结点结构体
pub struct WaitQ {
    q: STailq<*mut WaitQEntry>,
}

impl WaitQ {
    pub fn new(alloc: &'static dyn RKalloc)->Self {
        Self { q: STailq::new(alloc) }
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }

    pub fn add(&mut self, entry: *mut WaitQEntry) {
        unsafe {
            if !(*entry).waiting {
                self.q.push_back(entry).unwrap();
                (*entry).waiting = true;
            }
        }
    }

    pub fn remove(&mut self, entry: *mut WaitQEntry) {
        todo!();
    }

    pub fn wake_up(&mut self) {
        todo!();
    }

    pub fn wait_event(&mut self,condition: bool) {
        todo!();
    }

    pub fn clear(&mut self) {
        self.q.clear()
    }
}
