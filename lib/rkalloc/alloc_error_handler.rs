// SPDX-License-Identifier: BSD-3-Clause
// alloc_error_handler.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Fail to allocate memory. layout={:?}",layout);
}
