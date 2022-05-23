// SPDX-License-Identifier: BSD-3-Clause
// bitcount.rs

// Authors: John Baldwin <jhb@FreeBSD.org>
//          张子辰 <zichen350@gmail.com>

// Copyright (c) 2015 John Baldwin <jhb@FreeBSD.org>.
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

pub trait BitCount{
    fn bitcount(self)->Self;
}

impl BitCount for u16{
    fn bitcount(self)->u16{
        let mut x = self;
        x = (x & 0x5555_u16) + ((x & 0xaaaa_u16) >> 1);
	    x = (x & 0x3333_u16) + ((x & 0xcccc_u16) >> 2);
	    x = (x + (x >> 4)) & 0x0f0f_u16;
	    x = (x + (x >> 8)) & 0x00ff_u16;
	    x
    }
}

impl BitCount for u32{
    fn bitcount(self)->u32{
        let mut x = self;
        x = (x & 0x55555555_u32) + ((x & 0xaaaaaaaa_u32) >> 1);
        x = (x & 0x33333333_u32) + ((x & 0xcccccccc_u32) >> 2);
        x = (x + (x >> 4)) & 0x0f0f0f0f_u32;
        x =  x + (x >> 8);
        x = (x + (x >> 16)) & 0x000000ff_u32;
        x
    }
}

impl BitCount for u64{
    fn bitcount(self)->u64{
        let mut x = self;
        x = (x & 0x5555555555555555_u64) + ((x & 0xaaaaaaaaaaaaaaaa_u64) >> 1);
        x = (x & 0x3333333333333333_u64) + ((x & 0xcccccccccccccccc_u64) >> 2);
        x = (x + (x >> 4)) & 0x0f0f0f0f0f0f0f0f_u64;
        x =  x + (x >> 8);
        x =  x + (x >> 16);
        x = (x + (x >> 32)) & 0x000000ff_u64;
        x
    }
}

impl BitCount for u128{
    fn bitcount(self)->u128{
        (((self>>64) as u64).bitcount() + ((self & 0xffffffffffffffffu128) as u64).bitcount()) as u128
    }
} 
