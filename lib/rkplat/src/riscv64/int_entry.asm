.text
.globl __rkplat_irq_handle_entry
.align 2

# 异常处理程序的入口（TODO）
__rkplat_irq_handle_entry:
    csrrw sp, sscratch, sp #保存用户栈，读出系统栈
    addi sp,sp, -256    #32*8
    # 无论如何都需要保存tmp寄存器
    sd t0,(sp)
    sd t1,8(sp)
    sd t2,16(sp)
    sd t3,24(sp)
    sd t4,32(sp)
    sd t5,40(sp)
    sd t6,48(sp)
    #现在可以使用tmp寄存器
    csrr t0, scause
    li t1, 8
    beq t0,t1,1f    #syscall只需要保存t
    sd a0,56(sp)
    sd a1,64(sp)
    sd a2,72(sp)
    sd a3,80(sp)
    sd a4,88(sp)
    sd a5,96(sp)
    sd a6,104(sp)
    sd a7,112(sp)
    bgez t0,1f      #exception无需保存s（因为编译器会自动还原s）
    sd s0,120(sp)
    sd s1,128(sp)
    sd s2,136(sp)
    sd s3,144(sp)
    sd s4,152(sp)
    sd s5,160(sp)
    sd s6,168(sp)
    sd s7,176(sp)
    sd s8,184(sp)
    sd s9,192(sp)
    sd s10,200(sp)
    sd s11,208(sp)
1:
    sd ra,216(sp)
    csrr t3,sscratch#原本的sp
    csrr t2,sepc    #原本的pc
    csrr t1,sstatus
    sd t3,224(sp)   #sp
    sd tp,232(sp)    
    sd t2,240(sp)   #pc
    sd t1,248(sp)   #sstatus
    csrr a0, scause
    mv a1,sp
    call __rkplat_irq_handle
