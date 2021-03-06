// SPDX-License-Identifier: BSD-3-Clause
// blkdev.rs

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

use core::cmp::min;
use core::mem::size_of;
use core::ptr::write_bytes;
use rkplat::println;

use crate::blkdev_core::{RkBlkdev, RkBlkdevCap, RkBlkdevConf, RkBlkdevInfo, RkBlkdevQueueConf, RkBlkdevQueueInfo, RkBlkdevState};
use crate::blkdev_core::RkBlkdevState::{RkBlkdevConfigured, RkBlkdevRunning, RkBlkdevUnconfigured};
use crate::{_create_event_handler, _destory_event_handler, BLKDEV_COUNT, CONFIG_LIBUKBLKDEV_MAXNBQUEUES, ptriseer, RK_BLKDEV_LIST};
use crate::blkreq::{rk_blkreq_init, RkBlkreq, RkBlkreqOp};
use crate::blkreq::RkBlkreqOp::{RkBlkreqRead, RkBlkreqWrite};


/// 得到可得到的Runikraft块设备的数量
///
/// @返回值
///    - （usize）：块设备的数量
///
pub fn rk_blkdev_count() -> i16 {
    unsafe {
        match BLKDEV_COUNT {
            None => 0,
            Some(x) => x
        }
    }
}

/// 得到一个Runikraft块设备的引用，基于它的身份
/// 这个引用应该被应用保存并用于后续的应用程序接口调用
///
/// @参数 id
///
/// 要配置的Runikraft块设备的识别符
///
/// @返回值
/// - None：在列表中没有找到设备
/// - Some(&mut RkBlkdev)：将传递给应用程序接口的引用
pub fn rk_blkdev_get(id: u16) -> Option<&'static RkBlkdev<'static>> {
    unsafe {
        if let Some(x) = &RK_BLKDEV_LIST {
            let iter = x.iter();
            for x in iter {
                if (*x._data).id == id {
                    return Some(x);
                }
            }
        }
    }
    None
}

/// 返回块设备的身份
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - None：如果没有定义名称
/// - &str：如果名称可得到，返回字符串的引用
///
pub fn rk_blkdev_id_get(dev: RkBlkdev) -> u16 {
    unsafe { (*dev._data).id }
}

/// Returns the driver name of a blkdev device.
/// The name might be set to NULL.
///
/// @param dev
///
/// The Unikraft Block Device.
///
/// @return
/// - (NULL): if no name is defined.
/// - (const char *): Reference to string if name is available.
fn rk_blkdev_drv_name_get<'a>(dev: RkBlkdev) -> &str {
    unsafe { (*(dev._data)).drv_name }
}

/// 返回一个块设备的当前状态
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - enum RkBlkdevState：当前设备状态
fn rk_blkdev_state_get<'a>(dev: &RkBlkdev) -> &'a RkBlkdevState {
    unsafe { &(*(dev._data)).state }
}

/// 询问设备容量
/// 信息对设备初始化有用（例如可支持队列得的最大值）
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 dev_info
///
///     一个指向将装有块设备上下文信息的*RkBlkdevInfo*类型的指针
///
/// @返回值
///
/// - 0：成功
/// - <0：驱动器错误
pub fn rk_blkdev_get_info(dev: &RkBlkdev, dev_info: &mut RkBlkdevInfo) -> isize {
    let rc = 0;
    //在向驱动程序询问容量之前清除值
    unsafe { write_bytes::<RkBlkdevInfo>(dev_info, 0, size_of::<RkBlkdevInfo>()); }
    dev.dev_ops.get_info(dev_info);
    //根据应用程序接口的配置，限制最大的队列数
    dev_info.max_queues = min(16, dev_info.max_queues);
    rc
}

/// 配置一个Runikraft块设备
///
/// 这个函数必须在其他任何块应用程序接口被调用前被调用。
///
/// 当设备处于停止状态时，这个函数也可以被再次调用
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - 0：成功，设备被配置
/// - <0：被设备配置函数返回的错误码
fn rk_blkdev_configure(dev: &RkBlkdev, conf: &RkBlkdevConf) -> isize {
    let mut rc = 0;
    let mut dev_info: RkBlkdevInfo;
    rc = rk_blkdev_get_info(dev, &mut dev_info);
    if rc != 0 {
        unsafe { println!("blkdev{}:Failed to get initial info{}\n", (*dev._data).id, rc); }
        return rc;
    }
    if conf.nb_queues > dev_info.max_queues {
        return -12;
    }
    rc = dev.dev_ops.dev_configure(conf);
    if rc != 0 {
        unsafe {
            println!("blkdev{}: Configured interface\n", (*dev._data).id);
            (*dev._data).state = RkBlkdevConfigured;
        }
    } else {
        unsafe { println!("blkdev{}:Failed to configure interface {}\n", (*dev._data).id, rc); }
    }
    rc
}

/// 询问设备队列容量
///
/// 信息对设备队列初始化有用（例如在队列中可支持的描述符的最大值
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
///     将建立队列的索引
///
///     值必须位于过去应用于rk_blkdev_configure()的范围[0,nb_queue-1]内
///
/// @参数 q_info
///
/// 指向将被填写的RkBlkdevQueueInfo结构体的指针
///
/// @返回值
/// - 0：成功，队列信息被填写
/// - <0：驱动程序函数的错误码
fn rk_blkdev_queue_get_info(dev: &RkBlkdev, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize {
    //在向驱动程序询问队列容量之前清除值
    unsafe { write_bytes::<RkBlkdevQueueInfo>(q_info, 0, size_of::<RkBlkdevQueueInfo>()); }
    dev.dev_ops.queue_get_info(queue_id, q_info)
}

/// 为Runikraft块设备分配并建立一个队列
/// 这个队列负责请求和响应
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
///     将建立队列的索引
///
///     值必须位于过去应用于rk_blkdev_configure()的范围[0,nb_queue-1]内
///
/// @参数 nb_desc
///
///     队列中描述符的数量
///
///     检查rk_blkdev_queue_get_info()以取回限制
///
/// @参数 queue_conf
///
///     指向将用于队列配置的数据的指针
///
///     这个可以在多个队列配置之间共享
///
/// @返回值
///
/// - 0：成功，收到被正确建立的队列
/// - <0：不能分配也不能建立环描述符
///

fn rk_blkdev_queue_configure(dev: &RkBlkdev, queue_id: u16, nb_desc: u16, queue_conf: *mut RkBlkdevQueueConf) -> isize {
    assert!(!dev._data.is_null());

    unsafe {
        if let RkBlkdevConfigured = &(*dev._data).state {
            return 22;
        }
    }
    //确保我们没有第二次对这个队列进行初始化
    if !ptriseer(dev._queue[queue_id as usize].unwrap() as i64) {
        return -12;
    }
    #[cfg(not(feature = "dispatcherthreads"))]
        let err = _create_event_handler(queue_conf.callback, queue_conf.callback_cookie, dev._data.queue_handler[queue_id]);
    #[cfg(feature = "dispatcherthreads")]
        let err = _create_event_handler(queue_conf.callback, queue_conf.callback_cookie, dev, queue_id, queue_conf.s, dev._data.queue_handler[queue_id]);
    if err == 0 {
        return err;
    }
    dev._queue[queue_id] = dev.dev_ops.queue_configure(queue_id, nb_desc, queue_conf);
    if ptriseer(dev._queue[queue_id] as i64) {
        println!("blkdev{}-q{}: Failed to configure:{}\n", dev._data.id, queue_id, err);
        _destory_event_handler((dev._data.queue_handler[queue_id]));
        return err;
    }
    println!("blkdev{}: Configured queue {}\n", dev._data.id, queue_id);
    0
}

///
/// 开启块设备
///
/// 设备开启步骤是最后一步，并且由设定卸载特性及开始传输、以及接收设备单元组成
///
/// 一旦成功，被Runikraft块应用程序接口的所有基本函数都可以被调用
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - 0：成功，Runikraft块设备开启
/// - <0：驱动程序设备开启函数的错误码
///
fn rk_blkdev_start(dev: &RkBlkdev) -> isize {
    let mut rc = 0;
    assert!(!dev._data.is_null());
    assert!(dev._data.state == RkBlkdevConfigured);
    rc = dev.dev_ops.dev_start();
    if rc != 0 {
        println!("blkdev{}: Failed to start interface{}\n", dev._data.id, rc);
    } else {
        println!("blkdev{}: Start interface{}\n", dev._data.id);
        dev._data.state = RkBlkdevRunning;
    }
    rc
}

///得到存有关于设备信息的容量信息，例如nb_of_sectors、sector_size等等
///
/// @返回值
///
///     一个指向类型*RkBlkdevCapabilities*的指针
///
#[inline]
fn rk_blkdev_capbilities<'a>(blkdev: &'a RkBlkdev) -> &'a RkBlkdevCap {
    assert!(blkdev._data.state >= RkBlkdevRunning);
    &blkdev.capabilities
}

