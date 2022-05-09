#![no_std]

#[macro_use]
extern crate rkplat;

use core::cmp::min;
use core::mem::size_of;
use core::ptr::write_bytes;
use rkplat::println;

use crate::blkdev_core::{RkBlkdev, RkBlkdevCap, RkBlkdevConf, RkBlkdevInfo, RkBlkdevQueueConf, RkBlkdevQueueInfo, RkBlkdevState};
use crate::BLKDEV_COUNT;
use crate::blkreq::{RkBlkreq, RkBlkreqOp};
use crate::RkBlkdevState::RkBlkdevConfigured;



/// 得到可得到的Runikraft块设备的数量
///
/// @返回值
///    - （usize）：块设备的数量
///
pub unsafe fn rk_blkdev_count() -> i16 {
    match BLKDEV_COUNT {
        None => 0,
        Some(x) => x
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
pub unsafe fn rk_blkdev_get(id: u16) -> Option<&'static RkBlkdev<'static>> {
    if let Some(x) = &RK_BLKDEV_LIST {
        let iter = x.iter();
        for x in iter {
            if (*x._data).id == id {
                return Some(x);
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
pub unsafe fn rk_blkdev_id_get(dev: RkBlkdev) -> u16 {
    (*dev._data).id
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
unsafe fn rk_blkdev_drv_name_get<'a>(dev: RkBlkdev) -> &str {
    (*(dev._data)).drv_name
}

/// 返回一个块设备的当前状态
///
/// @参数 dev
///
///     Runikraft块设备
///
/// @返回值
/// - enum RkBlkdevState：当前设备状态
unsafe fn rk_blkdev_state_get<'a>(dev: &RkBlkdev) -> &'a RkBlkdevState {
    &(*(dev._data)).state
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
pub unsafe fn rk_blkdev_get_info(dev: &RkBlkdev, dev_info: &mut RkBlkdevInfo) -> isize {
    let rc = 0;
    //在向驱动程序询问容量之前清除值
    write_bytes::<RkBlkdevInfo>(dev_info, 0, size_of::<RkBlkdevInfo>());
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
unsafe fn rk_blkdev_configure(dev: &RkBlkdev, conf: &RkBlkdevConf) -> isize {
    let mut rc = 0;
    let mut dev_info: RkBlkdevInfo;
    rc = rk_blkdev_get_info(dev, &mut dev_info);
    if rc != 0 {
        println!("blkdev{}:Failed to get initial info{}\n", (*dev._data).id, rc);
        return rc;
    }
    if conf.nb_queues > dev_info.max_queues {
        return -12;
    }
    rc = dev.dev_ops.dev_configure(conf);
    if rc != 0 {
        println!("blkdev{}: Configured interface\n", (*dev._data).id);
        (*dev._data).state = RkBlkdevState::RkBlkdevConfigured;
    } else {
        println!("blkdev{}:Failed to configure interface {}\n", (*dev._data).id, rc);
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
unsafe fn rk_blkdev_queue_get_info(dev: &RkBlkdev, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize {
    //在向驱动程序询问队列容量之前清除值
    write_bytes::<RkBlkdevQueueInfo>(q_info, 0, size_of::<RkBlkdevQueueInfo>());
    dev.dev_ops.queue_get_info(dev, queue_id, q_info)
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

unsafe fn rk_blkdev_queue_configure(dev: &RkBlkdev, queue_id: u16, nb_desc: u16, queue_conf: &RkBlkdevQueueConf) -> isize {
    let err = 0;
    assert!(!dev._data.is_null());

    if let RkBlkdevConfigured = &(*dev._data).state {
        return 22;
    }
    #[cfg(feature = "dispatcherthreads")]
    //TODO 确保我们没有第二次对这个队列进行初始化
    todo!()
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
    todo!()
}

///得到存有关于设备信息的容量信息，例如nb_of_sectors、sector_size等等
///
/// @返回值
///
///     一个指向类型*RkBlkdevCapabilities*的指针
///
#[inline]
fn rk_blkdev_capbilities(dev: &RkBlkdev) -> &RkBlkdevCap {
    todo!()
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
fn rk_blkdev_queue_intr_enable(dev: &RkBlkdev, queue_id: u16) -> isize {
    todo!()
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
fn rk_blkdev_queue_intr_disble(dev: RkBlkdev, queue_id: u16) -> isize {
    todo!()
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
    todo!()
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
fn rk_blkdev_status_test_set(status: isize, flag: isize) -> bool { todo!() }

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
fn rk_blkdev_status_test_unset(status: isize, flag: isize) -> bool { todo!() }

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
fn rk_blkdev_status_successful(status: isize) -> bool { todo!() }

/// 测试`rk_blkdev_submut_one`返回的状态是否表明操作需要被重试
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：操作应该被重试
/// - false：操作是成功的或者发生了错误
#[inline]
fn rk_blkdev_status_notready(status: isize) -> bool { todo!() }

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
fn rk_blkdev_status_more(status: isize) -> bool { todo!() }

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
    todo!()
}
/**
 * Used for sending a synchronous request.
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub struct RkBlkdevSyncIORequest{
    /* Request structure. */
    req:RkBlkreq,
    /* Semaphore used for waiting after the response is done. */
    s:RkSemaphore,
}
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn __sync_io_callback(req:&RkBlkreq,cookie_callback:*mut u8){
    todo!()
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
pub fn rk_blkdev_sync_io(dev: &RkBlkdev, queue_id: u16, op: RkBlkreqOp, sector: Sector, nb_sectors: Sector, buf: *mut u8) {
    todo!()
}
/*
 * Wrappers for uk_blkdev_sync_io
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkdev_sync_write(dev: &RkBlkdev, queue_id: u16, op: RkBlkreqOp, sector: Sector, nb_sectors: Sector, buf: *mut u8) {
    todo!()
}
/*
 * Wrappers for uk_blkdev_sync_io
 */
#[cfg(feature = "sync_io_blocked_waiting")]
pub fn rk_blkdev_sync_read(dev: &RkBlkdev, queue_id: u16, op: RkBlkreqOp, sector: Sector, nb_sectors: Sector, buf: *mut u8) {
    todo!()
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
    todo!()
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
fn rk_blkdev_queue_unconfigure(dev: &RkBlkdev, queue: u16) -> isize {
    todo!()
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
    todo!()
}




