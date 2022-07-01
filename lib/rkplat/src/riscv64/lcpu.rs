// SPDX-License-Identifier: BSD-3-Clause
// lcpu.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

#![allow(unused_imports)]
use core::time::Duration;
use core::arch;
use core::sync::atomic;
use core::ptr::addr_of;

use super::{bootstrap,time,spinlock};
use super::sbi::sbi_call;

/// 中断标志，具体格式由平台决定
pub type IRQFlag = usize;

/// 启用中断
pub fn enable_irq() {
    unsafe {
        arch::asm!(
            "csrsi sstatus, 0x2" //0x2是sstatus的SIE段
        );
    }
}

/// 禁用中断
pub fn disable_irq() {
    unsafe {
        arch::asm!(
            "csrci sstatus, 0x2" //0x2是sstatus的SIE段
        );
    }
}

/// 保存中断标志并关闭中断
pub fn save_irqf() -> IRQFlag {
    let mut flag = 0_usize;
    unsafe {
        arch::asm!(
            "csrrw {flag},sie,{flag}",
            flag=inout(reg)flag
        );
    }
    flag
}

/// 加载中断标志
pub fn restore_irqf(flag: IRQFlag) {
    unsafe {
        arch::asm!(
            "csrw sip,0
            csrw sie,{flag}",
            flag=in(reg)flag
        );
    }
}

/// 检查中断是否被禁用
pub fn irqs_disabled() -> bool {
    let sstatus: u64;
    let sie: u64;
    unsafe {
        arch::asm!(
           "csrr {sstatus}, sstatus
            csrr {sie}, sie",
            sstatus=out(reg)sstatus,
            sie=out(reg)sie,
        );
    }
    (sstatus | 0x2 == 0 )||sie==0
}

/// 挂起当前的逻辑处理器
pub fn halt() -> !{
    save_irqf();
    //Hart State Management-
    //sbi_hart_suspend(suspend_type,resume_addr,opaque)
    //suspend_type=Default retentive suspend
    unsafe {
        bootstrap::hart_local().is_running = false;
        arch::asm!("wfi");
    }
    panic!("Fail to suspend.");
}

/// 挂起当前处理器一段时间，
/// 处理将在`deadline`到达(`get_ticks()>=deadline`)或中断/信号到来时重启
pub fn halt_to(until: Duration) {
    let flag = save_irqf();
    time::block(until);
    restore_irqf(flag);
}

/// 挂起当前处理器，
/// 处理将在中断/信号到来时重启
pub fn halt_irq() {
    let flag = save_irqf();
    unsafe {
        bootstrap::hart_local().is_running = false;
    }

    //wfi指令在全局IRQ被禁用时仍然有效，如果不这么做，假如在执行完restore_irqf后但在执行wfi前，
    //出现了中断，wfi仍然会被执行，但是中断不会再来，于是处理器将长时间等待
    disable_irq();
    restore_irqf(0xFFFF);//开启所有中断
    unsafe {
        arch::asm!("wfi");
        bootstrap::hart_local().is_running = true;
    }
    restore_irqf(flag);
}

pub type ID = usize;

#[cfg(feature = "has_smp")]
mod smp {
    use super::*;

    extern "C" {
        fn __rkplat_hart_entry();
    }

    pub type Entry = fn(*mut u8) -> !;
    pub type StackPointer = *mut u8;

    static LOCK: spinlock::SpinLock = spinlock::SpinLock::new();
    arch::global_asm!(include_str!("hart_entry.asm"));

    /// 返回当前的逻辑处理器的ID
    pub fn id() -> ID {
        unsafe{bootstrap::hart_local().hartid} 
    }

    /// 返回逻辑处理器的数量
    pub fn count() -> ID {
        unsafe{bootstrap::HART_NUMBER}
    }

    /// 启动逻辑处理器。逻辑处理器将从给定的其实位置开始执行，
    /// 未执行的逻辑处理器会进入低功耗状态。
    ///
    /// 参数是若干要启动的逻辑处理机的slice：
    /// - `lcpuid`: 逻辑处理器ID
    /// - `sp`: 栈指针
    /// - `entry`: 入口函数
    pub fn start(lcpuid: ID, sp: StackPointer, entry: Entry, arg: *mut u8) -> Result<(), i32> {
        let _lock = match LOCK.try_lock() {
            Some(x) => x,
            None => {return Err(0);}
        };
        rmb();
        unsafe {
            if !bootstrap::HART_LOCAL[lcpuid].is_running {
                bootstrap::HART_LOCAL[lcpuid].start_sp = sp as usize;
                bootstrap::HART_LOCAL[lcpuid].start_entry = entry as usize;
                bootstrap::HART_LOCAL[lcpuid].start_entry_arg = arg;
            }
            //sbi_hart_start
            //unsigned long hartid
            //unsigned long start_addr
            //unsigned long opaque
            if let Err(err) = sbi_call(0x48534D, 0, lcpuid,
                __rkplat_hart_entry as usize,
                addr_of!(bootstrap::HART_LOCAL[lcpuid]) as usize)
            {
                return Err(err as i32);
            }
        }
        Ok(())
    }

    /// 让编号为`lcpuid`的逻辑处理器等待`timeout`
    ///
    /// 可以用`timeout`=0等待不确定的时间
    pub fn wait(_lcpuid: ID, _timeout: Duration) -> Result<(), i32> {
        todo!();
    }

    //TODO:
    // fn run()

    /// 唤醒被挂起或处在低功耗状态的逻辑处理器
    pub fn wakeup(lcpuid: ID) -> Result<(), i32> {
        //sbi_send_ipi
        //unsigned long hart_mask
        //unsigned long hart_mask_base
        if let Err(err) = sbi_call(0x735049, 0, 1<<lcpuid,
            0,0)
        {
            return Err(err as i32);
        }
        Ok(())
    }
}

#[cfg(feature = "has_smp")]
pub use smp::*;

#[cfg(not(feature = "has_smp"))]
#[inline(always)]
pub fn id() -> ID { 0 }

#[cfg(not(feature = "has_smp"))]
#[inline(always)]
pub fn count() -> ID { 1 }

//来自ukarch
#[inline(always)]
pub fn barrier() {atomic::compiler_fence(atomic::Ordering::SeqCst);}

#[inline(always)]
pub fn mb() {atomic::fence(atomic::Ordering::AcqRel);}

#[inline(always)]
pub fn rmb() {atomic::fence(atomic::Ordering::Acquire);}

#[inline(always)]
pub fn wmb() {atomic::fence(atomic::Ordering::Release);}

#[inline(always)]
pub fn spinwait() {
    unsafe {
        arch::asm!(
            ".insn i 0x0F,0,x0,x0,0x010"//arch::pause的实现
        );
    }
}

#[inline(always)]
pub fn read_sp() -> usize{
    let sp: usize;
    unsafe{
        arch::asm!(
            "mv {sp},sp",
            sp=out(reg)sp
        );
    }
    sp
}

#[inline(always)]
pub fn read_tp() -> usize{
    let tp: usize;
    unsafe{
        arch::asm!(
            "mv {tp},tp",
            tp=out(reg)tp
        );
    }
    tp
}
