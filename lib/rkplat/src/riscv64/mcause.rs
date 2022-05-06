/*
    对 CSR mcause 的抽象

    Author: Jundong Wu
    Last edit: 2022.4.30
*/
use core::arch;

// RISCV interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoftwareInterrupt = 0,
    SupervisorSoftwareInterrupt = 1,
    MachineSoftwareInterrupt = 3,
    UserTimerInterrupt = 4,
    SupervisorTimerInterrupt = 5,
    MachineTimerInterrupt = 7,
    UserExternalInterrupt = 8,
    SupervisorExternalInterrupt = 9,
    MachineExternalInterruptL = 11,
}

// RISCV exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFalut = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFalut = 7,
    EnvironmentCallFromUmode = 8,
    EnvironmentCallFromSmode = 9,
    EnvironmentCallFromMmode = 11,
    EnvironmentPageFault = 12,
    LoadPageFault = 13,
    StorePageFalut = 15,
}

pub fn read_mcause() -> usize{
    let mcause: usize;
    unsafe{
        arch::asm!("csrr a0, mcause",
        out("a0")mcause);
    }
    mcause
}
