//控制台输入输出

use super::sbi::*;

//TODO: 更详细的错误信息
fn putchar(ch: usize) -> bool {
    if let Err(_) = sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0) {
        return false;
    }
    true
}

fn getchar() -> Result<usize, usize> {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
}


/// 向内核控制台输出字符串
/// 注意字符串不必是合法的UTF-8，也不会因null终止
/// - `buf`: 字符串缓冲区
/// - 返回值: 输出的字符数
pub fn coutk(buf: &[u8]) -> Result<usize, ()> {
    for i in buf {
        if !putchar(*i as usize) {
            return Err(());
        }
    }
    Ok(buf.len())
}

/// 向调试控制台输出字符串
pub fn coutd(buf: &[u8]) -> Result<usize, ()> {
    coutk(buf)
}

/// 从控制台读入字符
/// - `buf`: 目标缓冲区
/// - 返回值 读入的字符数
pub fn cink(buf: &mut [u8]) -> Result<usize, ()> {
    let mut cnt: usize = 0;
    for i in buf {
        match getchar() {
            Ok(ch) => {
                *i = ch as u8;
                cnt = cnt + 1;
            }
            Err(_) => { return Err(()); }
        }
    }
    Ok(cnt)
}

///////////////////
//Rust风格的输出

use core::fmt::{self, Write};

struct RustStyleOutput;

pub fn __print(args: fmt::Arguments) {
    RustStyleOutput.write_fmt(args).unwrap();
}

impl Write for RustStyleOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        coutk(s.as_bytes()).unwrap();
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::__print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
