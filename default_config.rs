// SPDX-License-Identifier: FSFAP
// default_config.rs
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.
// Copying and distribution of this file, with or without modification, are
// permitted in any medium without royalty provided the copyright notice and
// this notice are preserved. This file is offered as-is, without any warranty.

pub const HEAP_SIZE: usize = 16<<20;

pub mod rksched {
    pub const STACK_SIZE_PAGE_ORDER: usize = 4;
    pub const STACK_SIZE: usize = super::rkplat::PAGE_SIZE*(1<<STACK_SIZE_PAGE_ORDER);
    pub mod limit {
        use core::time::Duration;
        pub const MEMORY_SIZE: usize = usize::MAX;
        pub const OPEN_FILES: usize = 1024;
        pub const PIPE_SIZE: usize = 4096;
        pub const CPU_TIME: Duration = Duration::MAX;
    }
}

/// rkplat配置
pub mod rkplat{
    /// 处理器的最大数量
    pub const LCPU_MAXCOUNT: usize = 8;
    /// 主线程的栈的大小
    pub const MAIN_STACK_SIZE: usize = 65536;
    pub const PAGE_SIZE: usize = 4096;
}

//相比C语言，Rust需要巨大的栈空间，而且debug模式所需的栈空间大约是release模式下的10倍
#[cfg(debug_assertions)]
pub const STACK_SIZE_SCALE: usize = 10;

#[cfg(not(debug_assertions))]
pub const STACK_SIZE_SCALE: usize = 1;
