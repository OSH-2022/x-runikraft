// SPDX-License-Identifier: BSD-3-Clause
// blkdev_driver.rs

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

use core::sync::atomic;
use rkalloc::{dealloc_type, RKalloc, RKallocExt};
use crate::blkdev_core::{RkBlkdev, RkBlkdevEventHandler, RkBlkdevState};
use crate::{_alloc_data, BLKDEV_COUNT, CONFIG_LIBUKBLKDEV_MAXNBQUEUES, ptriseer, RK_BLKDEV_LIST, RkBlkdevData};
use crate::blkdev_core::RkBlkdevState::{RkBlkdevConfigured, RkBlkdevUnconfigured};
use crate::blkreq::RkBlkreq;
use crate::blkreq::RkBlkreqFinished;

/// 向设备链表增加Runikraft块设备
/// 一旦驱动增加了新找到的设备，这个函数就应该被调用
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 a
///
///    将被用于librkblkdev私有数据的分配器
///
/// @参数 drv_name
///
///    （可选）驱动名称
///    给这个字符串分配的内存必须保持可用直到设备被登记
///
/// @返回值
///
/// - （-ENOMEM）：私有分配
/// - （正值）：成功时的块设备的身份
pub fn rk_blkdev_drv_register(mut dev: RkBlkdev, a: &dyn RKalloc, drv_name: &str) -> i16 {

    //数据必须被取消分配
    assert!(ptriseer(dev._data as i64));
    //断言必要的配置
    unsafe {
        if let Some(x) = BLKDEV_COUNT {
            dev._data = _alloc_data(a, x as u16, drv_name);
        }
    }

    if !dev._data.is_null() {
        return -12;
    }

    unsafe {
        (*dev._data).state = RkBlkdevUnconfigured;
        if let Some(mut x) = &RK_BLKDEV_LIST {
            x.push_back(dev);
        }
        println!("Registered blkdev%{:?}:{:?} {:?}\n", BLKDEV_COUNT, dev, drv_name);
        BLKDEV_COUNT = match BLKDEV_COUNT {
            None => Some(1),
            Some(x) => Some(x + 1)
        };
        return match BLKDEV_COUNT {
            None => 0,
            Some(y) => y
        };
    }
}

/// 把一个队列事件向应用程序接口用户前移
/// 可以（并且应该）在设备中断的上下文中调用
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
///    接收事件相应的队列身份
pub fn rk_blkdev_drv_queue_event(dev: &RkBlkdev, queue_id: u16) {
    let queue_handler: RkBlkdevEventHandler;
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    queue_handler = dev._data.queue_handler[queue_id];
    //TODO #[cfg(feature = "dispatcherthreads")]
    // uk_semaphore_up(&queue_handler->events);
    #[cfg(not (feature = "dispatcherthreads"))]
    (queue_handler.callback)(dev, queue_id, queue_handler.cookie);
}

/**
 * Sets a request as finished.
 *
 * @param req
 *    uk_blkreq structure
 */
pub fn rk_blkdev_finished(req: RkBlkreq) {
    req.state.store(RkBlkreqFinished, atomic::Ordering::Release);
}

/// 释放给Runikraft块设备的数据
/// 把设备从列表中移除
///
/// @参数 dev
///
///     Runikraft块设备
///
pub fn rk_blkdev_drv_unregister(dev: &RkBlkdev) {
    let mut id: u16;
    assert!(!dev._data.is_null());
    assert!(dev._data.state == RkBlkdevUnconfigured);
    id = dev._data.id;
    unsafe {
        dealloc_type::<RkBlkdevData>(dev._data.a, dev._data);
        if let Some(x) = BLKDEV_COUNT {
            BLKDEV_COUNT = Some(x - 1);
        }
    }
    println!("Unregistered blkdev{}: {:?}\n", id, dev);
}
