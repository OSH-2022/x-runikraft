use super::{RKsched, wait::RKwaitQ};
use runikraft::list::Tailq;
use core::{debug_assert, panic};
use rkalloc::RKalloc;
use core::time::Duration;
use rkplat::thread::Context;

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
pub type PrioT = i32;

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

impl Default for RKthreadAttr {
    fn default() -> Self {
        Self {
            detached: false,
            prio: -1,
            timeslice: Duration::default(),
        }
    }
}

impl RKthreadAttr {
    pub fn new(&mut self, detached: bool, prio: i32, timeslice: Duration) -> Self {
        Self {
            detached,
            prio,
            timeslice,
        }
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

impl Clone for RKthreadAttr {
    fn clone(&self) -> Self {
        Self {
            detached: self.detached,
            prio: self.prio,
            timeslice: self.timeslice.clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////
/// 线程 thread 的结构体定义
////////////////////////////////////////////////////////////////////////
pub type RKthreadList<'a> = Tailq<'a, RKthread<'a>>;

pub struct RKthread<'a> {
    name: &'a str,
    pub attr: RKthreadAttr,
    stack: *mut u8,
    tls: *mut u8,
    pub ctx: *mut Context,
    thread_list: RKthreadList<'a>,
    flags: u32,
    pub wakeup_time: Duration,
    detached: bool,
    waiting_threads: RKwaitQ<'a>,
    sched: &'a dyn RKsched<'a>,
    entry: fn(*mut u8),
    arg: *mut u8,
    prv: *mut u8,
}

//一些要用到的常量
const RUNNABLE_FLAG: u32 = 0x00000001;
const EXITED_FLAG: u32 = 0x00000002;
const QUEUEABLE_FLAG: u32 = 0x00000004;

impl<'a> RKthread<'a> {
    ////////////////////////////////
    /// RKthread API 部分
    ////////////////////////////////
    pub fn new(name: *const char, function: fn(*mut u8), data: *mut u8) -> Self {
        // TODO
        panic!();
    }

    pub fn kill(&mut self) {
        // TODO
    }

    pub fn exit(&mut self) {
        // TODO
    }

    pub fn wait(&mut self) {
        // TODO
    }

    pub fn detached(&mut self) {
        // TODO
    }

    pub fn set_prio(&mut self, prio: PrioT) {
        // TODO
    }

    pub fn get_prio(&self) -> PrioT {
        // TODO

        //暂时返回 1
        1
    }

    pub fn set_timeslice(&mut self, timeslice: Duration) {
        // TODO
    }

    pub fn get_timeslice(&self) -> Duration {
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
    pub fn set_runnable(&mut self) {
        self.flags |= RUNNABLE_FLAG;
    }
    pub fn clear_runnable(&mut self) {
        self.flags &= !RUNNABLE_FLAG;
    }

    pub fn is_exited(&self) -> bool {
        match self.flags & EXITED_FLAG {
            0 => false,
            _ => true,
        }
    }
    pub fn set_exited(&mut self) {
        self.flags |= EXITED_FLAG;
    }

    pub fn is_queueable(&self) -> bool {
        match self.flags & QUEUEABLE_FLAG {
            0 => false,
            _ => true,
        }
    }
    pub fn set_queueable(&mut self) {
        self.flags |= QUEUEABLE_FLAG;
    }
    pub fn clear_queueable(&mut self) {
        self.flags &= !QUEUEABLE_FLAG;
    }

    //线程初始化
    unsafe fn init(&mut self, /*cbs: *mut plat_ctx_callbacks, */allocator: &'a dyn RKalloc,
                   name: &'a str, stack: *mut u8, tls: *const char, entry: fn(*mut u8), arg: *mut u8) {
        // TODO
    }
    //线程完成
    unsafe fn finish(&mut self, allocator: &'a dyn RKalloc) {
        // TODO
    }

    fn block(&mut self) {
        // TODO
    }

    pub fn block_timeout(&self, nsec: Duration) {
        // TODO
    }

    fn wake(&mut self) {
        // TODO
    }

    //后面还有一些仿函数宏未完成(thread.h 145~167)
}


//返回当前线程的函数
pub fn thread_current<'a>() -> *mut RKthread<'a> {
    todo!()//needs the function about stack(operations related to the bottom layer)
}