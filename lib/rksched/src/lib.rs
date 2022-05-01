#![no_std]

use rkalloc::RKalloc;
use core::time::Duration;
//thread.h, wait.h, thread_attr.h, wait_types.h

////////////////////////////////////////////////////////////////
/// 等待队列结构体定义
////////////////////////////////////////////////////////////////

//等待队列条目结构体
pub struct RKwaitQEntry<'a, T> {
    waiting: i32,
    thread: &'a mut RKthread<'a, T>,
    next: &'a mut RKwaitQEntry<'a, T>,
}

impl<'a, T> RKwaitQEntry<'a, T> {
    //等待队列条目初始化
    fn init(&mut self, thread: &'a mut RKthread<'a, T>) {
        self.waiting = 0;
        self.thread = thread;
    }
}

//等待队列头结点结构体
pub struct RKwaitQ<'a, T> {
    //指向第一个等待条目的指针
    first: &'a mut RKwaitQEntry<'a, T>,
    //等待队列的长度
    length: usize,
}

impl<'a, T> RKwaitQ<'a, T> {
    //等待队列头结点初始化
    fn init(&mut self) {
        //调用尾队列相应函数初始化队列头结点
    }
    //判断队列是否为空
    fn is_empty(&self) -> bool {
        //调用尾队列相应函数检查队列是否为空

        true    //先临时返回true
    }
    //向队列中添加元素
    unsafe fn add(&mut self, entry: &mut RKwaitQEntry<T>) {
        if let 0 = (*entry).waiting {
            //调用尾队列相应函数将新的 entry 插入队列，并更新 self.length，然后waiting置1
            (*entry).waiting = 1;
        }
    }
    //从队列中移除元素
    unsafe fn remove(&mut self, entry: &mut RKwaitQEntry<T>) {
        if let 1 = (*entry).waiting {
            //调用尾队列相应函数移走队列中的一个元素，并用entry存储。然后更新 self.length，再让waiting置0
            (*entry).waiting = 0;
        }
    }
    //唤醒队列中的进程(wait.h 161~171)
    unsafe fn wake_up(&mut self) {
        //会调用平台层的代码
        //然后有一个 for 循环，大概是对队列中的所有线程逐个调用 RKthread::wake 函数
        //过程中还有一个标志 flags 的操作，具体功能尚且未知
    }
}

////////////////////////////////////////////////////////////////////////
/// 线程属性 thread_attr 的结构体定义
////////////////////////////////////////////////////////////////////////

//一些要用到的常量
const RK_THREAD_ATTR_PRIO_INVALID: i32 = -1;
const RK_THREAD_ATTR_PRIO_MIN: i32 = 0;
const RK_THREAD_ATTR_PRIO_MAX: i32 = 255;
const RK_THREAD_ATTR_PRIO_DEFAULT: i32 = 127;

const RK_THREAD_ATTR_TIMESLICE_NIL: u64 = 0;

//优先级类型 prio_t 为 i32
type PrioT = i32;

//状态类型
enum ThreadAttrState {
    Waitable,
    Detached,
}

pub struct RKthreadAttr {
    //True if thread should detach
    detached: bool,
    //Priority
    prio: PrioT,
    //Time slice in nanoseconds
    timeslice: Duration,
}

impl RKthreadAttr {
    fn init(&mut self) {
        self.detached = false;
        self.prio = -1;
        self.timeslice = Duration::from_nanos(RK_THREAD_ATTR_TIMESLICE_NIL);
    }

    fn finish(&self) {} //暂定为空函数

    fn set_detachstate(&mut self, state: ThreadAttrState) {
        match state {
            Waitable => self.detached = true,
            Detached => self.detached = false,
        };
    }

    fn get_detachstate(&self) -> ThreadAttrState {
        match self.detached {
            true => ThreadAttrState::Detached,
            false => ThreadAttrState::Waitable,
        }
    }

    fn set_prio(&mut self, prio: PrioT) {
        if self.prio >= RK_THREAD_ATTR_PRIO_MIN && self.prio <= RK_THREAD_ATTR_PRIO_MAX {
            self.prio = prio;
        }
    }

