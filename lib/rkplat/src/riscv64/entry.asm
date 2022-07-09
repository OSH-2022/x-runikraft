# -*- assembly -*-
# SPDX-License-Identifier: BSD-3-Clause
# entry.asm
# Authors: 张子辰 <zichen350@gmail.com>
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

.extern __rkplat_boot_sbss
.extern __rkplat_boot_ebss
.extern __rkplat_boot_ekernel

.section .text.entry
.globl __runikraft_start

__runikraft_start:
    la t0, __runikraft_start_failed
    csrw stvec, t0
    #把FDT复制到安全的区域
    mv t0, a1
    la t1, __rkplat_boot_ekernel
    # a1 >= __rkplat_boot_ekernel说明FDT本来就处在安全的区域
    bge t0,t1,3f
    lwu t2,(t0)
    # magic number of FDT in big endianness
    li t3,0xEDFE0DD0
    bne t2,t3,__runikraft_start_failed
    lwu t2,4(t0)
    # exchange the endianness of t2
    srli t3,t2,24   #t3 = [31:24]
    srli ra,t2,16   #ra = [23:16]
    srli sp,t2,8    #sp = [15: 8]
    andi ra,ra,0xFF
    andi sp,sp,0xFF
    slliw t2,t2,24
    slli ra,ra,8
    slli sp,sp,16
    or t2,t2,t3
    or ra,ra,sp
    or t2,t2,ra
    add t3,t2,t0 #the ending address of FDT
    1:
        lbu t2,(t0)
        sb t2,(t1)
        addi t0,t0,1
        addi t1,t1,1
        blt t0,t3,1b
    la a1,__rkplat_boot_ekernel
    3:
    #清空bss段
    la t0,__rkplat_boot_sbss
    la t1,__rkplat_boot_ebss
    1:
        bge t0,t1,2f
        sd x0,(t0)
        addi t0,t0,8
        j 1b
    2:
    # 初始化中断响应函数
    la t0, __rkplat_int_except_entry
    csrw stvec, t0
    #加载栈指针（现在的写法仅适用于OpenSBI：开机时只有一个核会执行，而其他核暂停）
    la sp,boot_stack_top
    call __runikraft_entry_point

.align 2
__runikraft_start_failed:
    la s0,early_boot_fail
1:
    lb a0,(s0)
    beqz a0,2f
    li a7,1
    ecall
    addi s0,s0,1
    j 1b
2:
    li a7,0x53525354
    li a6,0
    li a0,0
    li a1,1
    li a2,0
    ecall
    j __runikraft_start_failed

.rodata
.align 0
early_boot_fail:
    .string "Fatal error: __runikraft_start failed.\nIf you have changed the heap size in menuconfig, try using -m <memory size> QEMU option to enlarge RAM limit.\n"
