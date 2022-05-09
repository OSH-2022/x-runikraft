#![no_std]

use runikraft::list::Tailq;
use crate::blkreq::{RkBlkreq, RkBlkreqOp};

pub struct RkBlkdev<'a> {
    ///提交请求的函数指针
    ///用特征实现
    ///配置请求的函数指针
    ///用特征实现
    ///内部应用程序接口状态数据的指针
    pub(crate) _data: *mut RkBlkdevData<'a>,
    ///容量
    capabilities: RkBlkdevCap,
    ///驱动器回调函数
    pub(crate) dev_ops: &'a dyn RkBlkdevOps,
    ///队列指针（私有应用程序接口）
    _queue: [RkBlkdevQueue; 16],
    ///块设备队列入口
    _list_tqe_next: &'a mut RkBlkdev<'a>,
    _list_tqe_prev: &'a mut *mut RkBlkdev<'a>,
}

impl  RkBlkdevT {
    ///向Runikraft块设备提交请求的驱动程序回调类型
    pub fn submit_one(&self, queue: *mut RkBlkdevQueue, req: *mut RkBlkreq) -> isize{
        todo!()
    }
    ///完成一串Runikraft快设备 请求的驱动程序回调类型
    pub fn finish_reqs(&self, queue: RkBlkdevQueue) -> isize{
        todo!()
    }

}
static mut RK_BLKDEV_LIST: Option<Tailq<RkBlkdev>> = None;

///用来描述块设备的枚举类型
pub enum RkBlkdevState {
    RkBlkdevInvalid,
    RkBlkdevUnconfigured,
    RkBlkdevConfigured,
    RkBlkdevRunning,
}

///用来配置Runikraft块设备的结构体
pub struct RkBlkdevConf {
    pub(crate) nb_queues: u16,
}

///用来在交涉前描述块设备容量的结构体
pub struct RkBlkdevInfo {
    ///支持排队设备的最大数量
    pub(crate) max_queues: u16,
}

///用来描述设备描述符环限制的结构体
pub struct RkBlkdevQueueInfo {
    ///描述符的最大允许数量
    nb_max: u16,
    ///描述符的最小允许数量
    nb_min: u16,
    ///该数字需要是nb_align的倍数
    nb_align: u16,
    ///该数字需要是2的幂
    nb_is_power_of_two: isize,
}

/**
 * Queue Structure used for both requests and responses.
 * This is private to the drivers.
 * In the API, this structure is used only for type checking.
 */
pub struct RkBlkdevQueue {}

impl RkBlkdevQueueConf {
    ///用于队列事件回调的函数类型
    ///
    ///@参数 dev
    ///
    ///	Runikraft块设备
    ///
    ///@参数 queue
    ///
    ///	事件发生的Runikraft块设备的队列
    ///
    ///@参数 argp
    ///
    ///	可以在回调登记被定义的额外参数
    ///
    ///注意：为了处理接收到的响应，应该调用dev的finish_reqs方法
    ///
    pub fn callback(&self, dev: &mut RkBlkdev, queue_id: u16, argp: *mut u8) { todo!() }
}

///用于配置Runikraft块设备队列的结构体
pub struct RkBlkdevQueueConf<'a> {
    ///用于设备描述符环的分配器
    a: &'a dyn rkalloc::RKalloc,
    ///回调的参数指针
    callback_cookie: *mut u8,
    #[cfg(feature = "dispatcherthreads")]
    ///描述符的调度器
    s: &'a rksched::RKsched<'a>,
}

/**
 * Status code flags returned queue_submit_one function
 */
/** Successful operation. */
static RK_BLKDEV_STATUS_SUCCESS: i32 = 0x1;
/**
 * More room available for operation (e.g., still space on queue for sending
 * a request.
 */
static RK_BLKDEV_STATUS_MORE: i32 = 0x2;

impl RkBlkdevOps for RkBlkreqOp {
    fn get_info(&self, dev_info: &RkBlkdevInfo) {
        todo!()
    }

    fn dev_configure(&self, conf: &RkBlkdevConf) -> isize {
        todo!()
    }

    fn queue_get_info(&self, dev: &RkBlkdev<'_>, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize {
        todo!()
    }

