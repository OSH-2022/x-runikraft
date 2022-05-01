//TODO 暂时用SBI实现底层操作
#![allow(unused)]

pub const SBI_SET_TIMER: usize = 0;
pub const SBI_CONSOLE_PUTCHAR: usize = 1;
pub const SBI_CONSOLE_GETCHAR: usize = 2;
pub const SBI_CLEAR_IPI: usize = 3;
pub const SBI_SEND_IPI: usize = 4;
pub const SBI_REMOTE_FENCE_I: usize = 5;
pub const SBI_REMOTE_SFENCE_VMA: usize = 6;
pub const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
pub const SBI_SHUTDOWN: usize = 8;
pub const SBI_SRST: usize = 0x53525354;

use core::arch;

#[inline(always)]
pub fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> Result<usize, usize>
{
    let mut error;
    let mut value;
    unsafe {
        arch::asm!("ecall",
        inlateout("a0")arg0 => error,
        inlateout("a1")arg1 => value,
        in("a2")arg2,
        in("a7")eid, in("a6")fid);
    }
    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}