///允许队列中断
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
/// 将被建立的队列的指引
///
/// 值必须位于过去应用于rk_blkdev_configure()的范围[0,nb_queue-1]内
///
/// @返回值
/// - 0：成功，中断被允许
/// - -ENOTSUP：驱动设备不支持中断
///
#[inline]
fn rk_blkdev_queue_intr_enable(dev: &RkBlkdev, queue_id: u16) -> bool {
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(!ptriseer(dev._queue[queue_id]));
    dev.dev_ops.queue_intr_enable(dev._queue[queue_id])
}

/// 禁止队列中断
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
/// 将被建立的队列的指引
///
/// 值必须位于过去应用于rk_blkdev_configure()的范围\[0,nb_queue-1\]内
///
/// @返回值
/// - 0：成功，中断被禁止
/// - -ENOTSUP：驱动设备不支持中断
#[inline]
fn rk_blkdev_queue_intr_disble(dev: RkBlkdev, queue_id: u16) -> bool {
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(!ptriseer(dev._queue[queue_id]));
    dev.dev_ops.queue_intr_disable(dev._queue[queue_id])
}

/// 向设备发送一个异步非阻塞模式请求
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
/// 将被建立的队列的指引
///
/// 值必须位于过去应用于rk_blkdev_configure()的范围\[0,nb_queue-1\]内
///
/// @参数 rqe
///
/// 请求结构体
///
/// @返回值
/// - >=0：状态标记正值
///     - RK_BLKDEV_STATUS_SUCCESS：`req`被成功加入队列
///     - RK_BLKDEV_STATUS_MORE：表明为了后续的传输仍然至少可得到一个描述符
///
///       如果标记没有被设置，表明队列已满
///
///         这仅仅可能在RK_BLKDEV_STATUS_SUCCESS时被同时设置
/// - <0：从驱动程序得到的错误码，没有发送任何请求
fn rk_blkdev_queue_submit_one(dev: &RkBlkdev, queue_id: u16, req: &mut RkBlkreq) -> isize {
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(dev._data.state == RkBlkdevRunning);
    assert!(!ptriseer(dev._queue[queue_id]));
    dev.submit_one(dev, dev._queue[queue_id], req)
}

///
/// 测试由`rk_blkdev_submit_one`返回的状态标记
///
/// 当函数返回一个错误码或者被选中的一个标记没有被设定，这个函数返回假
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @参数 flag
///
/// 要测试的标记
///
/// @返回值
/// - true：所有标记被设定并且没有负值
/// - false：至少一个标记没有被设定或状态是负值
#[inline]
fn rk_blkdev_status_test_set(status: isize, flag: isize) -> bool {
    (status & (flag | -2_147_483_648i32)) == flag
}

///
/// 测试由`rk_blkdev_submit_one`返回的未设置标记
///
/// 当函数返回一个错误码或者被选中的一个标记被设定，这个函数返回假
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @参数 flag
///
/// 要测试的标记
///
/// @返回值
/// - true：标记没有被设定并且没有负值
/// - false：至少一个标记被设定或状态是负值
#[inline]
fn rk_blkdev_status_test_unset(status: isize, flag: isize) -> bool {
    (status & (flag | -2_147_483_648i32)) == 0x0
}