    fn queue_configure(&self, queue_id: u16, nb_desc: u16, queue_conf: *mut RkBlkdevQueueConf) -> *mut RkBlkdevQueue {
        todo!()
    }

    fn dev_start(&self) -> isize {
        todo!()
    }

    fn dev_stop(&self) -> isize {
        todo!()
    }

    fn queue_intr_enable(&self, queue: *mut RkBlkdevQueue) -> bool {
        todo!()
    }

    fn queue_intr_disable(&self, queue: *mut RkBlkdevQueue) -> bool {
        todo!()
    }

    fn queue_unconfigure(&self, queue: *mut RkBlkdevQueue) -> isize {
        todo!()
    }

    fn dev_unconfigure(&self) -> isize {
        todo!()
    }
}

pub trait RkBlkdevOps {
    ///得到初始设备容量的驱动程序回调类型
    fn get_info(&self, dev_info: &RkBlkdevInfo);
    ///配置块设备的驱动程序回调类型
    fn dev_configure(&self, conf: &RkBlkdevConf) -> isize;
    ///得到关于设备队列信息的驱动程序回调类型
    fn queue_get_info(&self, dev: &RkBlkdev, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize;
    ///建立Runikraft块设备队列的驱动程序回调类型
    fn queue_configure(&self, queue_id: u16, nb_desc: u16, queue_conf: *mut RkBlkdevQueueConf) -> *mut RkBlkdevQueue;
    ///开启已配置的Runikraft块设备的驱动程序回调类型
    fn dev_start(&self) -> isize;
    ///停止Runikraft块设备的驱动程序回调类型
    fn dev_stop(&self) -> isize;
    ///为一个在Runikraft块设备的队列启用中断的驱动程序回调类型
    fn queue_intr_enable(&self, queue: *mut RkBlkdevQueue) -> bool;
    ///为一个在Runikraft块设备的队列禁用中断的驱动程序回调类型
    fn queue_intr_disable(&self, queue: *mut RkBlkdevQueue) -> bool;
    ///释放Runikraft块设备队列的驱动程序回调类型
    fn queue_unconfigure(&self, queue: *mut RkBlkdevQueue) -> isize;
    ///取消配置块设备的驱动程序回调类型
    fn dev_unconfigure(&self) -> isize;
}

///设备信息
pub struct RkBlkdevCap {
    ///扇区数量
    sectors: Sector,
    ///扇区大小
    ssize: usize,
    ///访问模式（只读（O_RDONLY）、读写（RDWR）、只写（O_WRONLY））
    mode: isize,
    ///一次操作最多支持的扇区数量
    max_sectors_per_req: Sector,
    ///用于从现在开始的请求的数据对齐方式（字节数）
    ioalign: u16,
}

///@内部
///
///事件处理程序配置
pub struct RkBlkdevEventHandler<'a> {
    //回调
    //使用静态方法实现
    ///回调的参数
    cookie: *mut u8,
    #[cfg(feature = "dispatcherthreads")]
    ///触发器事件的信号量
    //TODO events: rk_semaphore,
    #[cfg(feature = "dispatcherthreads")]
    ///块设备的引用
    dev: *mut RkBlkdev<'a>,
    #[cfg(feature = "dispatcherthreads")]
    ///分配器线程
    //TODO dispatcher: *mut rk_thread,
    #[cfg(feature = "dispatcherthreads")]
    ///线程名称的引用
    dispatcher_name: *mut char,
    #[cfg(feature = "dispatcherthreads")]
    ///分配器的调度器
    dispatcher_s: *mut rksched::RKsched<'a>,
}

impl<'a> RkBlkdevEventHandler<'a> {
    pub fn callback(dev: &mut RkBlkdev, queue_id: u16, argp: *mut u8) { todo!() }
}

///@内部
///librkblkdev中的和每个块设备相关的内部数据
pub struct RkBlkdevData<'a> {
    ///设备身份识别符
    pub(crate) id: u16,
    ///设备状态
    pub(crate) state: RkBlkdevState,
    ///每个队列的事件处理器
    queue_handler: [RkBlkdevEventHandler<'a>; 16],
    ///设备名称
    pub(crate) drv_name: &'a str,
    ///分配器
    a: &'a dyn rkalloc::RKalloc,
}