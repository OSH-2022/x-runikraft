// SPDX-License-Identifier: BSD-3-Clause
// irq.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use core::ptr::NonNull;

use rkalloc::Alloc;
use rkalloc::alloc_type;
use runikraft::compat_list::{Slist,SlistNode};
use crate::bootstrap;

use super::constants::*;
use super::lcpu;
use super::reg::RegGenInt;


static mut ALLOCATOR: Option<&dyn Alloc> = None;
/// 中断响应函数，返回false将中断转交给下一个函数处理，返回true表示中断处理完毕
pub type IRQHandlerFunc = fn(*mut u8)->bool;

struct IRQHandler {
    func: IRQHandlerFunc,
    arg: *mut u8,
}

/// 直接[None;64]会报 E0277
static mut IRQ_HANDLERS:[Option<Slist<IRQHandler>>;MAX_IRQ] = include!("64None.txt");

fn allocator() -> &'static dyn Alloc {
    unsafe {
        ALLOCATOR.unwrap()
    }
}

/// 初始化平台的IRQ子系统
/// - `a`: 内部使用的分配器
/// - 返回值: 初始化的状态
/// 
/// # 安全性
/// 
/// 必须保证分配器`a`在系统关机前仍有效，`a`可以拥有静态生命周期，也可以位于boot stack上
pub unsafe fn init(a: &dyn Alloc) -> Result<(), i32> {
    assert!(ALLOCATOR.is_none());
    union Helper<'a> {
        reference: &'a dyn Alloc,
        pointer: *const dyn Alloc,
    }
    ALLOCATOR = Some(Helper{pointer: Helper{reference: a}.pointer}.reference);
    for i in &mut IRQ_HANDLERS{
        *i = Some(Slist::new());
    }
    Ok(())
}

/// 注册中断响应函数，可以为一个中断号注册多个响应函数，它们将按注册的逆序被调用
/// - `irq`: 中断号
/// - `func`: 响应函数
/// - `arg`: 传递给响应函数的额外参数
/// 
/// # 安全性
/// 
/// - `arg`指向的数据必须在关机前仍然有效，它可以是静态数据，也可以是位于boot stack上的数据，
/// 还可以是由生命足够长的分配器分配的数据
/// - `func`需要将`arg`转换成合适的类型
pub unsafe fn register(irq: usize, func: IRQHandlerFunc, arg: *mut u8) -> Result<(), i32> 
{   
    assert!(irq<MAX_IRQ);
    let handler = IRQHandler{func,arg};
    let flags =lcpu::save_irqf(); 
    //interruption
    IRQ_HANDLERS[irq].as_mut().unwrap().push_front(NonNull::new(alloc_type(allocator(),SlistNode::new(handler))).unwrap());
    lcpu::restore_irqf(flags);
    if irq&1<<63 !=0 { clear_irq(irq); }
    Ok(())
}

//TODO: 
#[no_mangle]
unsafe extern "C" fn __rkplat_irq_handle(regs: &mut RegGenInt, irq: usize) {
    for i in IRQ_HANDLERS[irq].as_ref().unwrap().iter() {
        if (i.element.func)(i.element.arg) {
            ack_irq(irq);
            let rpc = bootstrap::hart_local().recovery_pc;
            if rpc !=0 {
                regs.pc = rpc;
                bootstrap::hart_local().recovery_pc = 0;
            }
            return;
        }
    }
    println!("Unhandled irq={}",irq);
    // panic!();
}

/// 确认已处理IRQ
pub fn ack_irq(irq: usize) {
    clear_irq(irq);
}

/// 强制触发IRQ
pub fn raise_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        core::arch::asm!("csrs sip, {irq}",
        irq=in(reg)irq);
    }
}

/// 清除正在等待处理的IRQ
pub fn clear_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        core::arch::asm!("csrc sip, {irq}",
        irq=in(reg)irq);
    }
}
