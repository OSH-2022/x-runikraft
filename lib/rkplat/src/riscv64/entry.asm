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
    #加载栈指针
    la sp,boot_stack_top
    call __runikraft_entry_point

.section .bss.stack
    .space 4096
boot_stack_top:
