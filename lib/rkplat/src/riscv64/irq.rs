use core::ptr::NonNull;

use rkalloc::RKalloc;
use rkalloc::alloc_type;
use rklist::{Slist,SlistNode};
use super::constants::*;
use super::{lcpu,intctrl};


static mut ALLOCATOR: Option<&dyn RKalloc> = None;
/// 中断响应函数，返回false将中断转交给下一个函数处理，返回true表示中断处理完毕
pub type IRQHandlerFunc = fn(*mut u8)->bool;

struct IRQHandler {
    func: IRQHandlerFunc,
    arg: *mut u8,
}

/// 直接[None;64]会报 E0277
static mut IRQ_HANDLERS:[Option<Slist<IRQHandler>>;MAX_IRQ] = include!("64None.txt");

fn allocator() -> &'static dyn RKalloc {
    unsafe {
        ALLOCATOR.unwrap()
    }
}

/// 初始化平台的IRQ子系统
/// - `a`: 内部使用的分配器
/// - 返回值: 初始化的状态
/// 
/// # 安全性
/// 
/// 必须保证分配器`a`在系统关机前仍有效，`a`可以拥有静态生命周期，也可以位于boot stack上
pub unsafe fn init(a: &dyn RKalloc) -> Result<(), i32> {
    assert!(ALLOCATOR.is_none());
    union Helper<'a> {
        reference: &'a dyn RKalloc,
        pointer: *const dyn RKalloc,
    }
    ALLOCATOR = Some(Helper{pointer: Helper{reference: a}.pointer}.reference);
    for i in &mut IRQ_HANDLERS{
        *i = Some(Slist::new());
    }
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
pub unsafe fn register(irq: usize, func: IRQHandlerFunc, arg: *mut u8) -> Result<(), i32> 
{   
    let handler = IRQHandler{func,arg};
    let flags =lcpu::save_irqf(); 
    //interruption
    IRQ_HANDLERS[irq].as_mut().unwrap().push_front(NonNull::new(alloc_type(allocator(),SlistNode::new(handler))).unwrap());
    lcpu::restore_irqf(flags);
    if irq&1<<63 !=0 { intctrl::clear_irq(irq&0x3F); }
    Ok(())
}

//TODO: 
#[no_mangle]
unsafe extern "C" fn __rkplat_irq_handle(irq: usize) {
    for i in IRQ_HANDLERS[irq].as_ref().unwrap().iter() {
        if (i.as_ref().element.func)(i.as_ref().element.arg) {
            intctrl::ack_irq(irq);
            return;
        }
    }
    println!("Unhandled irq={}",irq);
    // panic!();
}
