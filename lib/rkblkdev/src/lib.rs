#![no_std]


use rkalloc::RKalloc;

type Sector = usize;

//blkreq.h

///支持的操作
pub enum RkBlkreqOp {
    ///读操作
    RkBlkreqRead(u8),
    ///写操作
    RkBlkreqWrite(u8),
    ///冲洗易变的写缓存
    RkBlkreqFflush(u8),
}

///用于向设备发送请求
pub struct RkBlkreq {
    //输入成员
    ///操作类型
    operation: RkBlkreqOp,
    ///操作开始的起始扇区
    start_sector: Sector,
    ///扇区数量的大小
    nb_sectors: Sector,
    ///指向数据的指针
    aio_buf: *mut u8,
    ///回复的请求的参数
    cb_cookie: *mut u8,

    //输出成员
    ///请求的状态：完成/未完成
    state: __atomic,
    ///操作状态的结果（错误返回负值）
    result: isize,
}

///操作状态
pub enum RkBlkreqState {
    RkBlkreqFinished(bool),
    RkBlkreqUnfinished(bool),
}

pub trait RkBlkreqEvent {
    ///用于执行一个响应后的请求
    ///@参数 cookie_callback
    ///	由用户在递交请求时设定的可选参数
    ///
    fn rk_blkreq_eent_t(&self, cb_cookie: *mut u8);

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
    fn rk_blkreq_init(&self, op: RkBlkreqOp, start: Sector, nb_sectors: Sector, aio_buf: *mut u8, cb_cookie: *mut u8);

    ///检查请求是否结束
    fn rk_blkreg_is_done(&self) -> bool;


    ///当结束时设置一个请求
    fn rk_blkreq_finished(&self);
}

//blkdev_core.h


///用来描述块设备的枚举类型
pub enum RkBlkdevState {
    RkBlkdevInvalid(u8),
    RkBlkdevUnconfigured(u8),
    RkBlkdevConfigured(u8),
    RkBlkdevRunning(u8),
}

///用来配置Runikraft块设备的结构体
pub struct RkBlkdevConf {
    nb_queues: u16,
}

///用来在交涉前描述块设备容量的结构体
pub struct RkBlkdevInfo {
    ///支持排队设备的最大数量
    max_queues: u16,
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

///用于配置Runikraft块设备队列的结构体
pub struct RkBlkdevQueue {}

///用于配置Runikraft块设备队列的结构体
pub struct RkBlkdevQueueConf {
    ///用于设备描述符环的分配器
    a: *mut rk_alloc,
    ///TODO
    ///回调的参数指针
    callback_pointer: *mut u8,
    ///描述符的调度器
    s: *mut rk_sched,
    ///TODO
}

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
    pub fn callback(dev: * uk_blkdev, queue_id: u16, argp: *mut u8) {}
}

