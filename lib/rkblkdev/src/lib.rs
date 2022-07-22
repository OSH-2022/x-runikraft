// SPDX-License-Identifier: BSD-3-Clause
// rkblkdev/lib.rs

// Authors: Roxana Nicolescu  <nicolescu.roxana1996@gmail.com>
//          郭耸霄 <logname@mail.ustc.edu.cn>

// Copyright (c) 2019, University Politehnica of Bucharest.
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

extern crate alloc;

#[macro_use]
extern crate rkplat;

use core::ptr::{drop_in_place, null, null_mut};
use rkalloc::{alloc_type, Alloc};
use rksched::{Sched, ThreadAttr};
use runikraft::list::Tailq;
use crate::blkdev_core::{RkBlkdev, RkBlkdevData, RkBlkdevEventHandler, RkBlkdevQueueEventT, RkBlkdevState};

mod blkdev;
mod blkdev_core;
mod blkdev_driver;
mod blkreq;
mod blkfront;

static mut RK_BLKDEV_LIST: Option<Tailq<RkBlkdev>> = None;
static mut BLKDEV_COUNT: Option<i16> = None;
//FIXME
//const CONFIG_LIBUKBLKDEV_MAXNBQUEUES: u16 = core::u16::from_str(env!("PATH"));
const CONFIG_LIBUKBLKDEV_MAXNBQUEUES: u16 = 1;

pub unsafe fn _alloc_data<'a>(a: &'a (dyn Alloc + 'a), blkdev_id: u16, drv_name: &'a str) -> *mut RkBlkdevData<'a> {
    let mut data: *mut RkBlkdevData = alloc_type::<RkBlkdevData>(a, RkBlkdevData {
        id: blkdev_id,
        state: RkBlkdevState::RkBlkdevUnconfigured,
        queue_handler: [],
        drv_name,
        a,
    });
    if !data.is_null() {
        return null_mut();
    }
    //这仅仅会发生在我们设置设备身份的时候
    //在设备生命的剩余时间，这个身份是只读的
    data
}

#[cfg(feature = "dispatcherthreads")]
pub fn _dispatcher(args: *mut u8) {
    let handler = RkBlkdevEventHandler;
    loop {
        //TODO uk_semaphore_down(&handler->events);
        handler.callback(handler.dev, handler.queue_id, handler, cookie);
    }
}


#[cfg(not(feature = "dispatcherthreads"))]
pub fn _create_event_handler(callback: RkBlkdevQueueEventT, cookie: *mut u8, event_handler: &mut RkBlkdevEventHandler) -> isize {
    event_handler.callback = callback;
    event_handler.cookie = cookie;
    0
}

#[cfg(feature = "dispatcherthreads")]
pub fn _create_event_handler(callback: RkBlkdevQueueEventT, cookie: *mut u8, dev: *const RkBlkdev, queue_id: u16, s: *mut Sched, event_handler: &mut RkBlkdevEventHandler) -> isize {
    event_handler.callback = callback;
    event_handler.cookie = cookie;
    //如果我们没有回调，我们就不需要线程
    if callback.is_null() {
        return 0;
    }
    event_handler.dev = dev;
    event_handler.queue_id = queue_id;
    //TODO uk_semaphore_init(&event_handler->events, 0);
    event_handler.dispatcher_s = s;
    //为分派器线程创造一个名字
    //如果有错误，我们就在没有名字的状况下继续
    //TODO if (asprintf(&event_handler->dispatcher_name,
    // 		     "blkdev%" PRIu16 "-q%" PRIu16 "]", dev->_data->id,
    // 		     queue_id)
    // 	    < 0) {
    // 		event_handler->dispatcher_name = NULL;
    // 	}
    //创建线程
    unsafe { event_handler.dispatcher = (*event_handler.dispatcher_s).thread_create(event_handler.dispatcher_name, &mut RKthreadAttr::default(), _dispatcher, event_handler as *mut u8); }
    if event_handler.dispatcher.is_null() {
        if !event_handler.dispatcher_name.is_null() {
            unsafe { event_handler.dispatcher.drop_in_place(); }
            event_handler.dispatcher_name = null_mut();
        }
        return -12;
    }
    0
}

#[cfg(feature = "dispatcherthreads")]
pub fn _destory_event_handler(h: &mut RkBlkdevEventHandler) {
    if !h.dispatcher.is_null() {
        //TODO uk_semaphore_up(&h->events);
        assert!(!h.dispatcher_s.is_null());
        h.dispatcher.kill();
        h.dispatcher.wait();
        h.dispatcher = null_mut();
    }
    if !h.dispatcher_name.is_null() {
        unsafe { h.dispatcher_name; }
        h.dispatcher_name = null_mut();
    }
}

pub fn ptriseer(ptr: i64) -> bool {
    if ptr <= 0 && ptr >= -512 {
        true
    } else {
        false
    }
}
