#![no_std]


use core::borrow::BorrowMut;
use core::intrinsics::{ptr_guaranteed_ne, size_of};
use core::sync::atomic::{AtomicU16, Ordering};
use rkalloc::{alloc_type, RKalloc};
use rksched::RKsched;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use runikraft::list;

static BLKDEV_COUT: AtomicU16 = AtomicU16::new(0);



type Sector = usize;
type Atomic = u32;


pub fn ptriseer(ptr: u64) -> bool {
    if ptr <= 0 && ptr >= -512 {
        true
    } else {
        false
    }
}



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
    state: Atomic,
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
    fn rk_blkreq_ent_t(&self, cb_cookie: *mut u8);

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

///用来描述块设备的枚举类型
pub enum RkBlkdevState {
    RkBlkdevInvalid,
    RkBlkdevUnconfigured,
    RkBlkdevConfigured,
    RkBlkdevRunning,
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
pub struct RkBlkdevQueueConf<'a> {
    ///用于设备描述符环的分配器
    a: &'a dyn rkalloc::RKalloc,
    ///回调的参数指针
    callback_pointer: *mut u8,
    ///描述符的调度器
    s: &'a rksched::RKsched<'a, Self>,
}

impl RkBlkdevQueueConf<'static> {
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
    pub fn callback(dev: &mut RkBlkdev, queue_id: u16, argp: *mut u8) {todo!()}
}

pub trait RkBlkdevOps {
    ///得到初始设备容量的驱动程序回调类型
    fn get_info(&self, dev_info: *mut RkBlkdevInfo);
    ///配置块设备的驱动程序回调类型
    fn dev_configure(&self, conf: *mut RkBlkdevConf) -> isize;
    ///得到关于设备队列信息的驱动程序回调类型
    fn queue_get_info(&self, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize;
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

impl RkBlkdevOps for RkBlkreqOp{
    fn get_info(&self, dev_info: *mut RkBlkdevInfo) {
        todo!()
    }

    fn dev_configure(&self, conf: *mut RkBlkdevConf) -> isize {
        todo!()
    }

    fn queue_get_info(&self, queue_id: u16, q_info: *mut RkBlkdevQueueInfo) -> isize {
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
    ///触发器事件的信号量
    events: rk_semaphore,
    //TODO
    ///块设备的引用
    dev: &'a mut RkBlkdev<'a>,
    ///分配器线程
    dispatcher: *mut rk_thread,
    //TODO
    ///线程名称的引用
    dispatcher_name: *mut char,
    ///分配器的调度器
    dispatcher_s: &'a mut rksched::RKsched<'a, Self>,
}

impl RkBlkdevEventHandler<'static> {
    pub fn callback(dev: &mut RkBlkdev, queue_id: u16, argp: *mut u8) {todo!()}
}

///@内部
///librkblkdev中的和每个块设备相关的内部数据
pub struct RkBlkdevData<'a> {
    ///设备身份识别符
    id: u16,
    ///设备状态
    state: RkBlkdevState,
    ///每个队列的事件处理器
    queue_handler: [RkBlkdevEventHandler<'a>; CONFIG_LIBUKBLKDEV_MAXNBQUEUES],
    ///设备名称
    drv_name: &'a str,
    ///分配器
    a: &'a dyn rkalloc::RKalloc,
}

pub struct RkBlkdev<'a> {
    ///提交请求的函数指针
    ///用特征实现
    ///配置请求的函数指针
    ///用特征实现
    ///内部应用程序接口状态数据的指针
    _data: &'a mut RkBlkdevData<'a>,
    ///容量
    capabilities: RkBlkdevCap,
    ///驱动器回调函数
    dev_ops: &'a dyn RkBlkdevOps,
    ///队列指针（私有应用程序接口）
    _queue: [RkBlkdevQueue; CONFIG_LIBUKBLKDEV_MAXNBQUEUES],
    ///块设备队列入口
    _list_tqe_next: &'a mut RkBlkdev<'a>,
    _list_tqe_prev: &'a mut *mut RkBlkdev<'a>,
}

pub trait RkBlkdevT {
    ///向Runikraft块设备提交请求的驱动程序回调类型
    fn submit_one(&self, queue: *mut RkBlkdevQueue, req: *mut RkBlkreq) -> isize;
    ///完成一串Runikraft快设备 请求的驱动程序回调类型
    fn finish_reqs(&self, queue: RkBlkdevQueue) -> isize;

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
    /// - （-ENOMEM）：私有分配
    /// - （正值）：成功时的块设备的身份
    fn rk_blkdev_drv_register(&self, a: &dyn RKalloc, drv_name: &str) -> usize;

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
    fn rk_blkdev_drv_name_get(&self) -> Option<&str>;

