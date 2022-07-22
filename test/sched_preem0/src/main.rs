#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate rkplat;
extern crate rkboot;
extern crate alloc;
use core::sync::atomic::{AtomicI32,Ordering::SeqCst};
use alloc::{vec::Vec, format};
use rksched::this_thread;
use core::ptr::addr_of;

static CNT: AtomicI32 = AtomicI32::new(0);

#[derive(Clone,Copy)]
struct ThreadData {
    id: usize,
}

static mut set_times: [AtomicI32; 150] = [
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
    AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),AtomicI32::new(0),
];
static mut exited: [bool; 6] = [false; 6];

fn thread_main(args: *mut u8) {
    let args = unsafe {&mut *(args as *mut ThreadData)};
    while CNT.load(SeqCst)<(args.id *30) as i32 {
        unsafe{
            let cnt = CNT.fetch_add(1, SeqCst) as usize;
            set_times[cnt].fetch_add(1,SeqCst);
        }
    }
    unsafe {exited[args.id] = true; }
}

#[no_mangle]
fn main(_args: &mut [&str])->i32 { unsafe{
    let mut data = [ThreadData{id:0};5];
    let mut threads = Vec::new();
    for i in 0..5 {
        data[i].id = i+1;
        match rksched::sched::create_thread(format!("thread #{}",i+1).as_str(), rkalloc::get_default().unwrap(), 
            rksched::thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
            thread_main, addr_of!(data[i]) as *mut u8)
        {
            Ok(t) =>threads.push(t),
            Err(err) => panic!("fail to create thread: {:?}",err),
        }
    }

    for i in 0..5 {
        this_thread::control_block().block_for_thread(threads[i].element.as_non_null());
        rksched::sched::destroy_thread(threads[i]);
        assert!(exited[i+1]);
    }

    for i in 0..150 {
        assert_eq!(set_times[i].load(SeqCst),1);
    }

    rkplat::println!("Test sched_preem0 passed!");
    0
}}
