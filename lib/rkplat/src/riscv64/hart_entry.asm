.text
.globl __rkplat_hart_entry
.align 2

__rkplat_hart_entry:
    csrw sscratch, a1
    li t0, 1
    sb t0, 24(a1) #is_running
    ld sp, 32(a1) #start_sp
    ld ra, 40(a1) #start_entry
    ret
