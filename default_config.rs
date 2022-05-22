pub const RK_NETDEV_SCRATCH_SIZE: usize = 0;
pub const LIBUKNETDEV_MAXNBQUEUES: usize = 1;

/// rkplat配置
pub mod rkplat{
    /// 处理器的最大数量
    pub const LCPU_MAXCOUNT: usize = 16;
    /// 主线程的栈的大小
    pub const MAIN_STACK_SIZE: usize = 65536;
}

//相比C语言，Rust需要巨大的栈空间，而且debug模式所需的栈空间大约是release模式下的10倍
#[cfg(debug_assertions)]
pub const STACK_SIZE_SCALE: usize = 10;

#[cfg(not(debug_assertions))]
pub const STACK_SIZE_SCALE: usize = 1;
