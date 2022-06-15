// SPDX-License-Identifier: BSD-3-Clause
// reg.rs
// Authors: 张子辰 <zichen350@gmail.com>
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

/// 发生异常时需要保存的所有的通用寄存器
/// size=152
#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct RegGenExcept {
    pub t0: usize,      //0
    pub t1: usize,      //8
    pub t2: usize,      //16
    pub t3: usize,      //24
    pub t4: usize,      //32
    pub t5: usize,      //40
    pub t6: usize,      //48
    pub a0: usize,      //56
    pub a1: usize,      //64
    pub a2: usize,      //72
    pub a3: usize,      //80
    pub a4: usize,      //88
    pub a5: usize,      //96
    pub a6: usize,      //104
    pub a7: usize,      //112
    pub ra: usize,      //120
    pub pc: usize,      //128
    pub sstatus: usize, //136
    pub sp: usize,      //144
}

/// 发生中断时需要保存的所有的通用寄存器
/// size=144
#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct RegGenInt {
    pub t0: usize,      //0
    pub t1: usize,      //8
    pub t2: usize,      //16
    pub t3: usize,      //24
    pub t4: usize,      //32
    pub t5: usize,      //40
    pub t6: usize,      //48
    pub a0: usize,      //56
    pub a1: usize,      //64
    pub a2: usize,      //72
    pub a3: usize,      //80
    pub a4: usize,      //88
    pub a5: usize,      //96
    pub a6: usize,      //104
    pub a7: usize,      //112
    pub ra: usize,      //120
    pub pc: usize,      //128
    pub sstatus: usize, //136
}

/// ctx_switch时保存需要保存的所有的通用寄存器
/// size=96
#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct RegGenSw {
    pub s0: usize,  //0
    pub s1: usize,  //8
    pub s2: usize,  //16
    pub s3: usize,  //24
    pub s4: usize,  //32
    pub s5: usize,  //40
    pub s6: usize,  //48
    pub s7: usize,  //56
    pub s8: usize,  //64
    pub s9: usize,  //72
    pub s10: usize, //80
    pub s11: usize, //88
}

/// 浮点数寄存器
#[cfg(feature="save_fp")]
#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct RegFloat {
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
    // floating point environment
    pub fflags: usize,
    pub frm: usize,
    pub fcsr: usize,
}
