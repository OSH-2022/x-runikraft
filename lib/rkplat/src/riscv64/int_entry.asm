.text
.globl __rkplat_int_except_entry
.align 2

# 中断/异常处理程序的入口
# 异常会完整地在early boot stack中保存原来的寄存器信息，
# 而中断只会在当前线程的栈上保存通用寄存器
__rkplat_int_except_entry:
    csrrw sp, sscratch, sp #保存用户栈，读出系统栈
    sd t0,-8(sp)
    csrr t0, scause
    bgez t0,__rkplat_except_handle_entry    #scause>=0，即最高位是0，说明是异常
    ld t0,-8(sp)
    csrrw sp, sscratch, sp  #还原栈指针和t0
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
    csrr a0,scause
    andi a0,a0,63
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
    addi sp,sp, -152    #现在sp指向early boot stack
    #sd t0,(sp)         #已经保存过t0
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
    csrr t2,sscratch
    sd t0,128(sp)
    sd t1,136(sp)
    sd t2,144(sp)
    csrr a0, scause
    mv a1,sp
    call __rkplat_exception_handle
    ld t0,128(sp)
    ld t1,136(sp)
    csrw sepc,t0
    csrw sstatus,t1 #这里不还原sscratch，因为假定异常处理程序不会进一步触发异常
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
    addi sp,sp,152
    csrrw sp, sscratch, sp
    sret
