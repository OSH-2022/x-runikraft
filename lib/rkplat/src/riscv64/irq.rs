use rkalloc::RKalloc;

pub type IRQHandlerFunc = fn(*mut u8) -> i32;

/// 初始化平台的IRQ子系统
/// - `a`: 内部使用的分配器
/// - 返回值: 初始化的状态
pub fn init(a: &dyn RKalloc) -> Result<(), i32> {
    Err(-1)
}

/// 注册中断响应函数
/// - `irq`: 中断号
/// - `func`: 响应函数
/// - `arg`: 传递给响应函数的额外参数
pub fn register(irq: usize, func: IRQHandlerFunc, arg: *mut u8) -> Result<(), i32> {
    Err(-1)
}
