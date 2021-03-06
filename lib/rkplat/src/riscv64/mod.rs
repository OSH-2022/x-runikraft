// SPDX-License-Identifier: BSD-3-Clause
// riscv64/mod.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

#[macro_use]
pub mod console;
pub mod time;
pub mod bootstrap;
pub mod lcpu;
pub mod irq;
pub mod spinlock;
//pub mod memory;
pub mod thread;
pub mod device;

mod exception;

pub mod constants;

// supervisor binary interface
mod sbi;

// 寄存器
mod reg;
//mod mcause;
//mod mstauts;

// 导入所有的汇编代码
use core::arch::global_asm;


#[cfg(debug_assertions)]
global_asm!(concat!(include_str!("entry.asm"),
".section .bss.stack
.align 3
.space 40960
boot_stack_top:"));
#[cfg(not(debug_assertions))]
global_asm!(concat!(include_str!("entry.asm"),
".section .bss.stack
.align 3
.space 4096
boot_stack_top:"));
global_asm!(include_str!("int_entry.asm"));
global_asm!(include_str!("new_stack.asm"));
global_asm!(include_str!("thread.asm"));
