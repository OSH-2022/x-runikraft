// SPDX-License-Identifier: BSD-3-Clause
// rkswrand/lib.rs

// Authors:  张子辰 <zichen350@gmail.com>
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

use core::slice;

use rkplat::drivers::virtio::__ENTROPY_DEIVCE;
use rand::{prelude::*, distributions::Standard, rngs::OsRng};

#[cfg(feature="have_scheduler")]
type Lock = rklock::Mutex;

#[cfg(not(feature="have_scheduler"))]
type Lock = rkplat::spinlock::SpinLock;

static mut STD_GEN: Option<StdRng> = None;
static STD_LOCK: Lock = Lock::new();
static mut FAST_GEN: Option<SmallRng> = None;
static FAST_LOCK: Lock = Lock::new();

#[no_mangle]
extern "C" fn __getrandom_custom(mut dst: *mut u8, len: usize) -> u32 {
    if let Some(rng) = unsafe{&mut __ENTROPY_DEIVCE } {
        let mut size = 0;
        while size<len {
            let buf = unsafe{slice::from_raw_parts_mut(dst, len-size)};
            let size_received = 
                match rng.recv(buf) {
                    Ok(size) => size,
                    Err(_) => {
                        return getrandom::Error::CUSTOM_START+1;
                    }
                };
            size+=size_received;
            dst = unsafe{dst.add(size_received)};
        }
        0
    }
    else {
        getrandom::Error::CUSTOM_START+2
    }
}

pub fn hardware_random<T>() -> T
where
    Standard: Distribution<T>,
{
    OsRng{}.sample(Standard{})
}

pub fn random<T>() -> T 
where 
    Standard: Distribution<T>,
{
    let _lock = STD_LOCK.lock();
    unsafe {
        if STD_GEN.is_none() {
            STD_GEN = Some(StdRng::from_entropy());
        }
        STD_GEN.as_mut().unwrap().sample(Standard{})
    }
}

pub fn fast_random<T>() -> T
where 
    Standard: Distribution<T>,
{
    let _lock = FAST_LOCK.lock();
    unsafe {
        if FAST_GEN.is_none() {
            FAST_GEN = Some(SmallRng::from_entropy());
        }
        FAST_GEN.as_mut().unwrap().sample(Standard{})
    }
}
