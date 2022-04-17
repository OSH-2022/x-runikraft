pub type NanoSec = u64;

pub trait Time{
    fn get_irq()->u32;
    fn get_ticks()->NanoSec;
    fn monotonic_clock()->NanoSec;
    fn wall_clock()->NanoSec;
} 
