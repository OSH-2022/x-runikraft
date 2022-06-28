# -*- assembly -*-
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
    csrw mtvec, t0
    #加载栈指针
    la sp,boot_stack_bottom
    addi t0,a0,1
    li t1,boot_stack_size
    mul t0,t0,t1 #t0=(a0+1)*boot_stack_size
    add sp,sp,t0
    call __runikraft_entry_point
