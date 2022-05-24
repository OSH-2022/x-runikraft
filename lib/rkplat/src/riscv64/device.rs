use rkalloc::RKalloc;
use crate::drivers::device_tree;

pub(crate) static mut DEVICE_PTR: &'static [u8] = &[];

/// 初始化设备
/// - `a`: 用来初始化的分配器
/// 
/// # 安全性
/// 
/// `__runikraft_entry_point` 必须初始化DEVICE_PTR，使它指向合法的内存区域
pub unsafe fn init(a: &dyn RKalloc) -> Result<(), i32> { 
    if let Err(error) = device_tree::parse(a,DEVICE_PTR){
        panic!("Fail to load device tree. {:?}",&error);
    }
    Ok(())//TODO
}

pub unsafe fn ioreg_write8 (addr: *mut  u8, val:  u8) {addr.write_volatile(val)}
pub unsafe fn ioreg_write16(addr: *mut u16, val: u16) {addr.write_volatile(val)}
pub unsafe fn ioreg_write32(addr: *mut u32, val: u32) {addr.write_volatile(val)}
pub unsafe fn ioreg_write64(addr: *mut u64, val: u64) {addr.write_volatile(val)}

pub unsafe fn ioreg_read8  (addr: *const  u8) ->  u8 {addr.read_volatile()}
pub unsafe fn ioreg_read16 (addr: *const u16) -> u16 {addr.read_volatile()}
pub unsafe fn ioreg_read32 (addr: *const u32) -> u32 {addr.read_volatile()}
pub unsafe fn ioreg_read64 (addr: *const u64) -> u64 {addr.read_volatile()}
