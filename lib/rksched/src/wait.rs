// SPDX-License-Identifier: BSD-3-Clause
// wait.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// 等待队列*没有*修改自Unikraft

use crate::thread::Thread;
use runikraft::compat_list::{Stailq,StailqNode};
use rkplat::spinlock::SpinLock;
use core::ptr::NonNull;
use core::cell::UnsafeCell;

type Node = StailqNode<NonNull<Thread>>;

struct Internel {
    q: Stailq<NonNull<Thread>>,
    has_occurred: bool,
    last_occurred: bool,
}

/// 等待队列
/// - 资源的等待队列：在资源可用时，等待队列里的第一个线程会被唤醒
/// - 线程等待队列：在线程结束时，等待队列里的线程会被全部唤醒
/// 一个线程至多位于一个等待队列，在线程退出时，它必须主动将自己从等待队列中移除
pub struct WaitQ {
    data: UnsafeCell<Internel>,
    mutex: SpinLock,
}

unsafe impl Sync for WaitQ{}

// fn remove_first(q: &mut Stailq<ThreadRef>, alloc: &'static dyn RKalloc) -> Option<ThreadRef>{
//     // let _lock = self.mutex.lock();
//     q.pop_front().map(|node| {
//         unsafe {
//             let thread_ref = node.as_ref().element.clone();
//             dealloc_type(alloc, node.as_ptr());
//             thread_ref
//         }
//     })
// }

impl WaitQ {
    pub const fn new()->Self {
        Self { data: UnsafeCell::new(Internel{q: Stailq::new(), has_occurred: false, last_occurred: false}), mutex: SpinLock::new()}
    }

    // pub fn empty(&self) -> bool {
    //     self.q.is_empty()
    // }

    /// 如果没有待处理的事件，则将线程加入等待队列，并返回true；否则，不将线程加入等待队列，返回false
    #[must_use]
    pub fn add(&self, entry: NonNull<Node>) -> bool{
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        if data.last_occurred || data.has_occurred {
            data.has_occurred = false;
            false
        }
        else {
            data.q.push_back(entry);
            true
        }
    }

    /// 移除仍然等待在某个事件的线程，即使该事件并没有发生。当线程已经退出但是仍然在等待某个事件时，需要调用此函数。
    /// 调用者必须保证，事件确实在等待队列中。
    pub fn remove(&self, entry: NonNull<Thread>){
        let mut pos = None;
        let mut find = false;
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        for i in data.q.iter() {
            if i.element == entry {
                find = true;
                break;
            }
            pos = Some(i);
        }
        if find {
            if let Some(pos) = pos {
                pos.remove_after(Some(&mut data.q));
            }
            else {
                data.q.pop_front();
            }
        }
        else { panic!(); }
    }

    /// 如果等待队列非空，则唤醒等待队列中的所有线程；否则，标记事件为“待处理”（参见`add`）
    pub fn wakeup_all(&self) {
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        if data.q.is_empty() {
            data.has_occurred = true;
        }
        else {
            while !data.q.is_empty() {
                let mut t = data.q.pop_front().unwrap();
                unsafe{t.as_mut().element.as_mut().element.wake();}
            }
        }
    }

    /// 在事件最后一次发生后调用，唤醒等待队列中的所有线程，并且阻止新的线程等待
    pub fn wakeup_final(&self) {
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        data.last_occurred = true;
        while !data.q.is_empty() {
            let mut t = data.q.pop_front().unwrap();
            unsafe{t.as_mut().element.as_mut().element.wake();}
        }
    }

    /// 如果等待队列非空，则唤醒等待队列中的第一个线程；否则，标记事件为“待处理”（参见`add`）
    pub fn waitup_first(&self) {
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        if let Some(mut t) = data.q.pop_front() {
            unsafe{t.as_mut().element.as_mut().element.wake();}
        }
        else {
            data.has_occurred = true;
        }
    }
}

impl Drop for WaitQ {
    fn drop(&mut self) {
        let _lock = self.mutex.lock();
        let data = unsafe{&mut *self.data.get()};
        assert!(data.last_occurred,"Please manually call wakeup_final() before dropping waiting queue.");
    }
}
