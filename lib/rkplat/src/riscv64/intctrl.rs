use core::arch;

/// 确认已处理IRQ
pub(crate) fn ack_irq(_irq: usize) {
    //nop
}

pub(crate) fn mask_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        arch::asm!("csrs sip, {irq}",
        irq=in(reg)irq);
    }
}

/// 清除正在等待处理的IRQ
pub(crate) fn clear_irq(irq: usize) {
    let irq = 1usize<<irq;
    unsafe {
        arch::asm!("csrc sip, {irq}",
        irq=in(reg)irq);
    }
}
