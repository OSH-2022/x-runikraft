// SPDX-License-Identifier: BSD-3-Clause
// riscv64/mod.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.


#[cfg(feature="riscv_mmode")]
macro_rules! asm_path {
    ($x:expr) => {
        concat!("asm-m/",$x)
    };
}

#[cfg(feature="riscv_smode")]
macro_rules! asm_path {
    ($x:expr) => {
        concat!("asm-s/",$x)
    };
}

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

mod intctrl;
mod exception;

pub mod constants;

// supervisor binary interface
#[cfg(feature="riscv_smode")]
mod sbi;

// 寄存器
mod reg;
//mod mcause;
//mod mstauts;

// 导入所有的汇编代码
use core::arch::global_asm;

#[cfg(all(debug_assertions,feature="riscv_mmode"))]
global_asm!(concat!(
".equ boot_stack_size, 40960",
include_str!("asm-m/entry.asm"),
".section .bss.stack
.align 3
boot_stack_bottom:
.space 40960*8"));

#[cfg(all(not(debug_assertions),feature="riscv_mmode"))]
global_asm!(concat!(
".equ boot_stack_size, 4096",
include_str!("asm-m/entry.asm"),
".section .bss.stack
.align 3
boot_stack_bottom:
.space 4096*8"));

#[cfg(all(debug_assertions,feature="riscv_smode"))]
global_asm!(concat!(include_str!("asm-s/entry.asm"),
".section .bss.stack
.align 3
.space 40960
boot_stack_top:"));

#[cfg(all(not(debug_assertions),feature="riscv_smode"))]
global_asm!(concat!(include_str!("asm-s/entry.asm"),
".section .bss.stack
.align 3
.space 4096
boot_stack_top:"));


global_asm!(include_str!(asm_path!("int_entry.asm")));
global_asm!(include_str!("new_stack.asm"));
global_asm!(include_str!("thread.asm"));
