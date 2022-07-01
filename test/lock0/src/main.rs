#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate rkplat;
extern crate rkboot;
extern crate alloc;
use alloc::{vec::Vec, format};
use rksched::this_thread;
use core::ptr::addr_of;
use rklock::{mutex::Mutex,semaphore::Semaphore};

static mut CNT: usize = 0;
static MUTEX: Mutex = Mutex::new();
static SEMAPHORE: [Semaphore; 6] = [Semaphore::new(0),Semaphore::new(0),Semaphore::new(0),Semaphore::new(0),Semaphore::new(0),Semaphore::new(0)];

#[derive(Clone,Copy)]
struct ThreadData {
    id: usize,
}

static mut set_times: [i32; 150] = [0;150];
static mut exited: [bool; 6] = [false; 6];

fn thread_main(args: *mut u8) {
    unsafe{
        let args = &mut *(args as *mut ThreadData);
        loop {
            let _lock = MUTEX.lock();
            if CNT >= args.id * 30 {break;}
            set_times[CNT]+=1;
            CNT+=1;
            this_thread::r#yield();
        }
        exited[args.id] = true;
        SEMAPHORE[args.id].signal();
    }
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

    for i in 1..=5 {
        SEMAPHORE[i].wait();
        assert!(exited[i]);
    }

    for i in 0..150 {
        assert_eq!(set_times[i],1);
    }

    for t in threads.iter_mut() {
        this_thread::control_block().block_for_thread(t.element.as_non_null());
    }

    rkplat::println!("Test lock0 passed!");
    0
}}