/// 测试`rk_blkdev_submut_one`返回的状态是否表明了一个成功的操作
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：操作是成功的
/// - false：操作是不成功的或者发生了错误
#[inline]
fn rk_blkdev_status_successful(status: isize) -> bool {
    rk_blkdev_status_test_set(status, 0x1)
}

/// 测试`rk_blkdev_submut_one`返回的状态是否表明操作需要被重试
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：操作应该被重试
/// - false：操作是成功的或者发生了错误
#[inline]
fn rk_blkdev_status_notready(status: isize) -> bool {
    rk_blkdev_status_test_unset(status, 0x1)
}

/// 测试`rk_blkdev_submut_one`返回的状态是否表明了上一个操作可以被再一次成功重复
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：状态RK_BLKDEV_STATUS_MORE被设置
/// - false：操作是成功的或者发生了错误
#[inline]
fn rk_blkdev_status_more(status: isize) -> bool {
    rk_blkdev_status_test_set(status, 0x1 | 0x2)
}

/// 在队列和在目标队列上重新被许可的中断被重新许可之前，从它们那里得到回应
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
/// 队列的指引
///
/// @返回值
/// - 0：成功
/// - <0：当驱动程序返回错误的时候
fn rk_blkdev_queue_finish_reqs(dev: &RkBlkdev, queue_id: u16) -> isize {
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(dev._data.state == RkBlkdevRunning);
    assert!(!ptriseer(dev._queue[queue_id] as i64));
    dev.finish_reqs(dev, dev._queue[queue_id])
}

/**
 * Used for sending a synchronous request.
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub struct RkBlkdevSyncIORequest {
    /* Request structure. */
    req: RkBlkreq,
    /* Semaphore used for waiting after the response is done. */
    s: RkSemaphore,
}

#[cfg(feature = "sync_io_blocked_waiting")]
pub fn __sync_io_callback(req: &RkBlkreq, cookie_callback: *mut u8) {
    let sync_io_req: *const RkBlkdevSyncIORequest = cookie_callback as *const RkBlkdevSyncIORequest;
    //TODO uk_semaphore_up(&sync_io_req->s);
}

