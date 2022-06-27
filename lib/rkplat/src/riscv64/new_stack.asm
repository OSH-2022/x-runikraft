# -*- assembly -*-
# SPDX-License-Identifier: BSD-3-Clause
# new_stack.asm
# Authors: 张子辰 <zichen350@gmail.com>
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

.text
.globl __rkplat_newstack
# 切换栈: 早期引导栈->引导栈，对应_libkvmplat_newstack
# fn __rkplat_newstack(stack_top: *mut u8, tramp: extern fn(*mut u8)->!, arg: *mut u8)->!;
__rkplat_newstack:
    mv sp,a0
    mv a0,a2
    jr a1
