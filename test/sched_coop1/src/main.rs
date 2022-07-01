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
use core::time::Duration;

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
        unsafe{set_times[CNT.fetch_add(1, SeqCst) as usize].fetch_add(1,SeqCst);}
        this_thread::r#yield();
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
        this_thread::r#yield();
    }

    for t in threads.iter_mut() {
        t.element.detach();
    }

    let time_begin = rkplat::time::monotonic_clock();
    this_thread::sleep_for(Duration::from_millis(1500));
    let time_end = rkplat::time::monotonic_clock();
    let duration = time_end-time_begin;
    assert!(duration>=Duration::from_millis(1400));
    assert!(duration<=Duration::from_millis(1600));

    for i in 0..5 {
        assert!(exited[i+1]);
        rksched::sched::destroy_thread(threads[i]);
    }

    for i in 0..150 {
        assert_eq!(set_times[i].load(SeqCst),1);
    }

    rkplat::println!("Test sched_coop1 passed!");
    0
}}