    fn get_prio(&self) -> PrioT {
        self.prio
    }

    fn set_timeslice(&mut self, timeslice: Duration) {
        //这里要用到平台层定义的RK_PLAT_TIME_TICK_NSEC——时间滴答长度，暂且定义为1ns
        debug_assert!(self.timeslice.as_nanos() >= 1);
        self.timeslice = timeslice;
    }

    fn get_timeslice(&self) -> Duration {
        self.timeslice
    }
}

////////////////////////////////////////////////////////////////////////
/// 线程 thread 的结构体定义
////////////////////////////////////////////////////////////////////////
pub struct RKthread<'a, T> {
    name: *const char,
    stack: *mut T,
    tls: *mut T,
    ctx: *mut T,
    // thread_list: /*尾队列类型*/
    flags: u32,
    wakeup_time: Duration,
    detached: bool,
    waiting_threads: RKwaitQ<'a, T>,
    sched: &'a mut RKsched<'a, T>,
    entry: fn(*mut T),
    arg: *mut T,
    prv: *mut T,
}

//一些要用到的常量
const RUNNABLE_FLAG: u32 = 0x00000001;
const EXITED_FLAG: u32 = 0x00000002;
const QUEUEABLE_FLAG: u32 = 0x00000004;

impl<'a, T> RKthread<'a, T> {
    ////////////////////////////////
    /// RKthread API 部分
    ////////////////////////////////
    pub fn create(name: *const char, function: fn(*mut T), data: *mut T) -> RKthread<'a, T> {
        // TODO
        panic!();
    }

    fn kill(&mut self) {
        // TODO
    }

    fn exit(&mut self) {
        // TODO
    }

    fn wait(&mut self) {
        // TODO
    }

    fn detached(&mut self) {
        // TODO
    }

    fn set_prio(&mut self, prio: PrioT) {
        // TODO
    }

    fn get_prio(&self) -> PrioT {
        // TODO

        //暂时返回 1
        1
    }

    fn set_timeslice(&mut self, timeslice: Duration) {
        // TODO
    }

    fn get_timeslice(&self) -> Duration {
        // TODO

        //暂时返回一个任意值
        Duration::from_nanos(1000)
    }

    ////////////////////////////
    /// 非 API 部分
    ////////////////////////////
    fn is_runnable(&self) -> bool {
        match self.flags & RUNNABLE_FLAG {
            0 => false,
            _ => true,
        }
    }
    fn set_runnable(&mut self) {
        self.flags |= RUNNABLE_FLAG;
    }
    fn clear_runnable(&mut self) {
        self.flags &= !RUNNABLE_FLAG;
    }

    fn is_exited(&self) -> bool {
        match self.flags & EXITED_FLAG {
            0 => false,
            _ => true,
        }
    }
    fn set_exited(&mut self) {
        self.flags |= EXITED_FLAG;
    }

    fn is_queueable(&self) -> bool {
        match self.flags & QUEUEABLE_FLAG {
            0 => false,
            _ => true,
        }
    }
    fn set_queueable(&mut self) {
        self.flags |= QUEUEABLE_FLAG;
    }
    fn clear_queueable(&mut self) {
        self.flags &= !QUEUEABLE_FLAG;
    }

    //线程初始化
    unsafe fn init(&mut self, /*cbs: *mut plat_ctx_callbacks, */allocator: &dyn RKalloc,
                   name: *const char, stack: *mut T, tls: *const char, entry: fn(*mut T), arg: *mut T) {
        // TODO
    }
    //线程完成
    unsafe fn finish(&mut self, allocator: &dyn RKalloc) {
        // TODO
    }

    fn block(&mut self) {
        // TODO
    }

    fn block_timeout(&self, nsec: Duration) {
        // TODO
    }

    fn wake(&mut self) {
        // TODO
    }

    //后面还有一些仿函数宏未完成(thread.h 145~167)
}

//返回当前线程的函数
fn thread_current<'a, T>() -> &'a mut RKthread<'a, T> {
    // TODO
    panic!();
}

