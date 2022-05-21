.text
.globl __thread_starter
.globl __thread_context_start
.globl __thread_context_switch

__thread_starter:
    ld a0,(sp)      #arg: *mut u8
    # “return” to thread entry function
    ld ra,8(sp)
    addi sp,sp,16
    ret

__thread_context_start:
    mv sp, a0
    mv ra, a1
    ret

#a0 = prevctx
#a1 = nextctx
#offset:
# sp -24
# pc -16
# tp -8
# regs 0
__thread_context_switch:
    #store ra on current stack
    addi sp,sp,-8
    sd ra,(sp)
    addi a0,a0,24
    addi a1,a1,24
    #save s0~s11
    sd s0,0(a0)
    sd s1,8(a0)
    sd s2,16(a0)
    sd s3,24(a0)
    sd s4,32(a0)
    sd s5,40(a0)
    sd s6,48(a0)
    sd s7,56(a0)
    sd s8,64(a0)
    sd s9,72(a0)
    sd s10,80(a0)
    sd s11,88(a0)
    #restore s0~s11
    ld s0,0(a1)
    ld s1,8(a1)
    ld s2,16(a1)
    ld s3,24(a1)
    ld s4,32(a1)
    ld s5,40(a1)
    ld s6,48(a1)
    ld s7,56(a1)
    ld s8,64(a1)
    ld s9,72(a1)
    ld s10,80(a1)
    ld s11,88(a1)
    #save tp
    sd tp,-8(a0)
    #save pc (we can use tp as temporary register)
    la tp,.L1
    sd tp,-16(a0)
    #restore tp
    ld tp,-8(a1)
    #switch sp
    sd sp,-24(a0)
    ld sp,-24(a1)
    #restore pc
    ld ra,-16(a1)
    ret
.L1:
    #now another thread has called `switch` to switch back to this thread,
    #s0~s11, tp, sp should be restored, so we can load the previously stored ra
    #and return normally
    ld ra,(sp)
    addi sp,sp,8
    ret
