use core::time::Duration;

/// 中断标志，具体格式由平台决定
pub struct IRQFlag {
    flag: usize,
}

impl IRQFlag {
    // 将当前的 @flag 存储到 RISCV 中 
    //fn set_flag
}

/// 启用中断
pub fn enable_irq() {}

/// 禁用中断
pub fn disable_irq() {}

/// 保存中断标志并关闭中断
pub fn save_irqf() -> IRQFlag {
    IRQFlag { flag: 0 }
}

/// 加载中断标志
pub fn restore_irqf(flags: IRQFlag) {}

/// 检查中断是否被禁用
pub fn irqs_disabled() -> bool {
    true
}

/// 挂起当前的逻辑处理器
pub fn halt() {}

/// 挂起当前处理器一段时间，
/// 处理将在`deadline`到达(`get_ticks()>=deadline`)或中断/信号到来时重启
pub fn halt_to(until: Duration) {}

/// 挂起当前处理器，
/// 处理将在中断/信号到来时重启
pub fn halt_irq() {}

pub type ID = u32;

#[cfg(feature = "has_smp")]
mod smp {
    use super::*;

    pub type Entry = fn() -> !;
    pub type StackPointer = *mut u8;

    /// 返回当前的逻辑处理器的ID
    pub fn id() -> ID {
        0
    }

    /// 返回逻辑处理器的数量
    pub fn count() -> ID {
        1
    }

    /// 启动若干逻辑处理器。逻辑处理器将从给定的其实位置开始执行，
    /// 为执行的逻辑处理器会进入低功耗状态。
    ///
    /// 参数是若干要启动的逻辑处理机的slice：
    /// - `lcpuid.0`: 逻辑处理器ID
    /// - `lcpuid.1`: 栈指针
    /// - `lcpuid.2`: 入口函数
    pub fn start(lcpu_id_sp_entry: &[(ID, StackPointer, Entry)]) -> Result<(), i32> {
        Err(-1)
    }

    /// 让`lcpuid`中的逻辑处理器等待`timeout`
    ///
    /// 可以用`timeout`=0等待不确定的时间
    pub fn wait(lcpuid: &[ID], timeout: Duration) -> Result<(), i32> {
        Err(-1)
    }

    //TODO:
    // fn run()

    /// 唤醒被挂起或处在低功耗状态的逻辑处理器
    pub fn wakeup(lcpuid: &[ID]) -> Result<(), i32> {
        Err(-1)
    }
}

#[cfg(feature = "has_smp")]
pub use smp::*;

#[cfg(not(feature = "has_smp"))]
#[inline(always)]
fn id() -> ID { 0 }

#[cfg(not(feature = "has_smp"))]
#[inline(always)]
fn count() -> ID { 1 }
