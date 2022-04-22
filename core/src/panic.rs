use core::panic;
use super::plat::bootstrap;

#[panic_handler]
fn __panic_handler(_info: &panic::PanicInfo)->!
{
    println!("Panic!");
    bootstrap::crash();
}