/**
 * Make a sync io request on a specific queue.
 * `uk_blkdev_queue_finish_reqs()` must be called in queue interrupt context
 * or another thread context in order to avoid blocking of the thread forever.
 *
 * @param dev
 *    The Unikraft Block Device
 * @param queue_id
 *    queue_id
 * @param op
 *    Type of operation
 * @param sector
 *    Start Sector
 * @param nb_sectors
 *    Number of sectors
 * @param buf
 *    Buffer where data is found
 * @return
 *    - 0: Success
 *    - (<0): on error returned by driver
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkdev_sync_io(dev: &RkBlkdev, queue_id: u16, operation: RkBlkreqOp, start_sector: Sector, nb_sectors: Sector, buf: *mut u8) -> isize {
    let mut rc = 0;
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(!dev._data.is_null());
    assert!(dev._data.state == RkBlkdevRunning);
    assert!(!ptriseer(dev._queue[queue_id] as i64));
    let sync_io_req: RkBlkdevSyncIORequest = RkBlkdevSyncIORequest;
    let mut req: &RkBlkreq = &sync_io_req.req;
    rk_blkreq_init(&mut req, operation, start_sector, nb_sectors, buf, __sync_io_callback, *sync_io_req);
    req = &sync_io_req.req;
    //TODO uk_semaphore_init(&sync_io_req.s, 0);
    rc = rk_blkdev_queue_submit_one(dev, queue_id, &mut req);
    //TODO uk_semaphore_down(&sync_io_req.s);
    req.result
}
/*
 * Wrappers for uk_blkdev_sync_io
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkdev_sync_write(dev: &RkBlkdev, queue_id: u16, op: RkBlkreqOp, sector: Sector, nb_sectors: Sector, buf: *mut u8) -> isize {
    rk_blkdev_sync_io(dev, queue_id, RkBlkreqWrite, sector, nb_sectors, buf)
}
/*
 * Wrappers for uk_blkdev_sync_io
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkdev_sync_read(dev: &RkBlkdev, queue_id: u16, op: RkBlkreqOp, sector: Sector, nb_sectors: Sector, buf: *mut u8) -> isize {
    rk_blkdev_sync_io(dev, queue_id, RkBlkreqRead, sector, nb_sectors, buf)
}

///停止一个Runikraft块设备，并且把他的状态设定为RK_BLKDEV_CONFIGED状态
///
/// 从现在开始，用户不能发送任何请求
///
/// 如果有被挂起的请求，这个函数将返回-EBUSY因为队列非空。
///
/// 如果采用的是轮询而不是中断，要确保在调用这个函数前清空队列并且处理所有的响应
///
/// 设备可以通过调用rk_blkdev_start来重启
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - 0：成功
/// - <0：当驱动程序返回错误的时候
fn rk_blkdev_stop(dev: &RkBlkdev) -> isize {
    let mut rc = 0;
    assert!(!dev._data.is_null());
    assert!(dev._data.state == RkBlkdevRunning);
    println!("Trying to stop blkdev {} device\n", dev._data.id);
    rc = dev.dev_ops.dev_stop();
    if rc != 0 {
        println!("Failed to stop blkdev {} device{}\n", dev._data.id, rc);
    } else {
        println!("Stopped blkdev{}\n", dev._data.id);
        dev._data.state = RkBlkdevConfigured;
    }
    rc
}

///清空一个队列和它的Runikraft设备描述符
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @参数 queue_id
///
/// 将被建立的队列的指引
///
/// 值必须位于过去应用于rk_blkdev_configure()的范围\[0,nb_queue-1\]内
///
/// @返回值
/// - 0：成功
/// - <0：当驱动程序返回错误的时候
fn rk_blkdev_queue_unconfigure(dev: &RkBlkdev, queue_id: u16) -> isize {
    let mut rc = 0;
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    assert!(dev._data.state == RkBlkdevConfigured);
    assert!(!ptriseer(dev._queue[queue_id] as i64));
    rc = dev.dev_ops.queue_unconfigure(dev._queue[queue_id]);
    if rc != 0 {
        println!("Failed to unconfigure blkdev{}-q{}: {}\n", dev._data.id, queue_id, rc);
    } else {
        #[cfg(feature = "dispatcherthreads")]
        if dev._data.queue_handler[queue_id].callback.is_null() {
            _destory_event_handler(dev._data.queue_handler[queue_id]);
        }
        println!("blkdev{}: Stopped blkdev{}\n", dev._data.id);
        dev._queue[queue_id] = None;
    }
    rc
}

/// 关闭一个已经停止的Runikraft块设备
///
/// @参数 dev
///
///     Runikraft块设备
///
/// 这个函数释放除被RK_BLKDEV_UNCONFIGURE状态使用的所有资源
///
/// 设备可以通过调用rk_blkdev_configure来重新配置
///
/// @返回值
/// - 0：成功
/// - <0：当驱动程序返回错误的时候
fn rk_blkdev_unconfigure(dev: &RkBlkdev) -> isize {
    let q_id: u16;
    let rc: isize;
    assert!(!dev._data.is_null());
    assert!(dev._data.state == RkBlkdevConfigured);
    for x in [0, CONFIG_LIBUKBLKDEV_MAXNBQUEUES] {
        assert!(ptriseer(dev._queue[x] as i64));
    }
    rc = dev.dev_ops.dev_unconfigure();
    if rc != 0 {
        println!("Failed to unconfigure blkdev{}: {}\n", dev._data.id, rc);
    } else {
        println!("Unconfigured blkdev{}\n", dev._data.id);
        dev._data.state = RkBlkdevUnconfigured;
    }
    rc
}