////////////////////////////////////////////////////////////////////////
/// 调度器 sched 的结构体定义
////////////////////////////////////////////////////////////////////////

pub struct RKsched<'a, T> {
    threads_started: bool,
    idle: RKthread<'a, T>,
    // exited_threads: /* 尾队列头结点类型，其中的指针类型是 *mut RKthread */
    // plat_ctx_cbs: /* plat context callbacks 类型*/
    allocator: &'a dyn RKalloc,
    next: &'a mut RKsched<'a, T>,
    prv: *mut T,
    // 下面是函数“指针”部分，用于调用非抢占式调度器的相应函数，待确定是否加入
    // yield_fp: fn(s: &'a mut RKsched<'a, T>),

    // thread_add_fp: fn(s: &'a mut RKsched<'a, T>, t: &'a mut RKthread<'a, T>, attr: &'a mut RKthreadAttr),
    // thread_remove_fp: fn(s: &'a mut RKsched<'a, T>, t: &'a mut RKthread<'a, T>),
    // thread_blocked_fp: fn(s: &'a mut RKsched<'a, T>, t: &'a mut RKthread<'a, T>),
    // thread_woken_fp: fn(s: &'a mut RKsched<'a, T>, t: &'a mut RKthread<'a, T>),
}

impl<'a, T> RKsched<'a, T> {
    ////////////////////////////////
    /// RKsched API 部分
    ////////////////////////////////
    pub fn sched_yield() {
        // TODO
    }

    fn sched_start(&self) {
        // TODO sched.h 238
    }

    fn sched_started(&self) -> bool {
        self.threads_started
    }

    fn thread_add(&mut self, t: &'a mut RKthread<'a, T>, attr: &'a mut RKthreadAttr) {
        // TODO
    }

    fn thread_remove(&mut self, t: &'a mut RKthread<'a, T>) {
        // TODO
    }

    fn thread_blocked(&mut self, t: &'a mut RKthread<'a, T>) {
        // TODO
    }

    fn thread_woken(&mut self, t: &'a mut RKthread<'a, T>) {
        // TODO
    }

    fn thread_set_prio(&mut self, t: &'a mut RKthread<'a, T>, prio: PrioT) {
        // TODO
    }

    fn thread_get_prio(&self, t: &'a RKthread<'a, T>) -> PrioT {
        // TODO

        //暂时返回 1
        1
    }

    fn thread_set_timeslice(&mut self, t: &'a mut RKthread<'a, T>, tslice: Duration) {
        // TODO
    }

    fn thread_get_timeslice(&self, t: &'a RKthread<'a, T>) -> Duration {
        // TODO

        //暂时返回一个任意值
        Duration::from_nanos(1000)
    }

    //调度器使线程sleep的函数
    pub fn sched_thread_sleep(nsec: Duration) {
        // TODO
    }

    //调度器使线程退出的函数
    pub fn sched_thread_exit() {
        // TODO
    }

    ////////////////////////////////
    /// RKsched 非API 部分
    ////////////////////////////////
    pub fn sched_create(allocator: &'a dyn RKalloc, prv_size: usize) -> RKsched<'a, T> {
        // TODO
        panic!();
    }

    //待完成的调度器初始化函数，会将函数传入
    // fn init(&mut self, )

    fn idle_init(&mut self, stack: *mut T, function: fn(*mut T)) {}

    fn get_idle(&'a self) -> &'a mut RKthread<'a, T> {
        let idle: &'a mut RKthread<'a, T> = unsafe{&mut *(&self.idle as *const RKthread<'a,T> as *mut RKthread<'a,T>)};
        idle
    }

    fn thread_create(&mut self, name: *const char, attr: &mut RKthreadAttr, function: fn(*mut T), arg: *mut T) {
        // TODO
    }

    fn thread_destroy(&mut self, t: &'a mut RKthread<'a, T>) {
        // TODO
    }

    fn thread_kill(&mut self, t: &'a mut RKthread<'a, T>) {
        // TODO
    }

    fn thread_switch(&mut self, prev: &'a mut RKthread<'a, T>, next: &'a mut RKthread<'a, T>) {
        // TODO
    }
}

