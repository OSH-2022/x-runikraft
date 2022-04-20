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
