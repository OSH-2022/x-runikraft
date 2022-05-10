#![no_std]

use core::intrinsics::atomic_store_unordered;
use rkalloc::RKalloc;
use crate::blkdev_core::{RkBlkdev, RkBlkdevEventHandler, RkBlkdevState};
use crate::{_alloc_data, BLKDEV_COUNT, CONFIG_LIBUKBLKDEV_MAXNBQUEUES};
use crate::blkreq::RkBlkreq;
use crate::blkreq::RkBlkreqState::RkBlkreqFinished;

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
pub unsafe fn rk_blkdev_drv_register(mut dev: RkBlkdev, a: &dyn RKalloc, drv_name: &str) -> i16 {

    //数据必须被取消分配
    assert_ne!(dev._data);
    //断言必要的配置
    if let Some(x) = BLKDEV_COUNT {
        dev._data = _alloc_data(a, x as u16, drv_name);
    }

    if !dev._data.is_null() {
        return -12;
    }

    (*dev._data).state = RkBlkdevState::RkBlkdevUnconfigured;
    if let Some(mut x) = &RK_BLKDEV_LIST {
        x.push_back(dev);
    }
    //TODO println!("Registered blkdev%{:?}:{:?} {:?}\n", BLKDEV_COUNT, dev, drv_name);
    BLKDEV_COUNT = match BLKDEV_COUNT {
        None => Some(1),
        Some(x) => Some(x + 1)
    };
    return match BLKDEV_COUNT {
        None => 0,
        Some(y) => y
    };
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
    let queue_handler:RkBlkdevEventHandler;
    assert!(!dev._data.is_null());
    assert!(queue_id < CONFIG_LIBUKBLKDEV_MAXNBQUEUES);
    queue_handler=dev._data.queue_handler[queue_id];
    //TODO #[cfg(feature = "dispatcherthreads")]
    //TODO uk_semaphore_up(&queue_handler->events);
        queue_handler.callback(dev,queue_id,queue_handler.cookie)
}
/**
 * Sets a request as finished.
 *
 * @param req
 *	uk_blkreq structure
 */
pub unsafe fn rk_blkdev_finished(req:RkBlkreq){
    atomic_store_unordered(*(req.state),RkBlkreqFinished)
}
/// 释放给Runikraft块设备的数据
/// 把设备从列表中移除
///
/// @参数 dev
///
///     Runikraft块设备
///
pub fn rk_blkdev_drv_unregister(dev: &RkBlkdev) {
    todo!()
}
