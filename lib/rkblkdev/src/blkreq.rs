// SPDX-License-Identifier: BSD-3-Clause
// blkreq.rs

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

//use core::intrinsics::{atomic_load_unordered, atomic_store_unordered};
use core::sync::atomic;
#[cfg(feature = "sync_io_blocked_waiting")]
use crate::blkdev::RkBlkdevSyncIORequest;
use crate::blkdev_core::{RkBlkdevQueueFinishReqsT, RkBlkdevState};
use crate::blkdev_core::RkBlkdevState::RkBlkdevConfigured;
use crate::RkBlkdevEventHandler;

pub type Sector = usize;


///用于向设备发送请求
pub struct RkBlkreq {
    //输入成员
    ///操作类型
    pub(crate) operation: RkBlkreqOp,
    ///操作开始的起始扇区
    pub(crate) start_sector: Sector,
    ///扇区数量的大小
    pub(crate) nb_sectors: Sector,
    ///指向数据的指针
    pub(crate) aio_buf: *mut u8,
    ///回复的请求的参数
    #[cfg(feature = "sync_io_blocked_waiting")]
    cb_cookie: *const RkBlkdevSyncIORequest,
    //输出成员
    ///请求的状态：完成/未完成
    pub(crate) state: RkBlkreqState,
    ///操作状态的结果（错误返回负值）
    pub(crate) result: isize,
}

///操作状态
pub type RkBlkreqState = atomic::AtomicBool;
pub const RkBlkreqFinished: bool = true;
pub const RkBlkreqUnfinished: bool = false;

///支持的操作
pub enum RkBlkreqOp {
    ///读操作
    RkBlkreqRead,
    ///写操作
    RkBlkreqWrite,
    ///冲洗易变的写缓存
    RkBlkreqFflush,
}

///用于执行一个响应后的请求
/// @参数 req
///
/// RkBlkreq结构体
///
/// @参数 cookie_callback
///
///	由用户在递交请求时设定的可选参数
///
pub type RkBlkreqEventT = fn(&RkBlkreq, *mut u8);
///初始化一个请求结构体
///
///@参数 req
///
///	请求结构
///
///@参数 op
///
///	操作
///
///@参数 start
///
///	起始扇区
///
///@参数  nb_sectors
///
///	扇区数量
///
///@参数 aio_buf
///
///	数据缓冲区
///
///@参数 cb_cookie
///
///	请求回复的参数
///
#[inline]
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkreq_init(req: &mut RkBlkreq, op: RkBlkreqOp, start: Sector, nb_sectors: Sector, aio_buf: *mut u8, cb:RkBlkreqEventT, cb_cookie: *const RkBlkdevSyncIORequest) {
    req.operation=op;
    req.start_sector=start;
    req.nb_sectors=nb_sectors;
    req.aio_buf=aio_buf;
    req.state.store(RkBlkreqFinished, atomic::Ordering::Release);
    req.cb=cb;
    req.cb_cookie=cb_cookie;
}

#[inline]
#[cfg(not(feature = "sync_io_blocked_waiting"))]
pub fn rk_blkreq_init(req: &mut RkBlkreq, op: RkBlkreqOp, start: Sector, nb_sectors: Sector, aio_buf: *mut u8, cb:RkBlkreqEventT) {
    req.operation=op;
    req.start_sector=start;
    req.nb_sectors=nb_sectors;
    req.aio_buf=aio_buf;
    req.state.store(RkBlkreqFinished, atomic::Ordering::Release);
    req.cb=cb;
}

/// 检查请求是否结束
/// @参数 req
///
/// RkBlkreq结构体
unsafe fn rk_blkreg_is_done(req: &RkBlkreq) -> bool {
    req.state.load(atomic::Ordering::Relaxed) == RkBlkreqFinished
}