    ///
    /// 返回一个块设备的当前状态
    ///
    /// @返回值
    /// - enum RkBlkdevState：当前设备状态
    ///
    fn rk_blkdev_state_get(&self) -> RkBlkdevState;

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
    fn rk_blkdev_get_info(&self, dev_info: &RkBlkdevInfo) -> isize;

    ///
    /// 为Runikraft块设备分配并建立一个队列
    /// 这个队列负责请求和响应
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
    fn rk_blkdev_queue_configure(&self, queue_id: u16, nb_desc: u16, queue_conf: &RkBlkdevQueueConf) -> isize;

    ///
    /// 开启块设备
    ///
    /// 设备开启步骤是最后一步，并且由设定卸载特性及开始传输、以及接收设备单元组成
    ///
    /// 一旦成功，被Runikraft块应用程序接口的所有基本函数都可以被调用
    ///
    /// @返回值
    /// - 0：成功，Runikraft块设备开启
    /// - <0：驱动程序设备开启函数的错误码
    ///
    fn rk_blkdev_start(&self) -> isize;

    ///得到存有关于设备信息的容量信息，例如nb_of_sectors、sector_size等等
    ///
    /// @返回值
    ///
    ///     一个指向类型*RkBlkdevCapabilities*的指针
    ///
    fn rk_blkdev_cap(&self) -> &RkBlkdevCap;

    ///允许队列中断
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
    fn rk_blkdev_queue_intr_enable(&self, queue_id: u16) -> isize;

    /// 禁止队列中断
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
    ///
    fn rk_blkdev_queue_intr_disble(&self, queue_id: u16) -> isize;

    /// 向设备发送一个异步非阻塞模式请求
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
    ///
    fn rk_blkdev_queue_submit_one(&self, queue_id: u16, req: &mut RkBlkreq) -> isize;

    /// 在队列和在目标队列上重新被许可的中断被重新许可之前，从它们那里得到回应
    ///
    /// @参数 queue_id
    ///
    /// 队列的指引
    ///
    /// @返回值
    /// - 0：成功
    /// - <0：当驱动程序返回错误的时候
    ///
    fn rk_blkdev_queue_finish_reqs(&self, queue_id: u16) -> isize;

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
    /// @返回值
    /// - 0：成功
    /// - <0：当驱动程序返回错误的时候
    fn rk_blkdev_stop(&self) -> isize;

    ///清空一个队列和它的Runikraft设备描述符
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
    fn rk_blkdev_queue_unconfigure(&self, queue: u16) -> isize;

    /// 关闭一个已经停止的Runikraft块设备
    ///
    /// 这个函数释放除被RK_BLKDEV_UNCONFIGURE状态使用的所有资源
    ///
    /// 设备可以通过调用rk_blkdev_configure来重新配置
    ///
    /// @返回值
    /// - 0：成功
    /// - <0：当驱动程序返回错误的时候
    fn rk_blkdev_unconfigure(&self) -> isize;

    fn rk_blkdev_id_get(&self) -> u16;
}

pub unsafe fn _alloc_data<'a>(a: &dyn RKalloc, blkdev_id: u16, drv_name: &str) -> *mut RkBlkdevData<'a> {
    let mut data: *mut RkBlkdevData = alloc_type::<RkBlkdevData>(a, RkBlkdevData);
    data.drv_name = drv_name;
    data.state = RkBlkdevState::RkBlkdevUnconfigured;
    data.a = a;
    //这仅仅会发生在我们设置设备身份的时候
    //在设备生命的剩余时间，这个身份是只读的
    data
}

impl RkBlkdevT for RkBlkdev {
    fn submit_one(&self, queue: *mut RkBlkdevQueue, req: *mut RkBlkreq) -> isize {
        todo!()
    }

