// SPDX-License-Identifier: BSD-3-Clause
// thread.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use super::reg;
use core::arch;

extern "C" {
    fn __thread_starter();
    fn __thread_context_start(sp: usize, pc: usize);
    fn __thread_context_switch(prevctx: *mut Context, nextctx: *mut Context);
}

#[repr(C)]
#[derive(Default)]
pub struct Context {
    sp: usize, //栈指针
    pc: usize, //程序计数器
    tp: usize, //线程本地数据
    regs: reg::RegGenSw,  //保存通用寄存器的区域
    #[cfg(feature="save_fp")]
    fregs: reg::RegFloat, //保存浮点寄存器的区域（可以为空）
}


/// 初始化`Context`
/// 
/// 在Rust中，“标准”的初始化方式是`new`，但是使用new会导致Context的接口风格不统一
/// 
/// 创建`Context`对象的方法：在线程栈的顶部开辟一块大小为size_of::<Context>()的空间，
/// 然后以该空间的地址为参数调用`init`。
/// 
/// - `ctx`: 上面提到的“该空间的地址”
/// - `sp`: 新线程的栈底指针
/// - `tp`: 新线程的线程本地数据指针
/// - `arg`: 启动新线程的参数
/// 
/// 创建后的线程的入口地址是`__thread_starter`。
/// `__thread_starter`会调用形如`fn thread_entry(arg: *mut u8)->!`的函数，
/// `thread_entry`应进一步调用`thread_main`，并且在`thread_main`退出时通知调度器切换到另外一个线程
pub unsafe fn init(ctx: *mut Context, sp: usize, tp: usize, entry: unsafe fn(*mut u8)->!, arg: *mut u8) {
    arch::asm!(
       "sd {func}, -8({sp})
        sd {arg}, -16({sp})",
        sp=in(reg)sp,
        func=in(reg)entry,
        arg=in(reg)arg
    );
    (*ctx).sp = sp - 16;
    (*ctx).tp = tp;
    (*ctx).pc = __thread_starter as usize;
}

/// 启动新线程
/// 为了实现
pub unsafe fn start(ctx: *mut Context) -> ! {
    set_tp((*ctx).tp);
    __thread_context_start((*ctx).sp,(*ctx).pc);
    panic!("Thread did not start.");
}

/// 切换线程
/// 调用该函数会将控制权转交给线程`nextctx`，`nextctx`必须处在以下两种状态之一：
/// 1. 刚刚调用`switch`，这时，`nextctx`会观察到`switch`返回；
/// 2. 从未执行过，这时，将执行`__thread_starter`
pub unsafe fn switch(prevctx: *mut Context, nextctx: *mut Context) {
    #[cfg(feature="save_fp")]
    __thread_save_sp((*prevctx).fregs);
    #[cfg(feature="save_fp")]
    __thread_restore_sp((*nextctx).fregs);
    __thread_context_switch(prevctx,nextctx);
}

#[inline(always)]
unsafe fn set_tp(tp: usize) {
    arch::asm!("mv tp,{tp}",
        tp=in(reg)tp);
}
