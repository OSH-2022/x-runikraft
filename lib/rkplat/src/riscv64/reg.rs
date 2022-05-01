/// 所有的通用寄存器
#[repr(C)]
pub struct RegGen {
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    // arguments: non interrupts/non tracing syscalls only save upto here
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    // end arguments
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    //
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub pc: usize, //PC may refer to the real pc register or the uepc register
    //pub uepc: usize, //PC at the time the interruption is triggered
}


impl RegGen {
    // 联系硬件与抽象
    #[inline(always)]
    pub unsafe fn s0_read(mut &self) {
        arch::asm!("add s0, s0, x0",
        out("s0") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s1_read(mut &self) {
        arch::asm!("add s1, s1, x0",
        out("s1") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s2_read(mut &self) {
        arch::asm!("add s2, s2, x0",
        out("s2") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s3_read(mut &self) {
        arch::asm!("add s3, s3, x0",
        out("s3") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s4_read(mut &self) {
        arch::asm!("add s4, s4, x0",
        out("s4") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s5_read(mut &self) {
        arch::asm!("add s5, s5, x0",
        out("s5") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s6_read(mut &self) {
        arch::asm!("add s6, s6, x0",
        out("s6") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s7_read(mut &self) {
        arch::asm!("add s7, s7, x0",
        out("s7") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s8_read(mut &self) {
        arch::asm!("add s8, s8, x0",
        out("s8") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s9_read(mut &self) {
        arch::asm!("add s9, s9, x0",
        out("s9") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s10_read(mut &self) {
        arch::asm!("add s10, s10, x0",
        out("s10") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn s11_read(mut &self) {
        arch::asm!("add s11, s11, x0",
        out("s11") self.bits,
        );
    }

    #[inline(always)]
    pub unsafe fn a0_read(mut &self) {
        arch::asm!("add a0, a0, x0",
        out("a0") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a1_read(mut &self) {
        arch::asm!("add a1, a1, x0",
        out("a1") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a2_read(mut &self) {
        arch::asm!("add a2, a2, x0",
        out("a2") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a3_read(mut &self) {
        arch::asm!("add a3, a3, x0",
        out("a3") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a4_read(mut &self) {
        arch::asm!("add a4, a4, x0",
        out("a4") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a5_read(mut &self) {
        arch::asm!("add a5, a5, x0",
        out("a5") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a6_read(mut &self) {
        arch::asm!("add a6, a6, x0",
        out("a6") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn a7_read(mut &self) {
        arch::asm!("add a7, a7, x0",
        out("a7") self.bits,
        );
    }

    #[inline(always)]
    pub unsafe fn t0_read(mut &self) {
        arch::asm!("add t0, t0, x0",
        out("t0") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t1_read(mut &self) {
        arch::asm!("add t1, t1, x0",
        out("t1") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t2_read(mut &self) {
        arch::asm!("add t2, t2, x0",
        out("t2") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t3_read(mut &self) {
        arch::asm!("add t3, t3, x0",
        out("t3") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t4_read(mut &self) {
        arch::asm!("add t4, t4, x0",
        out("t4") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t5_read(mut &self) {
        arch::asm!("add t5, t5, x0",
        out("t5") self.bits,
        );
    }
    #[inline(always)]
    pub unsafe fn t6_read(mut &self) {
        arch::asm!("add t6, t6, x0",
        out("t6") self.bits,
        );
    }


    #[inline(always)]
    pub unsafe fn all_reg_read(mut &self) {
        self.s0_read();
        self.s1_read();
        self.s2_read();
        self.s3_read();
        self.s4_read();
        self.s5_read();
        self.s6_read();
        self.s7_read();
        self.s8_read();
        self.s9_read();
        self.s10_read();
        self.s11_read();
        self.a0_read();
        self.a1_read();
        self.a2_read();
        self.a3_read();
        self.a4_read();
        self.a5_read();
        self.a6_read();
        self.a7_read();
        self.t0_read();
        self.t1_read();
        self.t2_read();
        self.t3_read();
        self.t4_read();
        self.t5_read();
        self.t6_read();
    }
}

/// 浮点数寄存器
#[repr(C)]
pub struct RegFloat {
    pub fs0: f64,
    pub fs1: f64,
    pub fs2: f64,
    pub fs3: f64,
    pub fs4: f64,
    pub fs5: f64,
    pub fs6: f64,
    pub fs7: f64,
    pub fs8: f64,
    pub fs9: f64,
    pub fs10: f64,
    pub fs11: f64,
    // floating-point arguments
    pub fa0: f64,
    pub fa1: f64,
    pub fa2: f64,
    pub fa3: f64,
    pub fa4: f64,
    pub fa5: f64,
    pub fa6: f64,
    pub fa7: f64,
    // end arguments
    pub ft0: f64,
    pub ft1: f64,
    pub ft2: f64,
    pub ft3: f64,
    pub ft4: f64,
    pub ft5: f64,
    pub ft6: f64,
    pub ft7: f64,
    pub ft8: f64,
    pub ft9: f64,
    pub ft10: f64,
    pub ft11: f64,
    // floating point environment
    pub fflags: usize,
    pub frm: usize,
    pub fcsr: usize,
}
