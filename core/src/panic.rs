use core::panic;
use super::platform::bootstrap;

#[panic_handler]
fn __panic_handler(_info: &panic::PanicInfo)->!
{
    println!("Panic!");
    bootstrap::crash();
}
