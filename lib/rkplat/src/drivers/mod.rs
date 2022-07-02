#[cfg(feature = "driver_uart")]
pub mod uart;
#[cfg(feature = "driver_virtio")]
pub mod virtio;
#[cfg(feature = "driver_rtc")]
pub mod rtc;
pub mod device_tree;

pub type DriverIntHandler = fn();

pub trait Device: Sync {
    /// 设备名称
    fn name<'a>(&'a self) -> &'a str;
    /// 中断号，如果设备不支持中断，则返回None
    fn irq(&self) -> Option<usize> {
        None
    }
    /// 中断处理函数，如果设备不支持中断，则返回None
    fn irq_handler(&self) -> Option<DriverIntHandler> {
        None
    }
}
