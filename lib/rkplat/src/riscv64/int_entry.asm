# SPDX-License-Identifier: BSD-3-Clause
# int_entry.asm
# Authors: 张子辰 <zichen350@gmail.com>
# Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

.text
.globl __rkplat_int_except_entry
.align 2

# 中断/异常处理程序的入口
# 异常会完整地在early boot stack中保存原来的寄存器信息，
# 而中断只会在当前线程的栈上保存通用寄存器
__rkplat_int_except_entry:
    csrrw t1, sscratch, t1 #读取 HartLocal
    sd t0,(t1)             #暂存t0 到 _reg_space
    csrr t0, scause
    bgez t0,__rkplat_except_handle_entry    #scause>=0，即最高位是0，说明是异常
    ld t0,(t1)
    csrrw t1, sscratch, t1  #还原t1、t0和sscratch
    addi sp,sp,-144         #直接在当前线程的栈上保存寄存器
    sd t0,(sp)
    sd t1,8(sp)
    sd t2,16(sp)
    sd t3,24(sp)
    sd t4,32(sp)
    sd t5,40(sp)
    sd t6,48(sp)
    sd a0,56(sp)
    sd a1,64(sp)
    sd a2,72(sp)
    sd a3,80(sp)
    sd a4,88(sp)
    sd a5,96(sp)
    sd a6,104(sp)
    sd a7,112(sp)
    sd ra,120(sp)
    csrr t0,sepc
    csrr t1,sstatus
    sd t0,128(sp)
    sd t1,136(sp)
    mv a0,sp
    csrr a1,scause
    andi a1,a1,63
    call __rkplat_irq_handle #调用用Rust编写的中断处理函数
    ld t5,128(sp)
    ld t6,136(sp)
    csrw sepc,t5
    csrw sstatus,t6
    ld t0,(sp)
    ld t1,8(sp)
    ld t2,16(sp)
    ld t3,24(sp)
    ld t4,32(sp)
    ld t5,40(sp)
    ld t6,48(sp)
    ld a0,56(sp)
    ld a1,64(sp)
    ld a2,72(sp)
    ld a3,80(sp)
    ld a4,88(sp)
    ld a5,96(sp)
    ld a6,104(sp)
    ld a7,112(sp)
    ld ra,120(sp)
    addi sp,sp,144
    sret

__rkplat_except_handle_entry:
    ld t0,(t1)          #还原t0
    sd sp,(t1)          #暂存sp 到 _reg_space
    ld sp,8(t1)         #现在sp指向异常处理栈
    addi sp,sp, -152
    sd t0,(sp)
    csrrw t0,sscratch,t1 #还原sscratch，并且把t1原来的值加载到t0
    sd t0,8(sp)
    sd t2,16(sp)
    sd t3,24(sp)
    sd t4,32(sp)
    sd t5,40(sp)
    sd t6,48(sp)
    sd a0,56(sp)
    sd a1,64(sp)
    sd a2,72(sp)
    sd a3,80(sp)
    sd a4,88(sp)
    sd a5,96(sp)
    sd a6,104(sp)
    sd a7,112(sp)
    sd ra,120(sp)
    csrr t0,sepc
    csrr t3,sstatus
    ld t2,(t1)          #刚才保存到_reg_space的旧sp
    sd t0,128(sp)
    sd t3,136(sp)
    sd t2,144(sp)
    csrr a0, scause
    mv a1,sp
    call __rkplat_exception_handle
    ld t0,128(sp)
    ld t1,136(sp)
    addi t0,t0,4
    csrw sepc,t0    #将sepc还原成引发异常的指令的下一条指令
    csrw sstatus,t1
    ld t0,(sp)
    ld t1,8(sp)
    ld t2,16(sp)
    ld t3,24(sp)
    ld t4,32(sp)
    ld t5,40(sp)
    ld t6,48(sp)
    ld a0,56(sp)
    ld a1,64(sp)
    ld a2,72(sp)
    ld a3,80(sp)
    ld a4,88(sp)
    ld a5,96(sp)
    ld a6,104(sp)
    ld a7,112(sp)
    ld ra,120(sp)
    ld sp,144(sp)   #不允许出现多重异常，所以sscratch里储存的hartsp不应该改变，所以无需还原
    sret
