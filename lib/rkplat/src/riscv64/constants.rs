/// RISC-V支持2^(MXLEN-1)（2^31/2^63/2^127）种中断/异常，
/// 但目前大于等于64的exception code是被保留的，
/// 而一个exception code可能对应异常或中断，所以MAX_IRQ=128
/// 
/// IRQ[5:0]是exception code，IRQ[6]是Interrupt
pub const MAX_IRQ: usize = 128;
