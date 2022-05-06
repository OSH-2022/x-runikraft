use core::arch;

/// 确认已处理IRQ
pub fn ack_irq(_irq: usize) {
    //nop
}

pub fn mask_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        arch::asm!("csrs sip, {irq}",
        irq=in(reg)irq);
    }
}

/// 清除正在等待处理的IRQ
pub fn clear_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        arch::asm!("csrc sip, {irq}",
        irq=in(reg)irq);
    }
}
