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
use core::{slice,str};
use core::mem::{align_of, size_of};
use core::ptr::addr_of;
use rkalloc::RKalloc;
use rkplat::{irq,time,bootstrap};
use runikraft::align_as;

const HEAP_SIZE: usize = 65536;

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

unsafe fn rkboot_entry(alloc: *const dyn RKalloc, args: &mut [&str]) -> ! {
    irq::init(alloc).unwrap();
    time::init();
    let ret = main(args);
    rkplat::println!("main returned {}, halting system", ret);
    bootstrap::halt();
}
