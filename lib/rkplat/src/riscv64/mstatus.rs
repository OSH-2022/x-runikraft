/*
    对 CSR mstatus 的抽象

    支持外部对 mstauts 的更改
    支持内部与 RISCV 的联系 

    Author: Jundong Wu
    Last edit: 2022.4.30
*/

#![feature(asm)]

use core::arch;

pub const BIT_UIE: usize = 0x1;
// User Interrupt Enable
pub const BIT_SIE: usize = 0x2;
// Supervisor Interrupt Enable
pub const BIT_MIE: usize = 0x8;
// Machine Interrupt Enable
pub const BIT_UPIE: usize = 0x10;
// User Previous Interrupt Enable
pub const BIT_SPIE: usize = 0x20;
// Supervisor Previous Interrupt Enable
pub const BIT_MPIE: usize = 0x80;
// Machine Previous Interrupt Enable
pub const BIT_SPP: usize = 0x100;
// Supervisor Previous Privilege Mode
pub const BIT_MPP: usize = 0x1800;
// Machine Previous Privilege Mode
pub const BIT_FS: usize = 0x6000;
// Floating-point extension state
pub const BIT_XS: usize = 0x18000;
// Additional extension state
pub const BIT_MPRV: usize = 0x20000;
pub const BIT_SUM: usize = 0x40000;
pub const BIT_MXR: usize = 0x80000;
pub const BIT_TVM: usize = 0x100000;
pub const BIT_TW: usize = 0x200000;
pub const BIT_TSR: usize = 0x400000;
pub const BIT_SD: usize = 0x80000000;

pub struct Mstauts {
    bits: usize,
}

impl Mstauts {
    // 获取数值
    #[inline(always)]
    pub fn get_uie(&self) -> usize {
        self.bits & BIT_UIE
    }
    #[inline(always)]
    pub fn get_sie(&self) -> usize {
        self.bits & BIT_SIE
    }
    #[inline(always)]
    pub fn get_mie(&self) -> usize {
        self.bits & BIT_MIE
    }
    #[inline(always)]
    pub fn get_upie(&self) -> usize {
        self.bits & BIT_UPIE
    }
    #[inline(always)]
    pub fn get_spie(&self) -> usize {
        self.bits & BIT_SPIE
    }
    #[inline(always)]
    pub fn get_mpie(&self) -> usize {
        self.bits & BIT_MPIE
    }
    #[inline(always)]
    pub fn get_spp(&self) -> usize {
        self.bits & BIT_SPP
    }
    #[inline(always)]
    pub fn get_mpp(&self) -> usize {
        self.bits & BIT_MPP
    }
    #[inline(always)]
    pub fn get_fs(&self) -> usize {
        self.bits & BIT_FS
    }
    #[inline(always)]
    pub fn get_xs(&self) -> usize {
        self.bits & BIT_XS
    }
    #[inline(always)]
    pub fn get_mprv(&self) -> usize {
        self.bits & BIT_MPRV
    }
    #[inline(always)]
    pub fn get_sum(&self) -> usize {
        self.bits & BIT_SUM
    }
    #[inline(always)]
    pub fn get_mxr(&self) -> usize {
        self.bits & BIT_MXR
    }
    pub fn get_tvm(&self) -> usize {
        self.bits & BIT_TVM
    }
    #[inline(always)]
    pub fn get_tw(&self) -> usize {
        self.bits & BIT_TW
    }
    #[inline(always)]
    pub fn get_tsr(&self) -> usize {
        self.bits & BIT_TSR
    }
    #[inline(always)]
    pub fn get_sd(&self) -> usize {
        self.bits & BIT_SD
    }

    // 设置数值
    #[inline(always)]
    pub fn set_uie(&mut self) {
        self = self.bits | BIT_UIE
    }
    #[inline(always)]
    pub fn set_sie(&mut self) {
        self = self.bits | BIT_SIE
    }
    #[inline(always)]
    pub fn set_mie(&mut self) {
        self = self.bits | BIT_MIE
    }
    #[inline(always)]
    pub fn set_upie(&mut self) {
        self = self.bits | BIT_UPIE
    }
    #[inline(always)]
    pub fn set_spie(&mut self) {
        self = self.bits | BIT_SPIE
    }
    #[inline(always)]
    pub fn set_mpie(&mut self) {
        self = self.bits | BIT_MPIE
    }
    #[inline(always)]
    pub fn set_spp(&mut self) {
        self = self.bits | BIT_SPP
    }
    #[inline(always)]
    pub fn set_mpp(&mut self) {
        self = self.bits | BIT_MPP
    }
    #[inline(always)]
    pub fn set_fs(&mut self) {
        self = self.bits | BIT_FS
    }
    #[inline(always)]
    pub fn set_xs(&mut self) {
        self = self.bits | BIT_XS
    }
    #[inline(always)]
    pub fn set_mprv(&mut self) {
        self = self.bits | BIT_MPRV
    }
    #[inline(always)]
    pub fn set_sum(&mut self) {
        self = self.bits | BIT_SUM
    }
    #[inline(always)]
    pub fn set_mxr(&mut self) {
        self = self.bits | BIT_MXR
    }
    pub fn set_tvm(&mut self) {
        self = self.bits | BIT_TVM
    }
    #[inline(always)]
    pub fn set_tw(&mut self) {
        self = self.bits | BIT_TW
    }
    #[inline(always)]
    pub fn set_tsr(&mut self) {
        self = self.bits | BIT_TSR
    }
    #[inline(always)]
    pub fn set_sd(&mut self) {
        self = self.bits | BIT_SD
    }


    // 硬件操作
    // mstatus 对应 csr 编号为 0x300

    // 内联汇编参考 https://xiaopengli89.github.io/posts/rust-asm-macro/

    // 根据 @bits 按位清空 mstatus 的值
    #[inline(always)]
    pub unsafe fn mstatus_clear(&self) {
        let number = !self.bits;
        arch::asm!("csrrc zero, 0x300, a0",
        in("a0") number
        );
    }

    // 将 mstauts 的值读取到 @bit 中
    #[inline(always)]
    pub unsafe fn mstauts_read(&mut self) {
        let number: usize = 0;
        arch::asm!("csrrc a0, 0x300, a1",
        out("a0") self.bits,
        in("a1") number
        );
    }
}
