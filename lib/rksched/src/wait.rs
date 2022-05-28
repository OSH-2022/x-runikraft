// SPDX-License-Identifier: BSD-3-Clause
// wait.rs
// Authors: 陈建绿 <2512674094@qq.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use crate::thread::Thread;
use rklist::STailq;

/// 等待队列条目结构体
pub struct WaitQEntry {
    waiting: i32,
    thread: *mut Thread,
}

impl WaitQEntry {
    //等待队列条目初始化
    pub fn new(&mut self, thread: *mut Thread) -> Self {
        Self {
            waiting: 0,
            thread,
        }
    }
}

/// 等待队列头结点结构体
pub type RKwaitQ = STailq<WaitQEntry>;
