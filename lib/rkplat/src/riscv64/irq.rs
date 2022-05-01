use rkalloc::RKalloc;
use super::constants::*;
//use runikraft::list::SList;

union AllocWrapper {
    a: *const dyn RKalloc,
    i: usize
}

static mut ALLOCATOR: AllocWrapper = AllocWrapper{i:0};
/// 中断响应函数，返回false将中断转交给下一个函数处理，返回true表示中断处理完毕
pub type IRQHandlerFunc = fn(*mut u8)->bool;

struct IRQHandler {
    func: IRQHandlerFunc,
    arg: *mut u8,
}

// static mut irq_handlers = [SList<IRQHandler>::new(); MAX_IRQ];


fn allocator() -> &'static dyn RKalloc {
    unsafe {
        assert_ne!(ALLOCATOR.i,0);
        &*ALLOCATOR.a
    }
}

//pub type IRQHandlerFunc = fn(*mut u8) -> i32;

/// 初始化平台的IRQ子系统
/// - `a`: 内部使用的分配器
/// - 返回值: 初始化的状态
/// 
/// # 安全性
/// 
/// 必须保证分配器`a`在系统关机前仍有效，`a`可以拥有静态生命周期，也可以位于boot stack上
pub unsafe fn init(a: *const dyn RKalloc) -> Result<(), i32> {
    assert_eq!(ALLOCATOR.i,0);
    ALLOCATOR.a = a;
    Ok(())
}

/// 注册中断响应函数，可以为一个中断号注册多个响应函数，它们将按注册的逆序被调用
/// - `irq`: 中断号
/// - `func`: 响应函数
/// - `arg`: 传递给响应函数的额外参数
/// 
/// # 安全性
/// 
/// - `arg`指向的数据必须在关机前仍然有效，它可以是静态数据，也可以是位于boot stack上的数据，
/// 还可以是由生命足够长的分配器分配的数据
/// - `func`需要将`arg`转换成合适的类型
// TODO: 尝试用泛型处理（但看起来很困难）
pub unsafe fn register(irq: usize, func: IRQHandlerFunc, arg: *mut u8) -> Result<(), i32> 
{
    let handler = IRQHandler{func,arg};

    Err(-1)
}
