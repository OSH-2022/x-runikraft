// SPDX-License-Identifier: BSD-3-Clause
// rkmbox/lib.rs

// Authors: Simon Kuenzer <simon.kuenzer@neclab.eu>
//          蓝俊玮 <ljw13@mail.ustc.edu.cn>

// Copyright (c) 2018, NEC Europe Ltd., NEC Corporation.
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
#![no_std]

use runikraft::errno::Errno;
use core::cell::UnsafeCell;
use core::mem::{size_of, align_of};
use core::sync::atomic::{AtomicUsize,Ordering::SeqCst};
use core::time::Duration;
use rkalloc::*;
use rklock::Semaphore;

pub struct Mbox<'a, T> {
    len: usize,
    readsem: Semaphore,
    readpos: AtomicUsize,
    writesem: Semaphore,
    writepos: AtomicUsize,
    msgs: UnsafeCell<&'a mut [Option<T>]>,
    alloc: &'a dyn RKalloc,
}

unsafe impl<T> Sync for Mbox<'_,T> {}

impl<'a,T> Mbox<'a,T> {
    pub fn new( size: usize, a: &'a dyn RKalloc) -> Option<Self>{
        let msgs_data = unsafe{a.alloc_zeroed((size+1)*size_of::<T>(),align_of::<T>()) as *mut Option<T>};
        if msgs_data.is_null() {
            return None;
        }
        let msgs = unsafe{core::slice::from_raw_parts_mut(msgs_data, size+1)};
        Some(Self {
            len: size+1,
            readsem: Semaphore::new(0),
            writesem: Semaphore::new(size as i32),
            readpos: AtomicUsize::new(0),
            writepos: AtomicUsize::new(0),
            alloc: a,
            msgs: UnsafeCell::new(msgs),
        })
    }

    fn do_mbox_recv(& self) -> T {
        let ret = unsafe{(*self.msgs.get())[self.readpos.fetch_update(SeqCst, SeqCst, 
            |x| {
                if x+1 != self.len {
                    Some(x+1)
                }
                else {
                    Some(0)
                }
            }).unwrap()].take()};

        self.writesem.signal();

        return ret.unwrap();
    }

    fn do_mbox_post(&self, msg: T) {
        unsafe{(*self.msgs.get())[self.writepos.fetch_update(SeqCst, SeqCst, 
            |x| {
                if x+1 != self.len {
                    Some(x+1)
                }
                else {
                    Some(0)
                }
            }).unwrap()]=Some(msg);}

        self.readsem.signal();
    }

    pub fn mbox_post(&self, msg: T) {
        self.writesem.wait();
        self.do_mbox_post(msg);
    }

    pub fn mbox_post_try(&self, msg: T) -> Result<(),Errno> {
        if !(self.writesem.try_wait()) {
            return Err(Errno::NoBufS);
        }

        self.do_mbox_post(msg);
        Ok(())
    }

    pub fn mbox_post_to(&self, msg: T, duration: Duration) -> Result<(),Errno> {
        if !(self.writesem.wait_for(duration)) {
            return Err(Errno::NoBufS);
        }

        self.do_mbox_post(msg);
        Ok(())
    }

    pub fn mbox_recv(&self) -> T{
        self.readsem.wait();
        self.do_mbox_recv()
    }

    pub fn mbox_recv_try(&self) -> Result<T,Errno> {
        if !(self.readsem.try_wait()) {
            Err(Errno::NoBufS)
        }
        else {
            Ok(self.do_mbox_recv())
        }
    }

    pub fn mbox_recv_to(&self, duration: Duration) -> Result<T,Errno> {
        if !(self.readsem.wait_for(duration)) {
            Err(Errno::NoBufS)
        }
        else {
            Ok(self.do_mbox_recv())
        }
    }
}

impl<T> Drop for Mbox<'_,T> {
    fn drop(&mut self) {
        unsafe {
            self.alloc.dealloc(self.msgs.get_mut().as_ptr() as *mut u8, self.msgs.get_mut().len()*size_of::<T>(), align_of::<T>());
        }
    }
}
