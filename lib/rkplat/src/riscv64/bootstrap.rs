//TODO: arg

use super::sbi::*;
use super::*;
use core::arch;

arch::global_asm!(include_str!("entry.asm"));

// #[linkage = "weak"]
// #[no_mangle]
// fn main() {
//     println!("Error: weak main was called!");
//     panic!("no main");
// }

extern "C" {
    fn main();
}

/// 系统的入口，由引导程序调用
///
#[no_mangle]
pub fn __runikraft_entry_point() -> ! {
    time::init();
    unsafe { main(); }
    halt();
}

/// 退出
pub fn halt() -> ! {
    sbi_call(SBI_SRST, 0, 0, 0, 0).unwrap();
    panic!("Should halt.");
}

/// 重启
pub fn restart() -> ! {
    sbi_call(SBI_SRST, 0, 1, 0, 0).unwrap();
    panic!("Should restart.");
}

/// 崩溃
pub fn crash() -> ! {
    print!("System crashed!\n");
    sbi_call(SBI_SRST, 0, 0, 1, 0).unwrap();
    loop {}//不能用panic，因为panic会调用crash
}

/// 挂起
//TODO
pub fn suspend() -> ! {
    loop {}
}