pub trait RkBlkdevOps {
    ///得到初始设备容量的驱动程序回调类型
    fn get_info(&self, dev_info: *mut RkBlkdevInfo);
    ///配置块设备的驱动程序回调类型
    fn dev_configure(&self, conf:* RkBlkdevConf) -> isize;
    ///得到关于设备队列信息的驱动程序回调类型
    fn queue_get_info(&self, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize;
    ///建立Runikraft块设备队列的驱动程序回调类型
    fn queue_configure(&self, queue_id: u16, nb_desc: u16, queue_conf: * RkBlkdevQueueConf) -> *mut RkBlkdevQueue;
    ///开启已配置的Runikraft块设备的驱动程序回调类型
    fn dev_start(&self) -> isize;
    ///停止Runikraft块设备的驱动程序回调类型
    fn dev_stop(&self) -> isize;
    ///为一个在Runikraft块设备的队列启用中断的驱动程序回调类型
    fn queue_intr_enable(&self, queue: *mut RkBlkdevQueue) -> isize;
    ///为一个在Runikraft块设备的队列禁用中断的驱动程序回调类型
    fn queue_intr_disable(&self, queue: *mut RkBlkdevQueue) -> isize;
    ///释放Runikraft块设备队列的驱动程序回调类型
    fn queue_unconfigure(&self, queue: *mut RkBlkdevQueue) -> isize;
    ///取消配置块设备的驱动程序回调类型
    fn dev_unconfigure(&self) -> isize;
}

///设备信息
pub struct RkBlkdevCap {
    ///扇区数量
    sectors: Sector,
    //TODO
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
struct RkBlkdevEventHandler {
    //回调
    //使用静态方法实现
    ///回调的参数
    cookie: *mut u8,
    ///触发器事件的信号量
    events: rk_semaphore,
    ///TODO
    ///块设备的引用
    dev: *mut RkBlkdev,
    ///TODO
    ///分配器线程
    dispatcher: *mut rk_thread,
    ///TODO
    ///线程名称的引用
    dispatcher_name: *mut char,
    ///分配器的调度器
    dispatcher_s: *mut rk_sched,                    //TODO
}

impl RkBlkdevEventHandler {
    pub fn callback(dev: * uk_blkdev, queue_id: u16, argp: *mut u8) {}
}

///@内部
///librkblkdev中的和每个块设备相关的内部数据
pub struct RkBlkdevData {
    ///设备身份识别符
    id: u16,
    ///设备状态
    state: uk_blkdev_state,
    ///每个队列的事件处理器
    queue_handler: [uk_blkdev_event_handler; CONFIG_LIBUKBLKDEV_MAXNBQUEUES],
    ///设备名称
    drv_name: * char,
    ///分配器
    a: *mut uk_alloc,
}

pub struct RkBlkdev {
    ///提交请求的函数指针
    ///用特征实现
    ///配置请求的函数指针
    ///用特征实现
    ///内部应用程序接口状态数据的指针
    _data: rk_blkdev_data,
    ///容量
    capabilities: rk_blkdev_cap,
    ///驱动器回调函数
    dev_ops: rk_blkdev_ops,
    ///队列指针（私有应用程序接口）
    _queue: [uk_blkdev_queue; CONFIG_LIBUKBLKDEV_MAXNBQUEUES],
    ///块设备队列入口
    _list_tqe_next: *mut RkBlkdev,
    _list_tqe_prev: *mut *mut RkBlkdev,
}

pub trait RkBlkdevT {
    ///向Runikraft块设备提交请求的驱动程序回调类型
    fn submit_one(&self, queue: *mut rk_blkdev_queue, req: *mut rk_blkreq) -> isize;
    ///完成一串Runikraft快设备 请求的驱动程序回调类型
    fn finish_reqs(&self, queue: rk_blkdev_req) -> isize;

//blkdev_driver.h


    /// 向设备链表增加Runikraft块设备
    /// 一旦驱动增加了新找到的设备，这个函数就应该被调用
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
    ///     - （-ENOMEM）：私有分配
    ///     - （正值）：成功时的块设备的身份
    fn rk_blkdev_drv_register(&self, a: &dyn RKalloc, drv_name: * u8) -> usize;
    ///TODO

    /// 把一个队列事件向应用程序接口用户前移
    /// 可以（并且应该）在设备中断的上下文中调用
    ///
    /// @参数 queue_id
    ///
    ///    接收事件相应的队列身份
    fn rk_blkdev_drv_queue_event(&self, queue_id: i16);

    /// 释放给Runikraft块设备的数据
    /// 把设备从列表中移除
    fn rk_blkdev_drv_unregister(&self);

    /// 返回块设备的身份
    ///
    /// @参数 id
    ///     要配置的Runikraft块设备的识别符
    ///
    /// @返回值
    /// - None：如果没有定义名称
    /// - &str：如果名称可得到，返回字符串的引用
    ///
    fn rk_blkdev_drv_name_get(&self) -> Option<&str>;//TODO

    ///
    /// 返回一个块设备的当前状态
    ///
    /// @返回值
    /// - enum RkBlkdevState：当前设备状态
    ///
    fn rk_blkdev_state_get(&self) -> RkblkdevState;

    ///
    /// 询问设备容量
    /// 信息对设备初始化有用（例如可支持队列得的最大值）
    ///
    /// @参数 dev_info
    ///
    ///     一个指向将装有块设备上下文信息的*RkBlkdevInfo*类型的指针
    ///
    /// @返回值
    ///
    /// - 0：成功
    /// - <0：驱动器错误
    ///
    fn rk_blkdev_get_info(&self, dev_info: &rk_blkdev_info);
}


//blkdev.h


/// 得到可得到的Runikraft块设备的数量
///
/// @返回值
///    - （usize）：块设备的数量
///
fn rk_blkdev_count() -> usize { 0 }          //TODO

///
/// 得到一个Runikraft块设备的引用，基于它的身份
/// 这个引用应该被应用保存并用于后续的应用程序接口调用
///
/// @参数 id
///
///     要配置的Runikraft块设备的识别符
///
/// @返回值
/// - None：在列表中没有找到设备
/// - Some(&mut RkBlkdev)：将传递给应用程序接口的引用
///
fn rk_blkdev_get(id: usize) -> Option<&mut RkBlkdev> { None } //TODO

///
///
