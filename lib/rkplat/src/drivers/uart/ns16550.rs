use core::ptr::null_mut;
use core::{str, slice};

static mut UART_ADDR: *mut u8 = null_mut();
static mut INT: usize = 0;
static mut NAME: [u8;32] = [0;32];
static mut NAME_SIZE: usize = 0;

fn uart_addr() -> &'static mut *mut u8 {
    unsafe {&mut UART_ADDR}
}

fn int() -> &'static mut usize {
    unsafe {&mut INT}
}

fn name() -> &'static str {
    unsafe {str::from_utf8_unchecked(slice::from_raw_parts(NAME.as_ptr(), NAME_SIZE))}
}

pub fn init(name: &str, addr: *mut u8, irq: usize) {
    unsafe {
        assert!(name.len()<=32);
        for i in 0..name.len() {
            NAME[i] = name.as_bytes()[i];
        }
        NAME_SIZE = name.len();
    }
    *uart_addr() = addr;
    *int() = irq;
    println!("Init ns16550 device, name={},addr={:?},irq={}.",super::ns16550::name(),uart_addr(),int());
    
    unsafe {
        let ptr = uart_addr();
        // 偏移 3 指出每次传输的位数为 8 位，即一个字节
        ptr.add(3).write_volatile(8);
        // 使能 FIFO缓冲队列
        ptr.add(2).write_volatile(1);
        // 使能中断
        ptr.add(1).write_volatile(1);

        // 设置输入产生的中断频率
        let divisor : u16 = 592;
        let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
        let divisor_most:  u8 = (divisor >> 8).try_into().unwrap();
        let lcr = ptr.add(3).read_volatile();
        ptr.add(3).write_volatile(lcr | 1 << 7);
        ptr.add(0).write_volatile(divisor_least);
        ptr.add(1).write_volatile(divisor_most);
        ptr.add(3).write_volatile(lcr);
    }

    for char in "Hello, ns16550\n".as_bytes() {
        putchar(*char);
    }
}

pub fn putchar(char: u8) {
    unsafe {
        uart_addr().write_volatile(char);
    }
}
