// SPDX-License-Identifier: BSD-3-Clause
// intctrl.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

use core::arch;

/// 确认已处理IRQ
pub(crate) fn ack_irq(irq: usize) {
    clear_irq(irq);
}

/// 强制触发IRQ
#[allow(unused)]
pub(crate) fn raise_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        #[cfg(feature="riscv_mmode")]
        arch::asm!("csrs mip, {irq}",irq=in(reg)irq);
        #[cfg(feature="riscv_smode")]
        arch::asm!("csrs sip, {irq}",irq=in(reg)irq);
    }
}

/// 清除正在等待处理的IRQ
pub(crate) fn clear_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        #[cfg(feature="riscv_mmode")]
        arch::asm!("csrc mip, {irq}",irq=in(reg)irq);
        #[cfg(feature="riscv_smode")]
        arch::asm!("csrc sip, {irq}",irq=in(reg)irq);
    }
}
