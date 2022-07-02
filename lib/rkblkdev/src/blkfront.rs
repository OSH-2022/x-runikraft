// SPDX-License-Identifier: BSD-3-Clause
// blkfront.rs

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

use rkalloc::RKalloc;
use crate::RkBlkdev;

/**
 * Queue Structure used for both requests and responses.
 * This is private to the drivers.
 * In the API, this structure is used only for type checking.
 * Structure used to describe a queue used for both requests and responses
 */
#[cfg(feature = "xen_blkfront_grefpool")]
pub struct RkBlkdevQueue {
    /* Front_ring structure */
    //TODO struct blkif_front_ring ring,
    /* Grant ref pointing at the front ring. */
    //TODO grant_ref_t ring_ref;
    /* Event channel for the front ring. */
    //TODO evtchn_port_t evtchn;
    /* Allocator for this queue. */
    a: dyn RKalloc,
    /* The libukblkdev queue identifier */
    queue_id: u16,
    /* The flag to interrupt on the queue */
    intr_enabled: isize,
    /* Reference to the Blkfront Device */
    /* Grant refs pool. */
    //TODO struct blkfront_grefs_pool ref_pool,
}

#[cfg(not(feature = "xen_blkfront_grefpool"))]
pub struct RkBlkdevQueue {
    /* Front_ring structure */
    //TODO struct blkif_front_ring ring,
    /* Grant ref pointing at the front ring. */
    //TODO grant_ref_t ring_ref;
    /* Event channel for the front ring. */
    //TODO evtchn_port_t evtchn;
    /* Allocator for this queue. */
    a: *const dyn RKalloc,
    /* The libukblkdev queue identifier */
    queue_id: u16,
    /* The flag to interrupt on the queue */
    intr_enabled: isize,
    /* Reference to the Blkfront Device */
}

/**
 * Structure used to describe the Blkfront device.
 */
pub struct BlkfrontDev<'a> {
    /* Xenbus Device. */
    //TODO struct xenbus_device *xendev;
    /* Blkdev Device. */
    blkdev: RkBlkdev<'a>,
    /* Blkfront device number from Xenstore path. */
    //TODO blkif_vdev_t	handle;
    /* Value which indicates that the backend can process requests with the
     * BLKIF_OP_WRITE_BARRIER request opcode.
     */
    barrier: isize,
    /* Value which indicates that the backend can process requests with the
     * BLKIF_OP_WRITE_FLUSH_DISKCACHE request opcode.
     */
    flush: i32,
    /* Number of configured queues used for requests */
    nb_queues: u16,
    /* Vector of queues used for communication with backend */
    queues: *mut RkBlkdevQueue,
    /* The blkdev identifier */
    uid: u16,
}
