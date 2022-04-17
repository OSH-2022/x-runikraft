.extern sbss
.extern ebss

.section .text.entry
.globl __runikraft_start

__runikraft_start:
    #清空bss段
    la t0,sbss
    la t1,ebss
    clean_bss:
        bge t0,t1,clean_bss_end
        sd x0,(t0)
        addi t0,t0,8
        j clean_bss
    clean_bss_end:
    addi t0,zero,0
    addi t1,zero,0
    #加载栈指针
    la sp,stack_top
    call __runikraft_entry_point

.section .bss.stack
    .space 65536
stack_top:
