// SPDX-License-Identifier: BSD-3-Clause
// rkboot/lib.rs

// Authors: 张子辰 <zichen350@gmail.com>

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
use core::time::Duration;
use core::{slice,str};
use core::mem::{align_of, size_of};
use core::ptr::{addr_of, null_mut};
use rkalloc::RKalloc;
use rkplat::{irq,time,bootstrap,device, lcpu};
#[cfg(feature="have_scheduler")]
use rksched::RKsched;
use runikraft::align_as;

const HEAP_SIZE: usize = 1<<20;

static mut HEAP:align_as::A4096<[u8;HEAP_SIZE]> = align_as::A4096::new([0;HEAP_SIZE]);

extern "Rust" {
    fn main(args: &mut [&str])->i32;
}

#[no_mangle]
pub unsafe extern "C" fn rkplat_entry(argc: i32, argv: *mut *mut u8) -> ! {
    #[cfg(feature="alloc_buddy")]
    let a = rkalloc_buddy::RKallocBuddy::new(HEAP.data.as_mut_ptr(), HEAP.data.len());

    rkalloc::register(addr_of!(a));
    rkalloc::register_state(addr_of!(a));

    #[cfg(all(feature="alloc_buddy"))]
    rkalloc::register_ext(addr_of!(a));
    
    
    // 把C风格的arguments转换成Rust风格的arguments
    let args_heap = a.alloc(argc as usize*size_of::<usize>(), align_of::<usize>());
    let args = slice::from_raw_parts_mut(args_heap as *mut &str, argc as usize);
    for i in 0..(argc as usize) {
        let s = *argv.add(i);
        let mut len = 0;
        while *s.add(len) != 0 {len+=1;}
        args[i] = str::from_utf8(slice::from_raw_parts_mut(s, len)).unwrap();
    }

    rkboot_entry(&a, args);
}

struct ThreadMainArgWrapper<'a> {
    base: *mut &'a str,
    size: usize,
}

#[cfg(feature="have_scheduler")]
fn thread_main(arg: *mut u8) {
    let arg = arg as *mut ThreadMainArgWrapper;
    let arg = unsafe {
        slice::from_raw_parts_mut((*arg).base, (*arg).size)
    };
    let ret = unsafe {main(arg)};
    rkplat::println!("main returned {}, halting system", ret);
    bootstrap::halt();
}

#[cfg(feature="have_scheduler")]
fn sched_start(arg: *mut u8) -> !{
    unsafe {
        (**(arg as *mut *mut dyn RKsched)).start();
    }
}

unsafe fn rkboot_entry(alloc: &dyn RKalloc, args: &mut [&str]) -> ! {
    irq::init(alloc).unwrap();
    device::init(alloc).unwrap();
    time::init();
    #[cfg(feature="have_scheduler")]
    {
        let wrapper = ThreadMainArgWrapper{base: args.as_mut_ptr(), size: args.len()};
        let cpu_cnt = lcpu::count();
        let scheds = slice::from_raw_parts_mut(alloc.alloc(cpu_cnt*size_of::<*mut dyn rksched::RKsched>(), 
            align_of::<*mut dyn rksched::RKsched>()) as *mut *mut dyn rksched::RKsched, cpu_cnt);
        for i in 0..cpu_cnt {
            #[cfg(feature="sched_coop")]
            {scheds[i] = rkalloc::alloc_type(alloc,rkschedcoop::RKschedcoop::new(i));}
            rksched::sched::register(&mut *scheds[i]);
        }
        for i in 0..cpu_cnt {
            (*scheds[i]).__set_next_sheduler(scheds[if i+1==cpu_cnt {0} else {i+1}]);
        }
        let cpu_id = lcpu::id();
        for i in 0..cpu_cnt {
            use rksched::thread::*;
            use runikraft::config::STACK_SIZE_SCALE as SSS;
            rksched::sched::create_thread_on_sched("empty", rkalloc::make_static(alloc),
                i,
                ThreadAttr::new(WAITABLE, true,PRIO_EMPTY, Duration::MAX, Duration::MAX, 4096*SSS, 0),
                ThreadLimit::default(),
                rksched::sched::__empty_thread_function,
                null_mut()).unwrap();
            if i==cpu_id {continue;}
            lcpu::start(i, alloc.alloc(1024*SSS, 16), sched_start, addr_of!(scheds[i]) as *mut u8).unwrap();
        }
        
        rksched::sched::create_thread_on_sched("main", rkalloc::make_static(alloc), 
            cpu_id,
            rksched::thread::ThreadAttr::default(),
            rksched::thread::ThreadLimit::default(),
            thread_main, addr_of!(wrapper) as *mut u8).expect("Fail to create main thread");
        (*scheds[cpu_id]).start();
    }
    #[cfg(not(feature="have_scheduler"))]
    {
        let ret = main(args);
        rkplat::println!("main returned {}, halting system", ret);
        bootstrap::halt();
    }
}
