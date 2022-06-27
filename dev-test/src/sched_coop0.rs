#![no_std]
#![no_main]

#[macro_use]
extern crate rkplat;
extern crate rkboot;
use core::sync::atomic::{AtomicI32,Ordering::SeqCst};
use alloc::{vec::Vec, format};
use rksched::this_thread;
use core::ptr::addr_of;
extern crate alloc;

static CNT: AtomicI32 = AtomicI32::new(0);

#[derive(Clone,Copy)]
struct ThreadData {
    id: usize,
}

fn thread_main(args: *mut u8) {
    let args = unsafe {&mut *(args as *mut ThreadData)};
    while CNT.load(SeqCst)<(args.id *30) as i32 {
        println!("thread #{}, cnt={}",args.id,CNT.fetch_add(1, SeqCst));
        this_thread::r#yield();
    }
    println!("thread #{} exit",args.id);
}

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let mut data = [ThreadData{id:0};5];
    let mut threads = Vec::new();
    for i in 0..5 {
        data[i].id = i+1;
        println!("创建线程 #{}",i+1);
        match rksched::sched::create_thread(format!("thread #{}",i+1).as_str(), rkalloc::get_default().unwrap(), 
            rksched::thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
            thread_main, addr_of!(data[i]) as *mut u8)
        {
            Ok(t) =>threads.push(t),
            Err(err) => panic!("无法创建线程: {:?}",err),
        }
        this_thread::r#yield();
    }

    for t in threads.iter_mut() {
        println!("等待线程 {}结束",t.element.name());
        this_thread::control_block().block_for_thread(t.element.as_ref());
        rksched::sched::destroy_thread(*t);
    }

    0
}
