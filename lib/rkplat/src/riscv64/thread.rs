use super::reg;
use core::{arch, ptr::addr_of_mut};

extern "C" {
    fn __thread_starter();
    fn __thread_context_start(sp: usize, pc: usize)->!;
    fn __thread_context_switch(prevctx: *mut Context, nextctx: *mut Context);
    fn __save_extregs(reg: *mut reg::RegGen);
    fn __restore_extregs(reg: *mut reg::RegGen);
}

#[repr(C)]
#[derive(Default)]
pub struct Context {
    sp: usize, //栈指针
    pc: usize, //程序计数器
    tp: usize, //线程本地数据
    regs: reg::RegGen,  //保存通用寄存器的区域
    #[cfg(feature="save_fp")]
    fregs: reg::RegFloat, //保存浮点寄存器的区域（可以为空）
}

impl Context {
    //操作过于底层，无法使用new
    pub unsafe fn init(ctx: *mut Context, sp: usize, tp: usize) {
        (*ctx).sp = sp;
        (*ctx).tp = tp;
        (*ctx).pc = __thread_starter as usize;
        __save_extregs(addr_of_mut!((*ctx).regs));
    }

    pub unsafe fn start(ctx: *mut Context) -> ! {
        set_tp((*ctx).tp);
        __thread_context_start((*ctx).sp,(*ctx).pc);
        //panic!("Thread did not start.");
    }

    pub unsafe fn switch(prevctx: *mut Context, nextctx: *mut Context) {
        __save_extregs(addr_of_mut!((*prevctx).regs));
        __restore_extregs(addr_of_mut!((*prevctx).regs));
        set_tp((*nextctx).tp);
        __thread_context_switch(prevctx,nextctx);
    }
}


unsafe fn set_tp(tp: usize) {
    arch::asm!("mv tp,{tp}",
        tp=in(reg)tp);
}
