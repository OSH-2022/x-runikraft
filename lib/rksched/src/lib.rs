// SPDX-License-Identifier: BSD-3-Clause
// rksched/lib.rs

// Authors: 陈建绿 <2512674094@qq.com>
//          张子辰 <zichen350@gmail.com>

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

pub mod sched;
pub mod thread;
pub mod wait;

pub use sched::RKsched;

/// 针对当前线程的操作
pub mod this_thread {
    use core::time::Duration;
    use crate::thread::{Thread,ThreadRef};
    use runikraft::config::STACK_SIZE;
    ///返回当前线程的控制块
    pub fn control_block() -> ThreadRef {
        let thread_pointer = rkplat::lcpu::read_sp() / STACK_SIZE * STACK_SIZE;
        unsafe{(*(thread_pointer as *mut Thread)).as_ref()}
    }

    pub fn r#yield() {
        let current = control_block();
        unsafe {
            let s=current.sched;
            assert!(!s.is_null());
            (*s).r#yield();
        }
    }
    pub fn sleep_for(duration: Duration) {
        control_block().block_timeout(duration);
    }
    pub fn exit()->! {
        let current = control_block();
        unsafe {
            let s=current.sched;
            assert!(!s.is_null());
            (*s).remove_thread(current);
        }
        panic!("should exit");
    }
}
