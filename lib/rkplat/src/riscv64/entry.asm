# SPDX-License-Identifier: BSD-3-Clause
# entry.asm
# Authors: 张子辰 <zichen350@gmail.com>
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

.extern sbss
.extern ebss

.section .text.entry
.globl __runikraft_start

__runikraft_start:
    #清空bss段
    la t0,sbss
    la t1,ebss
    1:
        bge t0,t1,2f
        sd x0,(t0)
        addi t0,t0,8
        j 1b
    2:
    addi t0,zero,0
    addi t1,zero,0
    # 初始化中断响应函数
    la t0, __rkplat_int_except_entry
    csrw stvec, t0
    #加载栈指针（现在的写法仅适用于OpenSBI：开机时只有一个核会执行，而其他核暂停）
    la sp,boot_stack_top
    call __runikraft_entry_point
