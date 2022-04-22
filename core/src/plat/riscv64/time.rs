pub type Duration = core::time::Duration;
pub const SEC: Duration = Duration::new(1,0); /// 1秒
pub const NSEC: Duration = Duration::new(0,1);/// 1纳秒

use core::arch;

//1tick的长度
static mut TICK_NANOSEC: u64 = 0;

//初始化时的time寄存器的值
static mut INIT_TIME: u64 = 0;

//time寄存器的起点
static mut TIME_START: Duration = Duration::new(0,0);

/// 初始化时钟和时钟中断
//TODO: 未完成
pub fn init() {
    // OpenSBI启动时的输出 Platform Timer Device     : aclint-mtimer @ 10000000Hz
    unsafe{TICK_NANOSEC = 100};
}

//TODO: 获取时钟中断号
// pub fn get_irq()->u32;

fn get_time_counter()->u64 {
    let time: u64;
    unsafe {
        arch::asm!("rdtime t0",
            out("t0")time);
    }
    time
}

/// CPU内部的计时器的值
pub fn get_ticks()->Duration {
    Duration::from_nanos(unsafe{TICK_NANOSEC}*get_time_counter())
}

/// 获取自时钟初始化以来的时间
pub fn monotonic_clock()->Duration {
    Duration::from_nanos(unsafe{TICK_NANOSEC*(get_time_counter()-INIT_TIME)})
}

/// 获取UNIX时间
pub fn wall_clock()->Duration {
    get_ticks() + unsafe{TIME_START}
}
