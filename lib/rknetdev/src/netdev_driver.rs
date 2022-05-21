// SPDX-License-Identifier: BSD-3-Clause
// netdev_driver.rs

// Authors: Simon Kuenzer <simon.kuenzer@neclab.eu>
//          张子辰 <zichen350@gmail.com>

// Copyright (c) 2018, NEC Europe Ltd., NEC Corporation.
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
// Translated from unikraft/lib/uknetdev/include/uk/netdev_driver.h.

use super::netdev_core::{Netdev,CONFIG_LIBUKNETDEV_MAXNBQUEUES};
use rkalloc::RKalloc;
use core::ptr::addr_of_mut;

/// Adds a Runikraft network device to the device list.
/// This should be called whenever a driver adds a new found device.
///
/// - `dev`: Struct to runikraft network device that shall be registered
/// - `a`: Allocator to be use for rknetdev private data (dev->_data)
/// - `drv_name`: (Optional) driver name
///  The memory for this string has to stay available as long as the
///  device is registered.
/// - Return: 
///   - (-ENOMEM): Allocation of private
///   - (>=0): Network device ID on success
pub fn register<'a>(dev: &mut Netdev<'a>, a: *const dyn RKalloc, drv_name: &'a str)->Result<i32,i32>{
    unimplemented!();
}

/// Forwards an RX queue event to the API user
/// Can (and should) be called from device interrupt context
///
/// - `dev`: Runikraft network device to which the event relates to
/// - `queue_id`: receive queue ID to which the event relates to
pub fn rx_event(dev: &mut Netdev, queue_id: usize){
    assert!(queue_id<CONFIG_LIBUKNETDEV_MAXNBQUEUES);

    let p_dev = dev as *mut Netdev;
    let rxq_handler = &mut dev.data.rxq_handler[queue_id];
    #[cfg(feature="dispatcherthreads")]
    semaphore_up(&rxq_handler.events);//TODO
    #[cfg(not(feature="dispatcherthreads"))]
    (rxq_handler.callback)(p_dev,queue_id,rxq_handler.cookie);
}
