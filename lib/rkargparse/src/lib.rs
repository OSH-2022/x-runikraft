// SPDX-License-Identifier: BSD-3-Clause
// rkargparse/lib.rs

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

use core::slice;

fn left_shift(buf: &mut [u8], mut index: usize)
{
	while buf[index] != 0 && index < buf.len() {
		buf[index] = buf[index + 1];
		index+=1;
	}
}

pub fn argnparse(argb: *mut u8, maxlen: usize, argv: *mut *mut u8, maxcount: usize) -> i32 {
    let mut argc = 0;
    let mut prev_wspace = true;
    let mut in_quote = 0u8;
    let mut i: usize = 0;

    debug_assert!(!argb.is_null());
    debug_assert!(!argv.is_null());

    let argb = unsafe{slice::from_raw_parts_mut(argb, maxlen)};
    let argv = unsafe {slice::from_raw_parts_mut(argv, maxcount)};

    while i < maxlen && argc < maxcount {
        if match argb[i] as char {
            // end of string
            '\0' => {break;},
            ' '|'\r'|'\n'|'\t'|'v' => {
                if in_quote==0 {
                    argb[i] = 0;
                    prev_wspace = true;
                    false
                }
                else {
                    true
                }
            }, 
            '\''| '"' => {
                if in_quote==0 {
                    in_quote = argb[i];
                    left_shift(argb, i);
                    i-=1;
                    false
                }
                else if in_quote == argb[i] {
                    in_quote = 0;
                    left_shift(argb, i);
                    i-=1;
                    false
                }
                else {
                    true
                }
            },
            _ => true,
        } {
            // any character
            if prev_wspace {
                argv[argc] = &mut argb[i];
                argc += 1;
                prev_wspace = false;
            }
        }
        i+=1;
    }

	return argc as i32;
}
