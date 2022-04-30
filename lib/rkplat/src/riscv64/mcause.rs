/*
    对 CSR mcause 的抽象

    Author: Jundong Wu
    Last edit: 2022.4.30
*/

#![feature(asm)]

use core::arch;

// RISCV interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    User_Software_Interrupt = 0,
    Supervisor_Software_Interrupt = 1,
    Machine_Software_Interrupt = 3,
    User_Timer_Interrupt = 4,
    Supervisor_Timer_Interrupt = 5,
    Machine_Timer_Interrupt = 7,
    User_External_Interrupt = 8,
    Supervisor_External_Interrupt = 9,
    Machine_External_InterruptL = 11,
}

// RISCV exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    Instruction_Address_Misaligned = 0,
    Instruction_Access_Fault = 1,
    Illegal_Instruction = 2,
    Breakpoint = 3,
    Load_Address_Misaligned = 4,
    Load_Access_Falut = 5,
    Store_Address_Misaligned = 6,
    Store_Access_Falut = 7,
    Environment_Call_From_Umode = 8,
    Environment_Call_From_Smode = 9,
    Environment_Call_From_Mmode = 11,
    Environment_Page_Fault = 12,
    Load_Page_Fault = 13,
    Store_Page_Falut = 15,
}

pub struct Mcause {
    bits: usize,
}

impl Mcause {
    #[inline(always)]
    pub fn get_bits(&self) -> usize {
        self.bits
    } 

    // mcause 对应 csr 编号为 0x342

    // 将 mcause 的值读取到 @bit 中
    
    #[inline(always)]
    pub unsafe fn mcause_read(&mut self) {
        let number: usize = 0;
        arch::asm!("csrrc a0, 0x342, a1",
                out("a0") self.bits,
                in("a1") number
                );
    }

}
 