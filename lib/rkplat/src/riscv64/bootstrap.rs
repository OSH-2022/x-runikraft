// SPDX-License-Identifier: BSD-3-Clause
// bootstrap.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

//TODO: arg

use core::ptr::{null_mut, addr_of, addr_of_mut};
use core::{slice, arch};
use runikraft::config::rkplat::*;
use runikraft::config::STACK_SIZE_SCALE as SSS;
use super::device;

#[cfg(feature="riscv_smode")]
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

const DEVICE_TREE_MAGIC: u32 = 0xD00DFEED;

#[repr(C)]
#[derive(Debug,Clone,Copy)]
pub(crate) struct HartLocal {
    _reg_space: usize,  //供中断处理程序使用的临时保存寄存器的空间(offset 0)
    pub(crate) hartsp: usize,   //异常处理程序使用的栈的指针    (offset 8)
    pub(crate) hartid: usize,   // offset 16
    pub(crate) is_running: bool,// offset 24
    pub(crate) start_sp: usize, // 启动新的内核时使用的栈指针 (offset 32)
    pub(crate) start_entry: usize,// 启动新的内核时跳转到的位置 (offset 40)
    pub(crate) recovery_pc: usize,//从中断返回时的pc，如果=0，则返回中断发生的位置 (offset 48)
    pub(crate) start_entry_arg: *mut u8,// 传递给start_entry的参数 (offset 56)
}

impl HartLocal {
    const fn new()->Self {
        Self { _reg_space: 0, hartid: 0, hartsp: 0, start_entry:0, start_sp: 0, is_running: false, recovery_pc: 0, start_entry_arg: null_mut()}
    }
}

/// 读取内核本地数据
pub(crate) unsafe fn hart_local() -> &'static mut HartLocal{
    let mut scratch: usize;
    #[cfg(feature="riscv_mmode")]
    arch::asm!("csrr {scratch}, mscratch",scratch=out(reg)(scratch));
    #[cfg(feature="riscv_smode")]
    arch::asm!("csrr {scratch}, sscratch",scratch=out(reg)(scratch));
    (scratch as *mut HartLocal).as_mut().unwrap()
}

pub(crate) static mut HART_NUMBER: usize = 0;
pub(crate) static mut HART_LOCAL:[HartLocal;LCPU_MAXCOUNT] = [HartLocal::new();LCPU_MAXCOUNT];
static mut EXCEPT_STACK:[[usize;128*SSS];LCPU_MAXCOUNT] = [[0;128*SSS];LCPU_MAXCOUNT];
static mut MAIN_STACK:[usize;MAIN_STACK_SIZE/8*SSS] = [0;MAIN_STACK_SIZE/8*SSS];


#[repr(C)]
struct DeviceTreeHeader {
    be_magic: u32,
    be_size: u32,
}

#[cfg(feature="riscv_smode")]
fn detect_hart_number() -> usize {
    for i in 0.. {
        if let Err(_) = sbi_call(0x48534D, 2, i, 0, 0) {
            return i;
        }
    }
    unsafe{core::hint::unreachable_unchecked();}
}

#[cfg(feature="riscv_mmode")]
#[no_mangle]
unsafe fn __runikraft_entry_point(hartid: usize, device_ptr: usize) -> !{
    use core::sync::atomic::{AtomicUsize, Ordering};
    extern "C" {
        fn __rkplat_hart_entry();
    }

    #[allow(non_upper_case_globals)]
    static hart_cnt: AtomicUsize = AtomicUsize::new(0);
    hart_cnt.fetch_add(1, Ordering::SeqCst);
    HART_LOCAL[hartid].hartid = hartid;
    HART_LOCAL[hartid].hartsp = (addr_of!(EXCEPT_STACK[hartid]) as usize)+1024;
    let scratch_addr = addr_of!(HART_LOCAL[hartid]);
    arch::asm!("csrw mscratch, {s}",s=in(reg)scratch_addr);
    if hartid != 0 {
        HART_LOCAL[hartid].is_running = false;
        loop {
            arch::asm!("wfi");
            if HART_LOCAL[hartid].is_running {
                __rkplat_hart_entry();
                panic!("should jumped");
            }
        }
    }
    HART_LOCAL[hartid].is_running = true;
    HART_NUMBER = hart_cnt.load(Ordering::SeqCst);
    let header = &*(device_ptr as *const DeviceTreeHeader);
    let magic = u32::from_be(header.be_magic);
    assert_eq!(magic,DEVICE_TREE_MAGIC);
    let len = u32::from_be(header.be_size) as usize;
    device::DEVICE_PTR = slice::from_raw_parts(device_ptr as *const u8, len);
    let uart_device = crate::drivers::uart::ns16550::Ns16550::new("uart@10000000",0x10000000, 0x0a);
    //提前初始化串口设备
    crate::console::UART_DEIVCE = Some(&*addr_of!(uart_device));
    __rkplat_newstack((addr_of_mut!(MAIN_STACK) as *mut u8).add(MAIN_STACK_SIZE), __runikraft_entry_point2,null_mut());
}

//debug: addi    sp,sp,-560
//release: addi    sp,sp,-112
#[cfg(feature="riscv_smode")]
#[no_mangle]
unsafe fn __runikraft_entry_point(hartid: usize, device_ptr: usize) -> !{
    HART_NUMBER = detect_hart_number();
    for i in 0..HART_NUMBER {
        HART_LOCAL[i].hartid = i;
        HART_LOCAL[i].hartsp = (addr_of!(EXCEPT_STACK[i]) as usize)+1024;
    }
    HART_LOCAL[hartid].is_running = true;
    let scratch_addr = addr_of!(HART_LOCAL[hartid]);
    arch::asm!("csrw sscratch, {s}",s=in(reg)scratch_addr);
    let header = &*(device_ptr as *const DeviceTreeHeader);
    let magic = u32::from_be(header.be_magic);
    assert_eq!(magic,DEVICE_TREE_MAGIC);
    let len = u32::from_be(header.be_size) as usize;
    device::DEVICE_PTR = slice::from_raw_parts(device_ptr as *const u8, len);
    __rkplat_newstack((addr_of_mut!(MAIN_STACK) as *mut u8).add(MAIN_STACK_SIZE), __runikraft_entry_point2,null_mut());
}

/// 退出
#[cfg(feature="riscv_mmode")]
pub fn halt() -> ! {
    todo!()
}
#[cfg(feature="riscv_smode")]
pub fn halt() -> ! {
    sbi_call(SBI_SRST, 0, 0, 0, 0).unwrap();
    panic!("Should halt.");
}

/// 重启
#[cfg(feature="riscv_mmode")]
pub fn restart() -> ! {
    todo!()
}
#[cfg(feature="riscv_smode")]
pub fn restart() -> ! {
    sbi_call(SBI_SRST, 0, 1, 0, 0).unwrap();
    panic!("Should restart.");
}

/// 崩溃
#[cfg(feature="riscv_mmode")]
pub fn crash() -> ! {
    print_bios!("System crashed!\n");
    todo!()
}
#[cfg(feature="riscv_smode")]
pub fn crash() -> ! {
    print_bios!("System crashed!\n");
    sbi_call(SBI_SRST, 0, 0, 1, 0).unwrap();
    loop {}//不能用panic，因为panic会调用crash
}

/// 挂起
pub fn suspend() -> ! {
    todo!();
}