    fn finish_reqs(&self, queue: RkBlkdevQueue) -> isize {
        todo!()
    }

    unsafe fn rk_blkdev_drv_register(&mut self, a: &dyn RKalloc, drv_name: &str) -> usize {
        assert!(self);

        //数据必须被取消分配
        assert_ne!(self._data);
        //断言必要的配置
        assert!(self.dev_ops);
        assert!(self.dev_ops.dev_configure());
        assert!(self.dev_ops.dev_start());
        assert!(self.dev_ops.queue_configure());
        assert!(self.dev_ops.get_info());
        assert!(self.dev_ops.queue_get_info());
        assert!(self.submit_one());
        assert!(self.finish_reqs());
        assert!(self.dev_ops.queue_intr_enable() && self.dev_ops.queue_intr_disable() || !self.dev_ops.queue_intr_enable() && !self.dev_ops.queue_intr_disable());
        assert!(self);
        self._data = _alloc_data(a, BLKDEV_COUT.load(Ordering::SeqCst), drv_name).borrow_mut();

        todo!()
    }

    fn rk_blkdev_drv_queue_event(&self, queue_id: i16) {
        todo!()
    }

    fn rk_blkdev_drv_unregister(&self) {
        todo!()
    }

    fn rk_blkdev_drv_name_get(&self) -> &str {
        assert!(self);
        assert!(self._data);
        self._data.drv_name
    }

    fn rk_blkdev_state_get(&self) -> &RkBlkdevState {
        assert!(self);
        assert!(self._data);
        &self._data.state
    }

    fn rk_blkdev_get_info(&self, dev_info: &RkBlkdevInfo) -> isize {
        todo!()
    }

    fn rk_blkdev_queue_configure(&self, queue_id: u16, nb_desc: u16, queue_conf: &RkBlkdevQueueConf) -> isize {
        todo!()
    }

    fn rk_blkdev_start(&self) -> isize {
        todo!()
    }

    fn rk_blkdev_cap(&self) -> &RkBlkdevCap {
        todo!()
    }

    fn rk_blkdev_queue_intr_enable(&self, queue_id: u16) -> isize {
        todo!()
    }

    fn rk_blkdev_queue_intr_disble(&self, queue_id: u16) -> isize {
        todo!()
    }

    fn rk_blkdev_queue_submit_one(&self, queue_id: u16, req: &mut RkBlkreq) -> isize {
        todo!()
    }

    fn rk_blkdev_queue_finish_reqs(&self, queue_id: u16) -> isize {
        todo!()
    }

    fn rk_blkdev_stop(&self) -> isize {
        todo!()
    }

    fn rk_blkdev_queue_unconfigure(&self, queue: u16) -> isize {
        todo!()
    }

    fn rk_blkdev_unconfigure(&self) -> isize {
        todo!()
    }

    fn rk_blkdev_id_get(&self) -> u16 {
        assert!(self);
        assert!(self._data);
        self._data.id
    }
}

/// 得到可得到的Runikraft块设备的数量
///
/// @返回值
///    - （usize）：块设备的数量
///
fn rk_blkdev_count() -> u16 {
    BLKDEV_COUT.load(Ordering::SeqCst)
}

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
fn rk_blkdev_get(id: usize) -> Option<&'static mut RkBlkdev<'static>> {
    let mut blkdev;

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
///
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
///
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
///
fn rk_blkdev_status_successful(status: isize) -> bool { todo!() }

/// 测试`rk_blkdev_submut_one`返回的状态是否表明操作需要被重试
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：操作应该被重试
/// - false：操作是成功的或者发生了错误
///
fn rk_blkdev_status_notready(status: isize) -> bool {todo!()}

/// 测试`rk_blkdev_submut_one`返回的状态是否表明了上一个操作可以被再一次成功重复
///
/// @参数 status
///
/// 返回状态（整型）
///
/// @返回值
/// - true：状态RK_BLKDEV_STATUS_MORE被设置
/// - false：操作是成功的或者发生了错误
///
fn rk_blkdev_status_more(status: isize) -> bool { todo!() }