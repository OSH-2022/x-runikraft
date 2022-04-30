//平台
#![no_std]

use core::panic;

#[cfg(target_arch = "riscv64")]
#[macro_use]
mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

#[panic_handler]
fn __panic_handler(info: &panic::PanicInfo)->!
{
    println!("Kernel panic!\n{:?}",info);
    bootstrap::crash();
}

