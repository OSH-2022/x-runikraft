use super::thread::RKthread;
use runikraft::list::STailq;

/// 等待队列条目结构体
pub struct RKwaitQEntry<'a> {
    waiting: i32,
    thread: &'a mut RKthread<'a>,
}

impl<'a> RKwaitQEntry<'a> {
    //等待队列条目初始化
    fn new(&mut self, thread: &'a mut RKthread<'a>) -> Self {
        Self {
            waiting: 0,
            thread,
        }
    }
}

/// 等待队列头结点结构体
pub type RKwaitQ<'a> = STailq<'a, RKwaitQEntry<'a>>;