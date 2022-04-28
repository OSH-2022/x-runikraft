use core::panic;
use super::plat::bootstrap;

#[panic_handler]
fn __panic_handler(info: &panic::PanicInfo)->!
{
    println!("Kernel panic!\n{:?}",info);
    bootstrap::crash();
}
