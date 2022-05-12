#![no_std]

extern crate alloc;

use rkalloc::RKalloc;
use runikraft::list::Tailq;
use crate::blkdev_core::{RkBlkdev, RkBlkdevData, RkBlkdevEventHandler};

mod blkdev;
mod blkdev_core;
mod blkdev_driver;
mod blkreq;

static mut RK_BLKDEV_LIST:Tailq<RkBlkdev>=Tailq;
static mut BLKDEV_COUNT: Option<i16> = None;
const CONFIG_LIBUKBLKDEV_MAXNBQUEUES: u16 =  core::u16::from_str(env!("PATH"));

pub unsafe fn _alloc_data<'a>(a: &'a (dyn RKalloc + 'a), blkdev_id: u16, drv_name: &'a str) -> *mut RkBlkdevData<'a> {
    //TODO let mut data: *mut RkBlkdevData = alloc_type::<RkBlkdevData>(a, ());
    //这仅仅会发生在我们设置设备身份的时候
    //在设备生命的剩余时间，这个身份是只读的
    todo!()
}

#[cfg(feature = "dispatcherthreads")]
pub fn _dispatcher(args:*mut u8) {
    let handler=RkBlkdevEventHandler;
    loop{
        //TODO uk_semaphore_down(&handler->events);
        handler.callback(handler.dev,handler.queue_id,handler,cookie);
    }

}

#[cfg(feature = "dispatcherthreads")]
pub fn _create_event_handler() {
    todo!()
}

#[cfg(feature = "dispatcherthreads")]
pub fn _destory_event_handler() {
    todo!()
}

pub fn ptriseer(ptr: i64) -> bool {
    if ptr <= 0 && ptr >= -512 {
        true
    } else {
        false
    }
}