# -*- assembly -*-
# SPDX-License-Identifier: BSD-3-Clause
# hart_entry.asm
# Authors: 张子辰 <zichen350@gmail.com>
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

.text
.globl __rkplat_hart_entry
.align 2

__rkplat_hart_entry:
    csrw sscratch, a1
    li t0, 1
    sb t0, 24(a1) #is_running
    ld sp, 32(a1) #start_sp
    ld ra, 40(a1) #start_entry
    ld a0, 56(a1) #start_entry_arg
    ret
