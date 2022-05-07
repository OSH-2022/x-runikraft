//TODO: arg

use core::ptr::null_mut;

use super::sbi::*;

extern "C" {
    /// 在初始化时有平台层调用
    /// 
    /// 非平台层的库必须提供其实现
    /// 
    /// 在拥有已分析的参数的平台，这个函数会被直接调用；否则，平台层将调用
    /// `rkplat_entry_argp`，并由它分析参数，然后调用`rkplat_entry`
    /// - `argc`: 参数个数
    /// - `argv`: 参数，每个参数是NTBS
    pub fn rkplat_entry(argc: i32, argv: *mut *mut u8) -> !;

    /// 在初始化时有平台层调用
    /// 
    /// 非平台层的库必须提供其实现
    /// 
    /// 在没有已分析的参数的平台，平台层将调用
    /// `rkplat_entry_argp`，由它分析参数，然后调用`rkplat_entry`
    /// - `arg0`: NTBS，参数0，即镜像的名称；可能为空，这时分析后的参数的argv[0]也需要留空
    /// - `argb`: 剩余的参数
    /// - `argb_len`: 剩余的参数的长度，`argb_len=0`表示`argb`是空终止的
    pub fn rkplat_entry_argp(arg0: *mut u8, argb: *mut u8, argb_len: usize) -> !;

    fn __rkplat_newstack(stack_top: *mut u8, tramp: extern fn(*mut u8)->!, arg: *mut u8)->!;
}

#[no_mangle] extern "C" fn __runikraft_entry_point2(_arg: *mut u8) -> !{
    unsafe{
        rkplat_entry(0,0 as *mut *mut u8);
    }
}

//TODO:找到真正的最大合法地址
const MAX_MEM_ADDR: *mut u8 = 0x85000000 as *mut u8;

#[no_mangle]
pub unsafe fn __runikraft_entry_point() -> !{
    __rkplat_newstack(MAX_MEM_ADDR, __runikraft_entry_point2,null_mut());
}

/// 退出
pub fn halt() -> ! {
    sbi_call(SBI_SRST, 0, 0, 0, 0).unwrap();
    panic!("Should halt.");
}

/// 重启
pub fn restart() -> ! {
    sbi_call(SBI_SRST, 0, 1, 0, 0).unwrap();
    panic!("Should restart.");
}

/// 崩溃
pub fn crash() -> ! {
    print!("System crashed!\n");
    sbi_call(SBI_SRST, 0, 0, 1, 0).unwrap();
    loop {}//不能用panic，因为panic会调用crash
}

/// 挂起
pub fn suspend() -> ! {
    todo!();
}
