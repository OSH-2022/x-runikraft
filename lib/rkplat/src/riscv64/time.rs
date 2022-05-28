use super::{lcpu,sbi};

pub type Duration = core::time::Duration;

/// 1秒
pub const SEC: Duration = Duration::new(1, 0);
/// 1纳秒
pub const NSEC: Duration = Duration::new(0, 1);

use core::arch;

//1tick的长度
pub const TICK_NANOSEC: u64 = 100;

//初始化时的time寄存器的值
static mut INIT_TIME: u64 = 0;

//time寄存器的起点
static mut TIME_START: Duration = Duration::new(0, 0);

/// 初始化时钟和时钟中断
//TODO: 未完成
pub fn init() {
    // OpenSBI启动时的输出 Platform Timer Device     : aclint-mtimer @ 10000000Hz
    // unsafe { TICK_NANOSEC = 100 };
}

//获取时钟中断号
pub const fn get_irq() -> usize {
    0x8000_0000_0000_0005
}

fn get_time_counter() -> u64 {
    let time: u64;
    unsafe {
        arch::asm!("rdtime t0",
        out("t0")time);
    }
    time
}

/// CPU内部的计时器的值
pub fn get_ticks() -> Duration {
    Duration::from_nanos(TICK_NANOSEC * get_time_counter())
}

/// 获取自时钟初始化以来的时间
pub fn monotonic_clock() -> Duration {
    Duration::from_nanos(unsafe { TICK_NANOSEC * (get_time_counter() - INIT_TIME) })
}

/// 获取UNIX时间
pub fn wall_clock() -> Duration {
    get_ticks() + unsafe { TIME_START }
}

fn block(until: Duration) {
    assert!(lcpu::irqs_disabled());
    let time_now = monotonic_clock();
    if until <= time_now {return;}
    let duration = (until.as_nanos() - time_now.as_nanos()) as u64;
    //Set Timer
    sbi::sbi_call(0x54494D45, 0, (duration/TICK_NANOSEC) as usize, 0, 0).unwrap();
    lcpu::halt_irq();
}

/// 暂停当前处理器核，直到`until`时刻 
pub fn block_until(until: Duration) {
    loop {
        block(until);
        if monotonic_clock() >= until {break;}
    }
}
