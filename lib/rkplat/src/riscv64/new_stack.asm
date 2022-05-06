.text
# 切换栈: 早期引导栈->引导栈，对应_libkvmplat_newstack
.globl __rkplat_newstack
# fn __rkplat_newstack(stack_top: *mut u8, tramp: extern fn(*mut u8)->!, arg: *mut u8)->!;
__rkplat_newstack:
    csrw sscratch, sp
    mv sp,a0
    mv a0,a2
    jr a1
